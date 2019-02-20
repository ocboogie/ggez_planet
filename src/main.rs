#![feature(duration_float)]

mod camera;
mod fps_counter;
mod rendering;
mod resources;

use crate::resources::DeltaTime;
use crate::resources::ScreenSize;
use cgmath::Vector2;
use ggez::conf::WindowMode;
use ggez::graphics;
use ggez::{event, Context, GameResult};
use rendering::RenderingSystem;
use specs::prelude::*;
use std::time::Instant;

pub static SCREEN_WIDTH: f32 = 800.0;
pub static SCREEN_HEIGHT: f32 = 600.0;

struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    last_frame: Instant,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {
        let mut world = specs::World::new();
        let mut dispatcher_builder = DispatcherBuilder::new();

        resources::setup(ctx, &mut world, &mut dispatcher_builder);
        rendering::setup(ctx, &mut world, &mut dispatcher_builder);
        fps_counter::setup(ctx, &mut world, &mut dispatcher_builder);
        camera::setup(ctx, &mut world, &mut dispatcher_builder);

        let dispatcher = dispatcher_builder.build();

        Ok(MainState {
            world,
            dispatcher,
            last_frame: Instant::now(),
        })
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        {
            let mut delta = self.world.write_resource::<DeltaTime>();
            delta.0 = self.last_frame.elapsed().as_float_secs() as f32;
        }

        self.dispatcher.dispatch(&self.world.res);

        self.last_frame = Instant::now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        {
            let mut rendering_system = RenderingSystem::new(ctx);
            rendering_system.run_now(&self.world.res);
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let mut screen_size = self.world.write_resource::<ScreenSize>();
        screen_size.0 = Vector2::new(width, height);

        let _ = graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height));
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez").window_mode(
        WindowMode::default()
            .resizable(true)
            .dimensions(SCREEN_WIDTH, SCREEN_HEIGHT),
    );
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;

    event::run(ctx, event_loop, state)
}
