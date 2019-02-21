use crate::camera::{world_to_screen, Camera};
use crate::resources::ScreenSize;
use cgmath::Point2;
use ggez::graphics::{self, spritebatch, Color, DrawParam, Image, MeshBuilder};
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
  #[allow(dead_code)]
  SpriteBatch {
    image_builder: ImageBuilder,
    sprites: Vec<DrawParam>,
  },
  #[allow(dead_code)]
  Mesh(MeshBuilder),
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
    };

    Ok(())
  }
}

pub enum HorizontalDirection {
  Left,
  #[allow(dead_code)]
  Middle,
  #[allow(dead_code)]
  Right,
}

pub enum VerticalDirection {
  #[allow(dead_code)]
  Top,
  #[allow(dead_code)]
  Middle,
  Bottom,
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

#[derive(Default)]
pub struct UiElement {
  pub stick_horizontal: Option<HorizontalDirection>,
  pub stick_vertical: Option<VerticalDirection>,
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

        if let Some(position) = positions.get(entity) {
          // Reassigning "draw_param" because the dest property is using
          // nalgebra so we can't change it directly
          draw_param = draw_param.dest(position.0);
        }

        if let Some(ui_element) = &ui_elements.get(entity) {
          if let Some(direction) = &ui_element.stick_horizontal {
            draw_param.dest.x = match direction {
              HorizontalDirection::Left => 0.0,
              HorizontalDirection::Middle => screen_size.x / 2.0,
              HorizontalDirection::Right => screen_size.x,
            };
          }

          if let Some(direction) = &ui_element.stick_vertical {
            draw_param.dest.y = match direction {
              VerticalDirection::Top => 0.0,
              VerticalDirection::Middle => screen_size.y / 2.0,
              VerticalDirection::Bottom => screen_size.y,
            };
          }
        } else {
          draw_param = draw_param
            .dest(world_to_screen(
              // TODO: convert from nalgebra to cgmath more efficiently
              Point2::new(draw_param.dest.coords.x, draw_param.dest.coords.x),
              camera_position,
              camera.zoom,
              screen_size,
            ))
            .scale([camera.zoom, camera.zoom]);
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
