use super::Anchor;
use ggez::Context;
use specs::prelude::*;

#[derive(Default)]
pub struct UiElement {
    pub anchor: Option<Anchor>,
    pub origin: Option<Anchor>,
}

impl Component for UiElement {
    type Storage = DenseVecStorage<Self>;
}

pub fn setup<'a, 'b>(
    _ctx: &mut Context,
    world: &mut World,
    _dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
    world.register::<UiElement>();
}
