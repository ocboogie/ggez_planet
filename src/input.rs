use ggez::{
  input::{keyboard::KeyCode, mouse::MouseButton},
  nalgebra::{Point2, Vector2},
  Context,
};
use specs::prelude::*;
use std::{collections::HashMap, hash::Hash};

pub struct MousePosition(pub Point2<f32>);

impl Default for MousePosition {
  fn default() -> Self {
    Self(Point2::new(0.0, 0.0))
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputState {
  Pressed,
  Held,
  Released,
}

pub struct InputResource<K: Hash + Eq>(pub HashMap<K, InputState>);

impl<K: Hash + Eq + Copy> Default for InputResource<K> {
  fn default() -> Self {
    Self(HashMap::new())
  }
}

impl<K: Hash + Eq + Copy> InputResource<K> {
  pub fn update(&mut self) {
    self.0 = self
      .0
      .iter()
      .filter_map(|(key, state)| match state {
        InputState::Released => None,
        _ => Some((*key, InputState::Held)),
      })
      .collect::<HashMap<K, InputState>>();
  }

  /// Returns true if the key specified was pressed down this frame or is being held
  #[allow(dead_code)]
  pub fn is_down(&self, input: &K) -> bool {
    self
      .0
      .get(input)
      .filter(|state| **state != InputState::Released)
      .is_some()
  }

  /// Returns true if the key specified was just pressed down this frame
  #[allow(dead_code)]
  pub fn is_pressed(&self, input: &K) -> bool {
    self
      .0
      .get(input)
      .filter(|state| **state == InputState::Pressed)
      .is_some()
  }

  /// Returns true if the key specified is held this frame
  #[allow(dead_code)]
  pub fn is_held(&self, input: &K) -> bool {
    self
      .0
      .get(input)
      .filter(|state| **state == InputState::Held)
      .is_some()
  }

  /// Returns true if the key specified was released this frame
  #[allow(dead_code)]
  pub fn is_released(&self, input: &K) -> bool {
    self
      .0
      .get(input)
      .filter(|state| **state == InputState::Released)
      .is_some()
  }
}

pub type Keys = InputResource<KeyCode>;

pub type MouseButtons = InputResource<MouseButton>;

#[derive(Default)]
pub struct MouseMotion(pub Option<(Vector2<f32>)>);

pub fn setup<'a, 'b>(
  _ctx: &mut Context,
  world: &mut World,
  _dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
) {
  world.add_resource(MousePosition::default());
  world.add_resource(Keys::default());
  world.add_resource(MouseButtons::default());
  world.add_resource(MouseMotion::default());
}
