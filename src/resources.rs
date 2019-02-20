use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;
use cgmath::Vector2;
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

pub fn setup<'a, 'b>(
  _ctx: &mut Context,
  world: &mut World,
  _dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
  world.add_resource(DeltaTime::default());
  world.add_resource(ScreenSize::default());
}
