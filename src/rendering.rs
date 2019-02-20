use crate::resources::ScreenSize;
use cgmath::{Point2, Vector2};
use ggez::graphics::{self, spritebatch::SpriteBatch, DrawParam, Image, Mesh, BLACK};
use ggez::Context;
use specs::prelude::*;

pub enum RenderType {
  #[allow(dead_code)]
  Image(Image),
  #[allow(dead_code)]
  SpriteBatch(SpriteBatch),
  #[allow(dead_code)]
  Mesh(Mesh),
  ColumnGraph {
    columns: Vec<usize>,
    size: f32,
  },
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
  pub render_type: RenderType,
  pub draw_param: Option<DrawParam>,
  pub stick_horizontal: Option<HorizontalDirection>,
  pub stick_vertical: Option<VerticalDirection>,
}

impl Component for Renderable {
  type Storage = VecStorage<Self>;
}

pub struct Position(pub Point2<f32>);

impl Component for Position {
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
  type SystemData = (
    Entities<'a>,
    Read<'a, ScreenSize>,
    ReadStorage<'a, Renderable>,
    ReadStorage<'a, Position>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (entities, screen_size, renderables, positions) = data;

    let screen_size = screen_size.0;

    for (entitie, renderable) in (&*entities, &renderables).join() {
      let mut draw_param = renderable.draw_param.unwrap_or_else(DrawParam::default);

      if let Some(position) = positions.get(entitie) {
        // Reassigning "draw_param" because the dest property is using nalgebra
        // so we can't change it directly
        draw_param = draw_param.dest(position.0);
      }

      if let Some(direction) = &renderable.stick_horizontal {
        draw_param.dest.x = match direction {
          HorizontalDirection::Left => 0.0,
          HorizontalDirection::Middle => screen_size.x / 2.0,
          HorizontalDirection::Right => screen_size.x,
        };
      }

      if let Some(direction) = &renderable.stick_vertical {
        draw_param.dest.y = match direction {
          VerticalDirection::Top => 0.0,
          VerticalDirection::Middle => screen_size.y / 2.0,
          VerticalDirection::Bottom => screen_size.y,
        };
      }

      match &renderable.render_type {
        RenderType::Image(image) => graphics::draw(self.ctx, image, draw_param).unwrap(),
        RenderType::SpriteBatch(sprite_batch) => {
          graphics::draw(self.ctx, sprite_batch, draw_param).unwrap()
        }
        RenderType::Mesh(mesh) => graphics::draw(self.ctx, mesh, draw_param).unwrap(),
        RenderType::ColumnGraph { columns, size } => {
          let mut spritebatch = SpriteBatch::new(Image::solid(self.ctx, 1, BLACK).unwrap());

          for (index, column) in columns.iter().enumerate() {
            spritebatch.add(
              draw_param
                .dest(Point2::new(index as f32 * *size, 0.0))
                .scale(Vector2::new(*size, -(*column as f32))),
            );
          }

          graphics::draw(self.ctx, &spritebatch, draw_param).unwrap();
        }
      };
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
}
