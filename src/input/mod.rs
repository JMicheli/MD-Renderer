use log::trace;
use winit::{
  dpi::PhysicalPosition,
  event::{ElementState, KeyEvent, MouseButton},
  keyboard::{KeyCode, PhysicalKey},
};

pub struct MdrInputState {
  pub left: bool,
  pub up: bool,
  pub right: bool,
  pub down: bool,

  pub w: bool,
  pub a: bool,
  pub s: bool,
  pub d: bool,

  pub mouse_position: [f32; 2],
  pub mouse_left: bool,
  pub mouse_right: bool,
  pub mouse_delta: [f32; 2],
}

pub struct MdrInputContext {
  pub state: MdrInputState,
}

impl MdrInputContext {
  pub const fn new() -> Self {
    Self {
      state: MdrInputState {
        left: false,
        up: false,
        right: false,
        down: false,

        w: false,
        a: false,
        s: false,
        d: false,

        mouse_position: [0.0, 0.0],
        mouse_left: false,
        mouse_right: false,
        mouse_delta: [0.0, 0.0],
      },
    }
  }

  pub const fn keyboard_input(&mut self, input: &KeyEvent) {
    match input.state {
      ElementState::Pressed => match input.physical_key {
        PhysicalKey::Code(KeyCode::ArrowLeft) => self.state.left = true,
        PhysicalKey::Code(KeyCode::ArrowUp) => self.state.up = true,
        PhysicalKey::Code(KeyCode::ArrowRight) => self.state.right = true,
        PhysicalKey::Code(KeyCode::ArrowDown) => self.state.down = true,

        PhysicalKey::Code(KeyCode::KeyW) => self.state.w = true,
        PhysicalKey::Code(KeyCode::KeyA) => self.state.a = true,
        PhysicalKey::Code(KeyCode::KeyS) => self.state.s = true,
        PhysicalKey::Code(KeyCode::KeyD) => self.state.d = true,

        _ => (),
      },
      ElementState::Released => match input.physical_key {
        PhysicalKey::Code(KeyCode::ArrowLeft) => self.state.left = false,
        PhysicalKey::Code(KeyCode::ArrowUp) => self.state.up = false,
        PhysicalKey::Code(KeyCode::ArrowRight) => self.state.right = false,
        PhysicalKey::Code(KeyCode::ArrowDown) => self.state.down = false,

        PhysicalKey::Code(KeyCode::KeyW) => self.state.w = false,
        PhysicalKey::Code(KeyCode::KeyA) => self.state.a = false,
        PhysicalKey::Code(KeyCode::KeyS) => self.state.s = false,
        PhysicalKey::Code(KeyCode::KeyD) => self.state.d = false,

        _ => (),
      },
    }
  }

  pub fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
    trace!("Mouse event");
    match (button, state) {
      (MouseButton::Left, ElementState::Pressed) => {
        self.state.mouse_left = true;
      }
      (MouseButton::Right, ElementState::Pressed) => {
        self.state.mouse_right = true;
      }
      (MouseButton::Left, ElementState::Released) => {
        self.state.mouse_left = false;
      }
      (MouseButton::Right, ElementState::Released) => {
        self.state.mouse_right = false;
      }
      _ => {}
    }
  }

  pub fn mouse_moved_input(&mut self, position: PhysicalPosition<f64>) {
    trace!("Mouse moved event");
    let new_position = [position.x as f32, position.y as f32];
    self.state.mouse_delta = [
      new_position[0] - self.state.mouse_position[0],
      new_position[1] - self.state.mouse_position[1],
    ];
    self.state.mouse_position = new_position;
  }

  pub const fn cleanup_after_update(&mut self) {
    // Zero mouse delta in case mouse has stopped moving
    self.state.mouse_delta = [0.0, 0.0];
  }
}

impl Default for MdrInputContext {
  fn default() -> Self {
    Self::new()
  }
}
