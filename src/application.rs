use log::info;
use winit::{
  application::ApplicationHandler,
  event::WindowEvent,
  event_loop::{ActiveEventLoop, EventLoop},
  window::WindowId,
};

use crate::{engine::MdrEngine, input::MdrInputState, scene::MdrScene};

/// Represents an application that runs using the rendering engine.
pub trait MdrApplication {
  /// Initial setup, runs once at application startup.
  fn initialize(&self, engine: &mut MdrEngine);

  /// Application main loop update step.
  fn update(&self, scene: &mut MdrScene, input_state: &MdrInputState, dt: f32);

  /// Teardown function, runs once at application shutdown.
  fn shutdown(&self, engine: &mut MdrEngine);
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
    .unwrap()
}

/// This structure holds the winit application state and implements [`ApplicationHandler`]. It is
/// responsible for creating the [`MdrEngine`] and issuing calls to be handled by the provided
/// [`MdrApplication`]. The only way it should ever be created is by running [`run_application`].
struct InternalApplication {
  engine: Option<MdrEngine>,
  application: Box<dyn MdrApplication>,
  options: MdrRunOptions,
}

impl InternalApplication {
  pub fn new(application: impl MdrApplication + 'static, options: MdrRunOptions) -> Self {
    Self {
      engine: None,
      application: Box::new(application),
      options,
    }
  }
}

impl ApplicationHandler for InternalApplication {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    // Start engine
    self.engine = Some(MdrEngine::new(event_loop, &self.options));

    // Run initialization
    let engine = self.engine.as_mut().unwrap();
    self.application.initialize(engine);
  }

  fn about_to_wait(&mut self, _: &ActiveEventLoop) {
    if let Some(engine) = self.engine.as_mut() {
      engine.do_update(self.application.as_ref());
    }
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
    let engine = self.engine.as_mut().unwrap();
    engine.handle_event(event_loop, event);
  }

  fn exiting(&mut self, _: &ActiveEventLoop) {
    if let Some(engine) = self.engine.as_mut() {
      self.application.shutdown(engine);
    } else {
      info!("Exiting without existing engine")
    }
  }
}
