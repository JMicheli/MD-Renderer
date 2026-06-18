use std::sync::Arc;

use vulkano::{
  VulkanLibrary,
  device::{
    Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags,
    physical::{PhysicalDevice, PhysicalDeviceType},
  },
  format::Format,
  image::{Image, ImageCreateInfo, ImageUsage, view::ImageView},
  instance::{Instance, InstanceCreateInfo, InstanceExtensions},
  memory::allocator::{AllocationCreateInfo, FreeListAllocator, GenericMemoryAllocator},
  render_pass::{Framebuffer, FramebufferCreateInfo},
  swapchain::{Surface, Swapchain, SwapchainCreateInfo},
  sync::GpuFuture,
};
use winit::event_loop::ActiveEventLoop;

use crate::graphics::{render_pass::MdrRenderPass, window::MdrWindow};

/// Create a Vulkan instance with optional debug extensions.
pub fn create_instance(
  library: Arc<VulkanLibrary>,
  event_loop: &ActiveEventLoop,
  debug_enabled: bool,
) -> Arc<Instance> {
  let required_extensions = {
    let mut extensions = Surface::required_extensions(event_loop).unwrap();

    // If debugging is enabled, add the debug utility extension
    if debug_enabled {
      tracing::info!("Debug enabled");
      let debug_extensions = InstanceExtensions {
        ext_debug_utils: true,
        ..InstanceExtensions::empty()
      };
      extensions = extensions.union(&debug_extensions);
    }

    extensions
  };

  // Enable layers
  let enabled_layers = {
    let mut output_layers: Vec<String> = vec![];

    // Ignore layers if not in debug mode
    if debug_enabled {
      // Print out available layers
      tracing::debug!("Available debugging layers:");
      let available_layers = library.layer_properties().unwrap();

      let mut available_layers_str = String::new();
      for layer in available_layers {
        let layer_str = format!("\t{}\n", layer.name());
        available_layers_str.push_str(layer_str.as_str());
      }
      available_layers_str.pop();
      tracing::debug!("Available layers: \n{}", available_layers_str.as_str());

      // Push validation layer
      output_layers.push("VK_LAYER_KHRONOS_validation".to_owned());
      tracing::debug!("Enabled layer: VK_LAYER_KHRONOS_validation");
    }

    output_layers
  };

  match Instance::new(
    library,
    InstanceCreateInfo {
      enabled_extensions: required_extensions,
      enabled_layers,
      ..Default::default()
    },
  ) {
    Ok(instance) => instance,
    Err(e) => panic!("Failed to create instance: {e}"),
  }
}

/// Select a physical device to use. Returns the device and associated queue family index.
pub fn pick_physical_device(
  instance: &Arc<Instance>,
  surface: &Arc<Surface>,
) -> (Arc<PhysicalDevice>, u32) {
  let device_extensions = DeviceExtensions {
    khr_swapchain: true,
    ..DeviceExtensions::empty()
  };

  let device_creation_results = instance
    .enumerate_physical_devices()
    .unwrap()
    .filter(|p| p.supported_extensions().contains(&device_extensions))
    .filter_map(|p| {
      p.queue_family_properties()
        .iter()
        .enumerate()
        .position(|(i, q)| {
          q.queue_flags.intersects(QueueFlags::GRAPHICS)
            && p.surface_support(i as u32, surface).unwrap_or(false)
        })
        .map(|i| (p, i as u32))
    })
    .min_by_key(|(p, _)| match p.properties().device_type {
      PhysicalDeviceType::DiscreteGpu => 0,
      PhysicalDeviceType::IntegratedGpu => 1,
      PhysicalDeviceType::VirtualGpu => 2,
      PhysicalDeviceType::Cpu => 3,
      PhysicalDeviceType::Other => 4,
      _ => 5,
    });

  device_creation_results.unwrap_or_else(|| {
    panic!("Failed to find physical device and queue family.");
  })
}

/// Create a Vulkan logical device and queue.
pub fn create_logical_device(
  physical_device: Arc<PhysicalDevice>,
  device_extensions: &DeviceExtensions,
  queue_family_index: u32,
) -> (Arc<Device>, Arc<Queue>) {
  let device_creation_results = Device::new(
    physical_device,
    DeviceCreateInfo {
      enabled_extensions: *device_extensions,
      queue_create_infos: vec![QueueCreateInfo {
        queue_family_index,
        ..Default::default()
      }],
      ..Default::default()
    },
  );

  match device_creation_results {
    Ok(mut value) => {
      let device = value.0;
      let queues = value.1.next().unwrap();
      (device, queues)
    }
    Err(e) => {
      panic!("Failed to create logical device: {e}");
    }
  }
}

pub fn create_swapchain(
  window: &Arc<MdrWindow>,
  logical_device: &Arc<Device>,
  physical_device: &Arc<PhysicalDevice>,
) -> (Arc<Swapchain>, Vec<Arc<Image>>) {
  // Retrieve surface capabilities with respect to the physical device
  let surface = &window.surface;
  let surface_capabilities = physical_device
    .surface_capabilities(surface, Default::default())
    .expect("Failed to retrieve surface capabilities");
  // Get other settings
  let dimensions = window.dimensions();
  let vk_image_format = physical_device
    .surface_formats(surface, Default::default())
    .unwrap()[0]
    .0;

  let swapchain_result = Swapchain::new(
    logical_device.clone(),
    surface.clone(),
    SwapchainCreateInfo {
      min_image_count: surface_capabilities.min_image_count + 1,
      image_format: vk_image_format,
      image_extent: dimensions.into(),
      image_usage: ImageUsage::COLOR_ATTACHMENT,
      composite_alpha: surface_capabilities
        .supported_composite_alpha
        .into_iter()
        .next()
        .unwrap(),
      ..Default::default()
    },
  );

  match swapchain_result {
    Ok(value) => value,
    Err(e) => {
      panic!("Failed to generate swapchain: {e}");
    }
  }
}

pub fn create_framebuffers(
  memory_allocator: &Arc<GenericMemoryAllocator<FreeListAllocator>>,
  swapchain_images: &[Arc<Image>],
  render_pass: &MdrRenderPass,
) -> Vec<Arc<Framebuffer>> {
  let extent = swapchain_images[0].extent();
  // Create depth buffer
  let depth_buffer_image = Image::new(
    memory_allocator.clone(),
    ImageCreateInfo {
      extent,
      format: Format::D16_UNORM,
      usage: ImageUsage::TRANSIENT_ATTACHMENT | ImageUsage::DEPTH_STENCIL_ATTACHMENT,
      ..Default::default()
    },
    AllocationCreateInfo::default(),
  )
  .unwrap();
  let depth_buffer_view = ImageView::new_default(depth_buffer_image).unwrap();

  // Create and return framebuffers
  swapchain_images
    .iter()
    .map(|image| {
      let color_view = ImageView::new_default(image.clone()).unwrap();
      Framebuffer::new(
        render_pass.get_pass(),
        FramebufferCreateInfo {
          // Attach color and depth view
          attachments: vec![color_view, depth_buffer_view.clone()],
          ..Default::default()
        },
      )
      .unwrap()
    })
    .collect::<Vec<_>>()
}

/// Sets up a vector of futures corresponding to each framebuffer. These futures will be used to chain
/// draw commands and ensure that frames are processed in the order the swapchain acquires them.
pub fn set_up_frame_futures(frame_count: usize) -> Vec<Option<Box<dyn GpuFuture>>> {
  // Frames in flight setup
  let mut frame_futures: Vec<Option<Box<dyn GpuFuture>>> = Vec::new();
  for _ in 0..frame_count {
    frame_futures.push(None);
  }

  frame_futures
}
