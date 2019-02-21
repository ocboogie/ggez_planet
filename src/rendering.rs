use crate::camera::{world_to_screen, Camera};
use crate::resources::ScreenSize;
use ggez::graphics::{self, spritebatch, Color, DrawParam, Image, MeshBuilder};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};
use specs::prelude::*;

#[derive(Clone)]
pub enum ImageBuilder {
  Solid { size: u16, color: Color },
}

impl ImageBuilder {
  pub fn build(self, ctx: &mut Context) -> GameResult<Image> {
    use ImageBuilder::*;

    match self {
      Solid { size, color } => Image::solid(ctx, size, color),
    }
  }
}

// TODO: Add text and canvas variants
#[derive(Clone)]
pub enum RenderInstruction {
  #[allow(dead_code)]
  Image(ImageBuilder),
  SpriteBatch {
    image_builder: ImageBuilder,
    sprites: Vec<DrawParam>,
  },
  #[allow(dead_code)]
  Mesh(MeshBuilder),
  #[allow(dead_code)]
  Multi(Vec<RenderInstruction>),
}

impl RenderInstruction {
  pub fn render(self, ctx: &mut Context, draw_param: DrawParam) -> GameResult {
    use RenderInstruction::*;

    match self {
      Image(image_builder) => {
        let image = &image_builder.build(ctx)?;
        graphics::draw(ctx, image, draw_param)?;
      }
      SpriteBatch {
        image_builder,
        sprites,
      } => {
        let mut spritebatch = spritebatch::SpriteBatch::new(image_builder.build(ctx)?);

        for sprite in sprites {
          spritebatch.add(sprite);
        }

        graphics::draw(ctx, &spritebatch, draw_param)?;
      }
      Mesh(mesh_builder) => {
        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, draw_param)?;
      }
      Multi(instructions) => {
        for instruction in instructions {
          instruction.render(ctx, draw_param)?;
        }
      }
    };

    Ok(())
  }
}

pub struct Renderable {
  pub instruction: RenderInstruction,
  pub draw_param: Option<DrawParam>,
  pub layer: Option<i32>,
}

impl Renderable {
  pub fn render(self, ctx: &mut Context) -> GameResult {
    self
      .instruction
      .render(ctx, self.draw_param.unwrap_or_default())
  }
}

impl Component for Renderable {
  type Storage = VecStorage<Self>;
}

pub struct Position(pub Point2<f32>);

impl Component for Position {
  type Storage = VecStorage<Self>;
}

#[derive(Copy, Clone)]
pub enum Anchor {
  TopLeft,
  TopCenter,
  TopRight,

  CenterLeft,
  Center,
  CenterRight,

  BottomLeft,
  BottomCenter,
  BottomRight,
}

impl Anchor {
  pub fn get_postion(self, bounds: Vector2<f32>) -> Point2<f32> {
    use Anchor::*;

    match self {
      TopLeft => Point2::new(0.0, 0.0),
      TopCenter => Point2::new(bounds.x / 2.0, 0.0),
      TopRight => Point2::new(bounds.x, 0.0),

      CenterLeft => Point2::new(0.0, bounds.y / 2.0),
      Center => Point2::new(bounds.x / 2.0, bounds.y / 2.0),
      CenterRight => Point2::new(bounds.x, bounds.y / 2.0),

      BottomLeft => Point2::new(0.0, bounds.y),
      BottomCenter => Point2::new(bounds.x / 2.0, bounds.y),
      BottomRight => Point2::new(bounds.x, bounds.y),
    }
  }
}

impl Default for Anchor {
  fn default() -> Self {
    Anchor::Center
  }
}

#[derive(Default)]
pub struct UiElement {
  pub anchor: Option<Anchor>,
}

impl Component for UiElement {
  type Storage = DenseVecStorage<Self>;
}

pub struct RenderingSystem<'c> {
  ctx: &'c mut Context,
}

impl<'c> RenderingSystem<'c> {
  pub fn new(ctx: &'c mut Context) -> RenderingSystem<'c> {
    RenderingSystem { ctx }
  }
}

impl<'a, 'c> System<'a> for RenderingSystem<'c> {
  #[allow(clippy::type_complexity)]
  type SystemData = (
    Entities<'a>,
    Read<'a, ScreenSize>,
    WriteStorage<'a, Renderable>,
    ReadStorage<'a, UiElement>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Camera>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (entities, screen_size, mut renderables, ui_elements, positions, cameras) = data;

    let screen_size = screen_size.0;

    for (camera, camera_position) in (&cameras, &positions).join() {
      let camera_position = camera_position.0;

      let mut renderable_entities: Vec<(Entity, Renderable)> =
        (&*entities, renderables.drain()).join().collect();

      renderable_entities.sort_by_key(|(_, renderable)| renderable.layer);

      for (entity, mut renderable) in renderable_entities {
        let mut draw_param = renderable.draw_param.unwrap_or_else(DrawParam::default);

        if let Some(ui_element) = &ui_elements.get(entity) {
          draw_param.dest = ui_element
            .anchor
            .unwrap_or_default()
            .get_postion(screen_size);
        } else {
          draw_param.dest =
            world_to_screen(draw_param.dest, camera_position, camera.zoom, screen_size);

          draw_param.scale = Vector2::repeat(camera.zoom);
        }

        if let Some(position) = positions.get(entity) {
          draw_param.dest = position.0;
        }

        renderable.draw_param = Some(draw_param);

        renderable.render(self.ctx).unwrap();
      }
    }
  }
}

pub fn setup<'a, 'b>(
  _ctx: &mut Context,
  world: &mut World,
  _dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
  world.register::<Position>();
  world.register::<Renderable>();
  world.register::<UiElement>();
}
