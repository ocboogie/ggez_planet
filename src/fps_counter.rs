use crate::renderers::column_graph::ColumnGraph;
use crate::rendering::{HorizontalDirection, UiElement, VerticalDirection};
use crate::resources::DeltaTime;
use ggez::Context;
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
    WriteStorage<'a, ColumnGraph>,
    ReadStorage<'a, FpsCounter>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (delta_time, mut column_graphs, fps_counter) = data;

    let delta_time = delta_time.0;

    for (column_graph, _) in (&mut column_graphs, &fps_counter).join() {
      if column_graph.columns.len() >= COLUMNS {
        column_graph.columns.remove(0);
      }

      column_graph.columns.push((delta_time * 1000.0) as usize);
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
    .with(ColumnGraph {
      columns: Vec::with_capacity(COLUMNS),
      size: COLUMN_WIDTH,
    })
    .with(UiElement {
      stick_horizontal: Some(HorizontalDirection::Left),
      stick_vertical: Some(VerticalDirection::Bottom),
    })
    .build();
}
