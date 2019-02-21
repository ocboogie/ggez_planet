pub mod column_graph;
pub mod text;

use ggez::Context;
use specs::prelude::*;

pub fn setup<'a, 'b>(
  ctx: &mut Context,
  world: &mut World,
  dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
  column_graph::setup(ctx, world, dispatcher_builder);
  text::setup(ctx, world, dispatcher_builder);
}
