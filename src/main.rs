#![feature(duration_float)]

mod debug;
mod graphics;
mod input;
mod painter;
mod renderers;

use crate::{
    graphics::{rendering::RenderingSystem, ScreenSize},
    input::{InputState, Keys, MouseButtons, MouseMotion, MousePosition, MouseWheel},
};
use ggez::{
    conf::WindowMode,
    event,
    graphics::{self as ggez_graphics, Rect},
    input::{
        keyboard::{KeyCode, KeyMods},
        mouse::{self, MouseButton},
    },
    nalgebra::Vector2,
    Context, GameResult,
};
use specs::prelude::*;
use std::time::Instant;

pub static SCREEN_WIDTH: f32 = 800.0;
pub static SCREEN_HEIGHT: f32 = 600.0;

#[derive(Default)]
pub struct DeltaTime(pub f32);

struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    last_frame: Instant,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {
        let mut world = specs::World::new();
        let mut dispatcher_builder = DispatcherBuilder::new();

        world.add_resource(DeltaTime::default());

        graphics::setup(ctx, &mut world, &mut dispatcher_builder);
        input::setup(ctx, &mut world, &mut dispatcher_builder);

        renderers::setup(ctx, &mut world, &mut dispatcher_builder);

        debug::setup(ctx, &mut world, &mut dispatcher_builder);

        let dispatcher = dispatcher_builder.build();

        Ok(MainState {
            world,
            dispatcher,
            last_frame: Instant::now(),
        })
    }
}

impl<'a, 'b> MainState<'a, 'b> {
    fn update_delta_time(&mut self) {
        let mut delta = self.world.write_resource::<DeltaTime>();
        delta.0 = self.last_frame.elapsed().as_float_secs() as f32;
    }

    fn update_mouse_position(&mut self, ctx: &mut Context) {
        let mut mouse_position = self.world.write_resource::<MousePosition>();
        mouse_position.0 = mouse::position(ctx).into();
    }

    fn update_screen_size(&mut self, width: f32, height: f32) {
        let mut screen_size = self.world.write_resource::<ScreenSize>();
        screen_size.0 = Vector2::new(width, height);
    }

    fn update_keys(&mut self) {
        let mut keys = self.world.write_resource::<Keys>();
        keys.update();
    }

    fn update_mouse_buttons(&mut self) {
        let mut mouse_buttons = self.world.write_resource::<MouseButtons>();
        mouse_buttons.update();
    }

    fn update_mouse_motion(&mut self, mouse_motion: Option<Vector2<f32>>) {
        let mut mouse_motion_res = self.world.write_resource::<MouseMotion>();

        mouse_motion_res.0 = mouse_motion.map(|mouse_motion| {
            mouse_motion_res
                .0
                .map_or(mouse_motion, |motion| motion + mouse_motion)
        });
    }

    fn update_mouse_wheel(&mut self, mouse_wheel: Option<(Vector2<f32>)>) {
        let mut mouse_wheel_res = self.world.write_resource::<MouseWheel>();

        mouse_wheel_res.0 = mouse_wheel.map(|mouse_wheel| {
            mouse_wheel_res
                .0
                .map_or(mouse_wheel, |motion| motion + mouse_wheel)
        });
    }

    fn render(&mut self, ctx: &mut Context) {
        let mut rendering_system = RenderingSystem::new(ctx);
        rendering_system.run_now(&self.world.res);
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.update_delta_time();
        self.update_mouse_position(ctx);

        self.dispatcher.dispatch(&self.world.res);

        self.update_keys();
        self.update_mouse_buttons();
        self.update_mouse_motion(None);
        self.update_mouse_wheel(None);

        self.last_frame = Instant::now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        ggez_graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.render(ctx);

        ggez_graphics::present(ctx)?;
        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let _ = ggez_graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height));

        self.update_screen_size(width, height);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        let mut mouse_buttons = self.world.write_resource::<MouseButtons>();
        mouse_buttons.0.insert(button, InputState::Pressed);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        let mut mouse_buttons = self.world.write_resource::<MouseButtons>();
        mouse_buttons.0.insert(button, InputState::Released);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        let mut keys = self.world.write_resource::<Keys>();
        keys.0.insert(keycode, InputState::Pressed);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        let mut keys = self.world.write_resource::<Keys>();
        keys.0.insert(keycode, InputState::Released);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, dx: f32, dy: f32) {
        self.update_mouse_motion(Some(Vector2::new(dx, dy)));
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.update_mouse_wheel(Some(Vector2::new(x, y)));
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
