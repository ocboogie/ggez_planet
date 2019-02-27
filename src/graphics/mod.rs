pub mod camera;
pub mod rendering;
pub mod ui;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use ggez::{
  graphics::{Font, Rect},
  nalgebra::{Point2, Vector2},
  Context,
};
use smart_default::SmartDefault;
use specs::prelude::*;
use std::collections::HashMap;

#[derive(Copy, Clone, SmartDefault)]
pub enum Anchor {
  TopLeft,
  #[allow(dead_code)]
  TopCenter,
  #[allow(dead_code)]
  TopRight,

  #[allow(dead_code)]
  CenterLeft,
  #[default]
  Center,
  #[allow(dead_code)]
  CenterRight,

  BottomLeft,
  #[allow(dead_code)]
  BottomCenter,
  #[allow(dead_code)]
  BottomRight,
}

impl Anchor {
  pub fn get_postion(self, bounds: Rect) -> Point2<f32> {
    use Anchor::*;

    match self {
      TopLeft => Point2::new(bounds.x, bounds.y),
      TopCenter => Point2::new(bounds.x + bounds.w / 2.0, bounds.y),
      TopRight => Point2::new(bounds.x + bounds.w, bounds.y),

      CenterLeft => Point2::new(bounds.x, bounds.y + bounds.h / 2.0),
      Center => Point2::new(bounds.x + bounds.w / 2.0, bounds.y + bounds.h / 2.0),
      CenterRight => Point2::new(bounds.x + bounds.w, bounds.y + bounds.h / 2.0),

      BottomLeft => Point2::new(bounds.x, bounds.y + bounds.h),
      BottomCenter => Point2::new(bounds.x + bounds.w / 2.0, bounds.y + bounds.h),
      BottomRight => Point2::new(bounds.x + bounds.w, bounds.y + bounds.h),
    }
  }
}

pub struct Position(pub Point2<f32>);

impl Component for Position {
  type Storage = VecStorage<Self>;
}

pub struct Layer(pub i32);

impl Component for Layer {
  type Storage = VecStorage<Self>;
}

#[derive(SmartDefault)]
pub struct ScreenSize(#[default(Vector2::new(SCREEN_WIDTH, SCREEN_HEIGHT))] pub Vector2<f32>);

#[derive(Default)]
pub struct Fonts(pub HashMap<&'static str, Font>);

pub fn setup<'a, 'b>(
  ctx: &mut Context,
  world: &mut World,
  dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
  world.register::<Position>();
  world.register::<Layer>();

  world.add_resource(ScreenSize::default());
  world.add_resource(Fonts({
    let mut fonts = HashMap::new();

    fonts.insert(
      "roboto",
      Font::new_glyph_font_bytes(ctx, include_bytes!("../../resources/Roboto.ttf")).unwrap(),
    );

    fonts
  }));

  camera::setup(ctx, world, dispatcher_builder);
  ui::setup(ctx, world, dispatcher_builder);
  rendering::setup(ctx, world, dispatcher_builder);
}
