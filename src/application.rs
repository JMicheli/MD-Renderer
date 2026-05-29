use winit::event_loop::EventLoop;

use crate::{
  engine::{InternalApplication, MdrEngine},
  input::MdrInputState,
  scene::MdrScene,
};

/// Represents an application that runs using the rendering engine.
pub trait MdrApplication {
  /// Initial setup, runs once at application startup.
  fn initialize(&mut self, engine: &mut MdrEngine);

  /// Application main loop update step.
  fn update(&mut self, scene: &mut MdrScene, input_state: &MdrInputState, dt: f32);

  /// Teardown function, runs once at application shutdown.
  fn shutdown(&mut self, engine: &mut MdrEngine);
}

// Configure the way that the application is run
pub struct MdrRunOptions {
  pub debug: bool,
}

/// Start an [`MdrApplication`] with the given [`MdrRunOptions`].
pub fn run_application(application: impl MdrApplication + 'static, options: MdrRunOptions) {
  EventLoop::new()
    .unwrap()
    .run_app(&mut InternalApplication::new(application, options))
    .unwrap();
}
