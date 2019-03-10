# What is this?
ggez_planet is kinda a wrapper around [ggez](https://github.com/ggez/ggez) to use [specs](https://github.com/slide-rs/specs) for rendering, and a lot of other things

## How to use it?
To get setup you need to make a ["planet"](https://github.com/ocboogie/ggez_planet/blob/master/src/lib.rs#L27), which takes a [`World`](https://docs.rs/specs/0.14.3/specs/world/struct.World.html) and a [`DispatcherBuilder`](https://docs.rs/specs/0.14.3/specs/struct.DispatcherBuilder.html), and give that to ggez as a state:
```rust
let cb = ggez::ContextBuilder::new("ggez_planet", "ggez_planet");
let (ctx, event_loop) = &mut cb.build()?;

let mut world = specs::World::new();
let mut dispatcher_builder = specs::DispatcherBuilder::new();

// Create Planet, which implements ggez's EventHandler so we can just give it to ggez
let state = &mut Planet::new(ctx, world, dispatcher_builder);
ggez::event::run(ctx, event_loop, state)
```
Sense `Planet` implements `ggez::event::EventHandler` we can just pass it to `ggez::event::run`.

#### Rendering
How rendering works in ggez_planet is every entity that has the `Renderable` component will be rendered to the screen with the instructions given by the component, so the following would just make a white square:
```rust
Renderable {
    instruction: RenderInstruction::Image(ImageBuilder::Solid {
        size: 1,
        color: WHITE,
    }),
    draw_param: None,
}
```

But every time we render a `Renderable` we remove the component from the entity, so we have to make a system to reapply that  `Renderable` component, a renderer.
A renderer for a component called `WhiteSquare` would look something like the following:
```rust
pub struct WhiteSquare {
    size: usize
}

impl Component for WhiteSquare {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct WhiteSquareRenderer;

impl<'a> System<'a> for WhiteSquareRenderer {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Renderable>,
        ReadStorage<'a, WhiteSquare>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut renderables, white_squares) = data;

        for (entity, white_square) in (&*entities, &white_squares).join() {
            let white_square_image = ImageBuilder::Solid {
                size: white_square.size,
                color: WHITE,
            };

            let _ = renderables.insert(
                entity,
                Renderable {
                    instruction: RenderInstruction::Image(white_square_image),
                    draw_param: None,
                },
            );
        }
    }
}
```
This would add a `Renderable` component, with the instruction to render a white square, to every entity with the `WhiteSquare` component every frame.

This readme is not complete. Look at [this](https://github.com/ocboogie/ggez_planet/blob/master/examples/drawing.rs) for a better understanding.