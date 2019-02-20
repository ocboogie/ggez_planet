use crate::rendering::Position;
use cgmath::{EuclideanSpace, Point2, Vector2};
use ggez::Context;
use specs::prelude::*;

pub fn world_to_screen(
  from: Point2<f32>,
  camera_pos: Point2<f32>,
  zoom: f32,
  screen_size: Vector2<f32>,
) -> Point2<f32> {
  let offset = from - camera_pos;
  let view_scale = offset * zoom;

  Point2::from_vec(view_scale + screen_size / 2.0)
}

#[allow(dead_code)]
pub fn screen_to_world(
  from: Point2<f32>,
  camera_pos: Point2<f32>,
  zoom: f32,
  screen_size: Vector2<f32>,
) -> Point2<f32> {
  let screen_coords = Point2::to_vec(from) - screen_size / 2.0;
  let view_scale = screen_coords / zoom;

  camera_pos + view_scale
}

pub struct Camera {
  pub zoom: f32,
}

impl Default for Camera {
  fn default() -> Self {
    Self { zoom: 1.0 }
  }
}

impl Component for Camera {
  type Storage = VecStorage<Self>;
}

pub fn setup<'a, 'b>(
  _ctx: &mut Context,
  world: &mut World,
  _dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
  world.register::<Camera>();

  world
    .create_entity()
    .with(Camera::default())
    .with(Position(Point2::origin()))
    .build();
}