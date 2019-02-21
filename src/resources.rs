use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;
use cgmath::{Point2, Vector2};
use ggez::Context;
use specs::prelude::*;

#[derive(Default)]
pub struct DeltaTime(pub f32);

pub struct ScreenSize(pub Vector2<f32>);

impl Default for ScreenSize {
  fn default() -> Self {
    Self(Vector2::new(SCREEN_WIDTH, SCREEN_HEIGHT))
  }
}

pub struct MousePosition(pub Point2<f32>);

impl Default for MousePosition {
  fn default() -> Self {
    Self(Point2::new(0.0, 0.0))
  }
}

pub fn setup<'a, 'b>(
  _ctx: &mut Context,
  world: &mut World,
  _dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
  world.add_resource(DeltaTime::default());
  world.add_resource(ScreenSize::default());
  world.add_resource(MousePosition::default());
}
