use crate::renderers::text::Text;
use crate::rendering::{Anchor, UiElement};
use ggez::graphics::Scale;
use ggez::Context;
use specs::prelude::*;
use std::time::Instant;

static UPDATE_INTERVAL: f64 = 0.5;
static FONT_SIZE: f32 = 48.0;

#[derive(Default)]
pub struct FpsDisplay;

impl Component for FpsDisplay {
  type Storage = NullStorage<Self>;
}

pub struct UpdateFpsDisplay {
  last_update: Instant,
  frames_since_last_update: usize,
}

impl Default for UpdateFpsDisplay {
  fn default() -> Self {
    Self {
      last_update: Instant::now(),
      frames_since_last_update: 0,
    }
  }
}

impl<'a> System<'a> for UpdateFpsDisplay {
  type SystemData = (ReadStorage<'a, FpsDisplay>, WriteStorage<'a, Text>);

  fn run(&mut self, data: Self::SystemData) {
    let (fps_displays, mut texts) = data;

    self.frames_since_last_update += 1;

    let elapsed = self.last_update.elapsed().as_float_secs();

    if elapsed > UPDATE_INTERVAL {
      for (text, _) in (&mut texts, &fps_displays).join() {
        text.text = format!("{:.1}", self.frames_since_last_update as f64 / elapsed);
      }

      self.frames_since_last_update = 0;
      self.last_update = Instant::now();
    }
  }
}

pub fn setup<'a, 'b>(
  _ctx: &mut Context,
  world: &mut World,
  dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
  world.register::<FpsDisplay>();

  dispatcher_builder.add(UpdateFpsDisplay::default(), "update_fps_display", &[]);

  world
    .create_entity()
    .with(Text {
      text: String::default(),
      font: "roboto",
      scale: Scale::uniform(FONT_SIZE),
    })
    .with(FpsDisplay::default())
    .with(UiElement {
      anchor: Some(Anchor::TopLeft),
      origin: Some(Anchor::TopLeft),
    })
    .build();
}
