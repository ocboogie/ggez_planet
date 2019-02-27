use crate::{
  graphics::{camera::Camera, Position},
  input::{MouseButtons, MouseMotion, MouseWheel},
};
use ggez::{input::mouse::MouseButton, Context};
use specs::prelude::*;

static ZOOM_SPEED: f32 = 1.1;

struct ScrollZoom;

impl<'a> System<'a> for ScrollZoom {
  type SystemData = (Read<'a, MouseWheel>, WriteStorage<'a, Camera>);

  fn run(&mut self, data: Self::SystemData) {
    let (mouse_wheel, mut cameras) = data;

    if let Some(mouse_wheel) = mouse_wheel.0 {
      // This could be 0 if the user scrolls horizontally
      if mouse_wheel.y == 0.0 {
        return;
      }

      for camera in (&mut cameras).join() {
        if mouse_wheel.y < 0.0 {
          camera.zoom /= ZOOM_SPEED.powf(mouse_wheel.y.abs())
        } else {
          camera.zoom *= ZOOM_SPEED.powf(mouse_wheel.y)
        };
      }
    }
  }
}

struct MousePan;

impl<'a> System<'a> for MousePan {
  type SystemData = (
    Read<'a, MouseMotion>,
    Read<'a, MouseButtons>,
    WriteStorage<'a, Position>,
    WriteStorage<'a, Camera>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (mouse_motion, mouse_buttons, mut positions, mut cameras) = data;

    if !mouse_buttons.is_down(&MouseButton::Middle) {
      return;
    }

    if let Some(mouse_motion) = mouse_motion.0 {
      for (camera, position) in (&mut cameras, &mut positions).join() {
        position.0 -= mouse_motion / camera.zoom;
      }
    }
  }
}

pub fn setup<'a, 'b>(
  _ctx: &mut Context,
  _world: &mut World,
  dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
  dispatcher_builder.add(ScrollZoom, "scroll_zoom", &[]);
  dispatcher_builder.add(MousePan, "mouse_pan", &[]);
}
