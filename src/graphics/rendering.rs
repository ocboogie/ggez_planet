use super::{
  camera::{world_to_screen, Camera},
  ui::UiElement,
  Fonts, Layer, Position, ScreenSize,
};
use ggez::{
  graphics::{
    self, spritebatch, Color, DrawParam, Drawable, Font, Image, MeshBuilder, Rect, Scale,
  },
  nalgebra::Vector2,
  Context, GameResult,
};
use specs::prelude::*;
use std::collections::HashMap;

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
  Text {
    text: String,
    font: &'static str,
    scale: Scale,
  },
}

impl RenderInstruction {
  pub fn construct(
    self,
    ctx: &mut Context,
    fonts: &HashMap<&'static str, Font>,
  ) -> GameResult<Box<Drawable>> {
    use RenderInstruction::*;

    Ok(match self {
      Image(image_builder) => Box::new(image_builder.build(ctx)?),
      SpriteBatch {
        image_builder,
        sprites,
      } => {
        let mut spritebatch = spritebatch::SpriteBatch::new(image_builder.build(ctx)?);

        for sprite in sprites {
          spritebatch.add(sprite);
        }

        Box::new(spritebatch)
      }
      Mesh(mesh_builder) => Box::new(mesh_builder.build(ctx)?),
      Text { text, font, scale } => {
        let mut drawable_text = graphics::Text::new(text);

        drawable_text.set_font(fonts.get(font).cloned().unwrap_or_default(), scale);

        Box::new(drawable_text)
      }
    })
  }
}

pub struct Renderable {
  pub instruction: RenderInstruction,
  pub draw_param: Option<DrawParam>,
}

impl Component for Renderable {
  type Storage = VecStorage<Self>;
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
    Read<'a, Fonts>,
    WriteStorage<'a, Renderable>,
    ReadStorage<'a, UiElement>,
    ReadStorage<'a, Layer>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Camera>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (entities, screen_size, fonts, mut renderables, ui_elements, layers, positions, cameras) =
      data;

    let screen_size = screen_size.0;

    for (camera, camera_position) in (&cameras, &positions).join() {
      let camera_position = camera_position.0;

      let mut renderable_entities: Vec<(Entity, Renderable)> =
        (&*entities, renderables.drain()).join().collect();

      renderable_entities
        .sort_by_key(|(entity, _)| layers.get(*entity).map(|layer| layer.0).unwrap_or_default());

      for (entity, renderable) in renderable_entities {
        let mut draw_param = renderable.draw_param.unwrap_or_else(DrawParam::default);

        let drawable = renderable
          .instruction
          .construct(self.ctx, &fonts.0)
          .unwrap();

        if let Some(ui_element) = &ui_elements.get(entity) {
          draw_param.dest = ui_element.anchor.unwrap_or_default().get_postion(Rect::new(
            0.0,
            0.0,
            screen_size.x,
            screen_size.y,
          ));

          if let Some(dimensions) = drawable.dimensions(self.ctx) {
            draw_param.dest -= ui_element
              .origin
              .unwrap_or_default()
              .get_postion(dimensions)
              .coords;
          }
        } else {
          draw_param.dest =
            world_to_screen(draw_param.dest, camera_position, camera.zoom, screen_size);

          draw_param.scale = Vector2::repeat(camera.zoom);
        }

        if let Some(position) = positions.get(entity) {
          draw_param.dest = position.0;
        }

        drawable.draw(self.ctx, draw_param).unwrap();
      }
    }
  }
}

pub fn setup<'a, 'b>(
  _ctx: &mut Context,
  world: &mut World,
  _dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
  world.register::<Renderable>();
}
