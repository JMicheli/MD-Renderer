use std::sync::Arc;

use vulkano::{instance::Instance, pipeline::graphics::viewport::Viewport, swapchain::Surface};
use vulkano_win::VkSurfaceBuild;

use winit::{
  dpi::{LogicalSize, PhysicalSize},
  event_loop::EventLoop,
  window::{Window, WindowBuilder},
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
    event_loop: &EventLoop<()>,
    options: MdrWindowOptions,
  ) -> Arc<Self> {
    // Set up event loop and build window
    let surface = WindowBuilder::new()
      .with_title(options.title)
      .with_inner_size(LogicalSize::new(
        f64::from(options.width),
        f64::from(options.height),
      ))
      .with_resizable(options.resizable)
      .build_vk_surface(event_loop, instance.clone())
      .unwrap();

    Arc::new(Self { surface })
  }

  pub fn create_viewport(&self) -> Viewport {
    Viewport {
      origin: [0.0, 0.0],
      dimensions: self.dimensions().into(),
      depth_range: 0.0..1.0,
    }
  }

  /// Returns the dimensions of the window.
  pub fn dimensions(&self) -> PhysicalSize<u32> {
    return self.get_window().inner_size();
  }

  /// Returns whether or not the window has no visible drawing surface.
  pub fn is_minimized(&self) -> bool {
    let dimensions = self.dimensions();

    dimensions.width == 0 || dimensions.height == 0
  }

  /// Gets a reference to the window which the surface is a part of
  fn get_window(&self) -> &Window {
    // TODO - Any safety improvements here? This feels a bit haphazard with the unwraps and downcast.
    self
      .surface
      .object()
      .unwrap()
      .downcast_ref::<Window>()
      .unwrap()
  }
}
