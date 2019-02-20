use crate::rendering::{HorizontalDirection, RenderType, Renderable, VerticalDirection};
use crate::resources::DeltaTime;
use ggez::{graphics::DrawParam, Context};
use specs::prelude::*;

static COLUMNS: usize = 50;
static COLUMN_WIDTH: f32 = 5.0;

#[derive(Default)]
pub struct FpsCounter;

impl Component for FpsCounter {
  type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct UpdateFps;

impl<'a> System<'a> for UpdateFps {
  type SystemData = (
    Read<'a, DeltaTime>,
    WriteStorage<'a, Renderable>,
    ReadStorage<'a, FpsCounter>,
  );

  fn run(&mut self, data: Self::SystemData) {
    use specs::Join;

    let (delta_time, mut renderable, fps_counter) = data;

    let delta_time = delta_time.0;

    for (renderable, _) in (&mut renderable, &fps_counter).join() {
      if let RenderType::ColumnGraph { columns, .. } = &mut renderable.render_type {
        if columns.len() > COLUMNS {
          columns.remove(0);
        }

        columns.push((delta_time * 1000.0) as usize);
      }
    }
  }
}

pub fn setup<'a, 'b>(
  _ctx: &mut Context,
  world: &mut World,
  dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
  world.register::<FpsCounter>();

  dispatcher_builder.add(UpdateFps, "update_fps", &[]);

  world
    .create_entity()
    .with(FpsCounter::default())
    .with(Renderable {
      draw_param: Some(DrawParam::default()),
      render_type: RenderType::ColumnGraph {
        columns: Vec::with_capacity(COLUMNS),
        size: COLUMN_WIDTH,
      },
      stick_horizontal: Some(HorizontalDirection::Left),
      stick_vertical: Some(VerticalDirection::Bottom),
    })
    .build();
}
