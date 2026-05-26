use std::time::Instant;

use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

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

  pub fn do_update(&mut self, application: &dyn MdrApplication) {
    let current_instant = Instant::now();
    let dt = (current_instant - self.last_update).as_secs_f32();

    application.update(&mut self.scene, &self.input_context.state, dt);

    self.last_update = current_instant;
    self.graphics_context.window.get_window().request_redraw();
    self.input_context.cleanup_after_update();
  }
}

// Event::MainEventsCleared => {
//   self
//     .update_context
//     .update_scene(&mut self.scene, &self.input_context.state);
//   self.input_context.cleanup_after_update();
//   None
// }
