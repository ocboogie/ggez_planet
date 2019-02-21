use crate::graphics::rendering::{ImageBuilder, RenderInstruction, Renderable};
use ggez::{
    graphics::{DrawParam, BLACK},
    nalgebra::{Point2, Vector2},
    Context,
};
use specs::prelude::*;

pub struct ColumnGraph {
    pub columns: Vec<usize>,
    pub size: f32,
}

impl Component for ColumnGraph {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct ColumnGraphRenderer;

impl<'a> System<'a> for ColumnGraphRenderer {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Renderable>,
        ReadStorage<'a, ColumnGraph>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut renderables, column_graphs) = data;

        for (entitiy, column_graph) in (&*entities, &column_graphs).join() {
            let image_builder = ImageBuilder::Solid {
                size: 1,
                color: BLACK,
            };
            let sprites: Vec<DrawParam> = column_graph
                .columns
                .iter()
                .cloned()
                .enumerate()
                .map(|(index, column)| {
                    DrawParam::default()
                        .dest(Point2::new(index as f32 * column_graph.size, 0.0))
                        .scale(Vector2::new(column_graph.size, -(column as f32)))
                })
                .collect();

            let _ = renderables.insert(
                entitiy,
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

pub fn setup<'a, 'b>(
    _ctx: &mut Context,
    world: &mut World,
    dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
    world.register::<ColumnGraph>();
    dispatcher_builder.add(ColumnGraphRenderer::default(), "column_graph_renderer", &[]);
}
