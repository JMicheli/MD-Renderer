use std::sync::Arc;

use vulkano::{instance::Instance, pipeline::graphics::viewport::Viewport, swapchain::Surface};

use winit::{
  dpi::{LogicalSize, PhysicalSize},
  event_loop::ActiveEventLoop,
  window::Window,
};

pub struct MdrWindowOptions<'a> {
  pub width: u32,
  pub height: u32,
  pub resizable: bool,
  pub title: &'a str,
}

pub struct MdrWindow {
  pub(crate) surface: Arc<Surface>,
}

impl MdrWindow {
  pub fn new(
    instance: &Arc<Instance>,
    event_loop: &ActiveEventLoop,
    options: MdrWindowOptions,
  ) -> Arc<Self> {
    // Set up event loop and build window
    let window_attributes = Window::default_attributes()
      .with_title(options.title)
      .with_inner_size(LogicalSize::new(
        f64::from(options.width),
        f64::from(options.height),
      ))
      .with_resizable(options.resizable);
    let window = event_loop.create_window(window_attributes).unwrap();

    let surface = Surface::from_window(instance.clone(), Arc::new(window)).unwrap();
    Arc::new(Self { surface })
  }

  pub fn create_viewport(&self) -> Viewport {
    Viewport {
      offset: [0.0, 0.0],
      extent: self.dimensions().into(),
      depth_range: 0.0..=1.0,
    }
  }

  /// Returns the dimensions (`extent` in Vulkan terminology) of the window.
  pub fn dimensions(&self) -> PhysicalSize<u32> {
    self.get_window().inner_size()
  }

  /// Returns whether or not the window has no visible drawing surface.
  pub fn is_minimized(&self) -> bool {
    let dimensions = self.dimensions();

    dimensions.width == 0 || dimensions.height == 0
  }

  /// Gets a reference to the window which the surface is a part of
  pub fn get_window(&self) -> &Window {
    // TODO - Any safety improvements here? This feels a bit haphazard with the unwraps and downcast.
    self
      .surface
      .object()
      .unwrap()
      .downcast_ref::<Window>()
      .unwrap()
  }
}
