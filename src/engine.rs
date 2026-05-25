use std::time::Instant;

use log::{info, trace};
use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

use crate::{
  application::MdrRunOptions,
  graphics::{MdrGraphicsContext, MdrResourceManager},
  input::{MdrInputContext, MdrInputState},
  scene::MdrScene,
  update::MdrUpdateContext,
  MdrApplication,
};

pub struct MdrEngine {
  pub scene: MdrScene,

  graphics_context: MdrGraphicsContext,
  input_context: MdrInputContext,
  update_context: MdrUpdateContext,

  last_update: Instant,
}

impl MdrEngine {
  pub fn new(event_loop: &ActiveEventLoop, options: &MdrRunOptions) -> Self {
    Self {
      scene: MdrScene::new(),

      graphics_context: MdrGraphicsContext::new(event_loop, options.debug),
      input_context: MdrInputContext::new(),
      update_context: MdrUpdateContext::new(),

      last_update: Instant::now(),
    }
  }

  pub fn manage_resources(&mut self) -> &mut MdrResourceManager {
    &mut self.graphics_context.resource_manager
  }

  pub fn set_update_function(&mut self, f: Box<dyn FnMut(&mut MdrScene, &MdrInputState, f32)>) {
    self.update_context.set_update_function(f);
  }

  pub fn handle_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
    match event {
      WindowEvent::Resized(_) => {
        trace!("Resized");
        self.graphics_context.notify_resized();
      }
      WindowEvent::CloseRequested => {
        info!("Exiting");
        event_loop.exit();
      }
      WindowEvent::Destroyed => {
        // TODO - Figure out how to use this for shutdown?
        info!("Window destroyed")
      }
      WindowEvent::KeyboardInput { event, .. } => {
        self.input_context.keyboard_input(&event);
      }
      WindowEvent::CursorMoved { position, .. } => {
        self.input_context.mouse_moved_input(position);
      }
      WindowEvent::MouseInput { state, button, .. } => {
        self.input_context.mouse_input(&state, &button);
      }
      WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
        // TODO - I think I may need to use this to fix a crash?
        info!("Scale factor changed to {scale_factor}");
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

  pub fn do_update(&mut self, application: &Box<dyn MdrApplication>) {
    self
      .update_context
      .update_scene(&mut self.scene, &self.input_context.state);

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
