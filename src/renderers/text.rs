use crate::rendering::{RenderInstruction, Renderable};
use ggez::{graphics::Scale, Context};
use specs::prelude::*;

// TODO: Implement color text
pub struct Text {
  pub text: String,
  pub font: &'static str,
  pub scale: Scale,
}

impl Component for Text {
  type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct TextRenderer;

impl<'a> System<'a> for TextRenderer {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, Renderable>,
    ReadStorage<'a, Text>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (entities, mut renderables, texts) = data;

    for (entitiy, text) in (&*entities, &texts).join() {
      let _ = renderables.insert(
        entitiy,
        Renderable {
          instruction: RenderInstruction::Text {
            text: text.text.clone(),
            font: text.font,
            scale: text.scale,
          },
          draw_param: None,
        },
      );
    }
  }
}

pub fn setup<'a, 'b>(
  _ctx: &mut Context,
  world: &mut World,
  dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
  world.register::<Text>();

  dispatcher_builder.add(TextRenderer::default(), "text_renderer", &[]);
}
