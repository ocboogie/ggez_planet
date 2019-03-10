#![feature(duration_float)]

pub mod debug;
pub mod graphics;
pub mod input;
pub mod renderers;

use crate::{
    graphics::{rendering::RenderingSystem, ScreenSize},
    input::{InputState, Keys, MouseButtons, MouseMotion, MousePosition, MouseWheel},
};
use ggez::{
    event,
    graphics::{self as ggez_graphics, Rect},
    input::{
        keyboard::{KeyCode, KeyMods},
        mouse::{self, MouseButton},
    },
    nalgebra::Vector2,
    timer, Context, GameResult,
};
use specs::{prelude::*, shred::RunNow};

#[derive(Default)]
pub struct DeltaTime(pub f32);

pub struct Planet<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Planet<'a, 'b> {
    pub fn new(
        ctx: &mut Context,
        mut world: World,
        mut dispatcher_builder: DispatcherBuilder<'a, 'b>,
    ) -> Self {
        world.add_resource(DeltaTime::default());

        graphics::setup(ctx, &mut world, &mut dispatcher_builder);
        input::setup(ctx, &mut world, &mut dispatcher_builder);

        renderers::setup(ctx, &mut world, &mut dispatcher_builder);

        debug::setup(ctx, &mut world, &mut dispatcher_builder);

        Self {
            world,
            dispatcher: dispatcher_builder.build(),
        }
    }

    fn update_delta_time(&mut self, ctx: &mut Context) {
        let mut delta = self.world.write_resource::<DeltaTime>();
        delta.0 = timer::delta(ctx).as_float_secs() as f32;
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

impl<'a, 'b> event::EventHandler for Planet<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.update_delta_time(ctx);
        self.update_mouse_position(ctx);

        self.dispatcher.dispatch(&self.world.res);

        self.update_keys();
        self.update_mouse_buttons();
        self.update_mouse_motion(None);
        self.update_mouse_wheel(None);

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
