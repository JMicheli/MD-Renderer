use std::time::Instant;

use winit::{
  application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
  window::WindowId,
};

use crate::{
  MdrApplication,
  application::MdrRunOptions,
  graphics::{MdrGraphicsContext, MdrResourceManager},
  input::MdrInputContext,
  scene::MdrScene,
};

pub struct MdrEngine {
  pub scene: MdrScene,

  graphics_context: MdrGraphicsContext,
  input_context: MdrInputContext,

  last_update: Instant,
}

impl MdrEngine {
  pub fn new(event_loop: &ActiveEventLoop, options: &MdrRunOptions) -> Self {
    Self {
      scene: MdrScene::new(),

      graphics_context: MdrGraphicsContext::new(event_loop, options.debug),
      input_context: MdrInputContext::new(),
      last_update: Instant::now(),
    }
  }

  pub const fn manage_resources(&mut self) -> &mut MdrResourceManager {
    &mut self.graphics_context.resource_manager
  }

  pub fn handle_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
    match event {
      WindowEvent::Resized(_) => {
        tracing::trace!("Resized");
        self.graphics_context.notify_resized();
      }
      WindowEvent::CloseRequested => {
        tracing::info!("Exiting");
        event_loop.exit();
      }
      WindowEvent::Destroyed => {
        // TODO - Figure out how to use this for shutdown?
        tracing::info!("Window destroyed");
      }
      WindowEvent::KeyboardInput { event, .. } => {
        self.input_context.keyboard_input(&event);
      }
      WindowEvent::CursorMoved { position, .. } => {
        self.input_context.mouse_moved_input(position);
      }
      WindowEvent::MouseInput { state, button, .. } => {
        self.input_context.mouse_input(state, button);
      }
      WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
        // TODO - I think I may need to use this to fix a crash?
        tracing::info!("Scale factor changed to {scale_factor}");
      }
      WindowEvent::RedrawRequested => {
        self
          .graphics_context
          .update_scene_aspect_ratio(&mut self.scene);

        self.graphics_context.draw(&self.scene);
      }
      _ => (),
    }
  }

  pub fn do_update(&mut self, application: &mut dyn MdrApplication) {
    let current_instant = Instant::now();
    let dt = (current_instant - self.last_update).as_secs_f32();

    application.update(&mut self.scene, &self.input_context.state, dt);

    self.last_update = current_instant;
    self.graphics_context.window.get_window().request_redraw();
    self.input_context.cleanup_after_update();
  }
}

/// This structure holds the winit application state and implements [`ApplicationHandler`]. It is
/// responsible for creating the [`MdrEngine`] and issuing calls to be handled by the provided
/// [`MdrApplication`].
///
/// The only way it should ever be created is by running [`crate::application::run_application`].
pub(crate) struct InternalApplication {
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
    // Create engine
    self.engine = Some(MdrEngine::new(event_loop, &self.options));

    // Run application-provided initialization
    let engine = self.engine.as_mut().unwrap();
    self.application.initialize(engine);
  }

  fn about_to_wait(&mut self, _: &ActiveEventLoop) {
    if let Some(engine) = self.engine.as_mut() {
      engine.do_update(self.application.as_mut());
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
      tracing::info!("Exiting without existing engine");
    }
  }
}
