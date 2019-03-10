extern crate ggez_planet;
extern crate line_drawing;

use line_drawing::Bresenham;
use specs::prelude::*;

use ggez::{
    event,
    graphics::{DrawParam, WHITE},
    input::mouse::MouseButton,
    nalgebra::Point2,
    Context, GameResult,
};
use ggez_planet::{
    graphics::{
        camera::{screen_to_world, ActiveCamera, Camera},
        rendering::{ImageBuilder, RenderInstruction, Renderable},
        Position, ScreenSize,
    },
    input::{MouseButtons, MouseMotion, MousePosition, MouseWheel},
    Planet,
};
use std::collections::HashSet;

static PIXEL_SIZE: f32 = 10.0;
static ZOOM_SPEED: f32 = 1.1;

// Canvas is just a basic component that holds some data that is used to render it
pub struct Canvas {
    pixels: HashSet<Point2<i32>>,
}

impl Component for Canvas {
    type Storage = VecStorage<Self>;
}

// All renderers are just systems that query specific components and add the
// renderable component to it.
// So a canvas renderer would just query for all the canvases and add
// the Renderable component with the instructions on how to render them
#[derive(Default)]
pub struct CanvasRenderer;

impl<'a> System<'a> for CanvasRenderer {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Renderable>,
        ReadStorage<'a, Canvas>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut renderables, canvases) = data;

        for (entity, canvas) in (&*entities, &canvases).join() {
            let image_builder = ImageBuilder::Solid {
                size: PIXEL_SIZE as u16,
                color: WHITE,
            };

            let sprites: Vec<DrawParam> = canvas
                .pixels
                .iter()
                .map(|pos| {
                    DrawParam::default().dest(Point2::<f32>::from(
                        pos.coords.map(|pos| pos as f32) * PIXEL_SIZE,
                    ))
                })
                .collect();

            let _ = renderables.insert(
                entity,
                Renderable {
                    instruction: RenderInstruction::SpriteBatch {
                        image_builder,
                        sprites,
                    },
                    draw_param: None,
                },
            );
        }
    }
}

// This is a system that tracks the mouse and adds pixels to the canvas accordingly
#[derive(Default)]
pub struct MousePaint {
    last_mouse_position: Option<Point2<i32>>,
}

impl<'a> System<'a> for MousePaint {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'a, MousePosition>,
        Read<'a, MouseButtons>,
        Read<'a, ScreenSize>,
        Read<'a, ActiveCamera>,
        WriteStorage<'a, Canvas>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mouse_position,
            mouse_buttons,
            screen_size,
            active_camera,
            mut canvases,
            cameras,
            positions,
        ) = data;

        let mouse_position = mouse_position.0;
        let screen_size = screen_size.0;

        if !mouse_buttons.is_down(&MouseButton::Left) {
            self.last_mouse_position = None;
            return;
        }

        // Get the active camera's entity
        if let Some(active_camera_entity) = active_camera.0 {
            // and it's position
            if let Some(camera_position) = positions.get(active_camera_entity) {
                let camera_position = camera_position.0;
                // and it's camera component itself
                if let Some(camera) = cameras.get(active_camera_entity) {
                    // Figure out which pixel the mouse is on
                    let mouse_position: Point2<i32> =
                        screen_to_world(mouse_position, camera_position, camera.zoom, screen_size)
                            .coords
                            .map(|pos| (pos / PIXEL_SIZE as f32).floor() as i32)
                            .into();

                    let last_mouse_position =
                        self.last_mouse_position.unwrap_or_else(|| mouse_position);

                    // Loop through all the canvas,
                    // which there really only should be one,
                    // and add the pixels to them
                    for canvas in (&mut canvases).join() {
                        // Using the bresenham algorithm to interpolate
                        // the mouse position in between frames
                        for (x, y) in Bresenham::new(
                            (last_mouse_position.x, last_mouse_position.y),
                            (mouse_position.x, mouse_position.y),
                        ) {
                            canvas.pixels.insert(Point2::new(x, y));
                        }
                    }
                    self.last_mouse_position = Some(mouse_position);
                }
            }
        }
    }
}

// Camera panning with middle mouse
struct MousePan;

// TODO: Only pam the active camera
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

// Camera zooming with mouse wheel
struct ScrollZoom;

// TODO: Only zoom the active camera
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

pub fn setup<'a, 'b>(
    _ctx: &mut Context,
    world: &mut World,
    dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
    world.register::<Canvas>();
    dispatcher_builder.add(CanvasRenderer::default(), "canvas_renderer", &[]);
    dispatcher_builder.add(MousePaint::default(), "mouse_paint", &[]);

    dispatcher_builder.add(MousePan, "mouse_pan", &[]);
    dispatcher_builder.add(ScrollZoom, "scroll_zoom", &[]);

    world
        .create_entity()
        .with(Canvas {
            pixels: {
                let mut set = HashSet::new();

                set.insert(Point2::new(0, 0));
                set
            },
        })
        .build();
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("drawing_with_ggez_planet", "ggez_planet");
    let (ctx, event_loop) = &mut cb.build()?;

    let mut world = specs::World::new();
    let mut dispatcher_builder = DispatcherBuilder::new();

    // Setup the world and dispatcher for ggez planet to use
    setup(ctx, &mut world, &mut dispatcher_builder);

    // Create Planet, which implements ggez's EventHandler so we can just run it
    let state = &mut Planet::new(ctx, world, dispatcher_builder);
    event::run(ctx, event_loop, state)
}
