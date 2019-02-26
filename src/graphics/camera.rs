use crate::graphics::Position;
use ggez::{
    nalgebra::{Point2, Vector2},
    Context,
};
use specs::prelude::*;

pub fn world_to_screen(
    from: Point2<f32>,
    camera_pos: Point2<f32>,
    zoom: f32,
    screen_size: Vector2<f32>,
) -> Point2<f32> {
    let offset = from - camera_pos;
    let view_scale = offset * zoom;

    Point2::<f32>::from(view_scale + screen_size / 2.0)
}

#[allow(dead_code)]
pub fn screen_to_world(
    from: Point2<f32>,
    camera_pos: Point2<f32>,
    zoom: f32,
    screen_size: Vector2<f32>,
) -> Point2<f32> {
    let screen_coords = from.coords - screen_size / 2.0;
    let view_scale = screen_coords / zoom;

    camera_pos + view_scale
}

pub struct Camera {
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self { zoom: 1.0 }
    }
}

impl Component for Camera {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct ActiveCamera(pub Option<Entity>);

pub fn setup<'a, 'b>(
    _ctx: &mut Context,
    world: &mut World,
    _dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
    world.register::<Camera>();

    let camera_entity = world
        .create_entity()
        .with(Camera::default())
        .with(Position(Point2::origin()))
        .build();

    world.add_resource(ActiveCamera(Some(camera_entity)));
}
