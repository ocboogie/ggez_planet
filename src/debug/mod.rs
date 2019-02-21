pub mod fps_display;
pub mod performance_graph;

use ggez::Context;
use specs::prelude::*;

pub fn setup<'a, 'b>(
    ctx: &mut Context,
    world: &mut World,
    dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
    fps_display::setup(ctx, world, dispatcher_builder);
    performance_graph::setup(ctx, world, dispatcher_builder);
}
