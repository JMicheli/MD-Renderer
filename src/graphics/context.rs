use log::{debug, error, info, trace};
use nalgebra::Vector3;
use std::sync::Arc;
use winit::event_loop::EventLoop;

use vulkano::{
  buffer::{BufferUsage, CpuAccessibleBuffer},
  command_buffer::{
    allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
    AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, RenderPassBeginInfo,
    SubpassContents,
  },
  descriptor_set::{
    allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
  },
  device::{
    physical::{PhysicalDevice, PhysicalDeviceType},
    Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo,
  },
  format::{ClearValue, Format},
  image::{view::ImageView, AttachmentImage, ImageAccess, ImageUsage, SwapchainImage},
  instance::{Instance, InstanceCreateInfo, InstanceExtensions},
  memory::allocator::{FreeListAllocator, GenericMemoryAllocator, StandardMemoryAllocator},
  pipeline::{graphics::viewport::Viewport, Pipeline, PipelineBindPoint},
  render_pass::{Framebuffer, FramebufferCreateInfo},
  swapchain::{self, AcquireError, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo},
  sync::{self, FlushError, GpuFuture},
  VulkanLibrary,
};

use crate::{
  config::MAX_POINT_LIGHTS,
  graphics::{
    pipeline::MdrMeshPipeline,
    render_pass::MdrRenderPass,
    shaders::mesh_vertex_shader::ty::{MdrPushConstants, MdrSceneData},
    window::{MdrWindow, MdrWindowOptions},
  },
  scene::MdrScene,
};

use super::{
  resources::MdrResourceManager,
  shaders::mesh_vertex_shader::ty::{CameraData, PointLightData},
};

/// A Vulkan graphics context, contains Vulkano members.
pub struct MdrGraphicsContext {
  pub(crate) window: Arc<MdrWindow>,

  logical_device: Arc<Device>,
  queue: Arc<Queue>,
  swapchain: Arc<Swapchain>,
  swapchain_images: Vec<Arc<SwapchainImage>>,
  render_pass: MdrRenderPass,
  viewport: Viewport,
  pipeline: MdrMeshPipeline,
  framebuffers: Vec<Arc<Framebuffer>>,

  pub(crate) resource_manager: MdrResourceManager,
  memory_allocator: Arc<GenericMemoryAllocator<Arc<FreeListAllocator>>>,
  command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
  descriptor_set_allocator: StandardDescriptorSetAllocator,

  window_was_resized: bool,
  should_recreate_swapchain: bool,
  updated_aspect_ratio: bool,
  frame_futures: Vec<Option<Box<dyn GpuFuture>>>,
  previous_frame_index: usize,
}

impl MdrGraphicsContext {
  /// Create a new MD Renderer Graphics context with optional debug.
  pub fn new(event_loop: &EventLoop<()>, debug_enabled: bool) -> Self {
    debug!("Creating graphics context");

    // Get a vulkan library
    let library = VulkanLibrary::new().expect("Failed to acquire vulkan library");
    // Create instance containing Vulkan function pointers
    let instance = Self::create_instance(library, debug_enabled);
    debug!("Created vulkan instance");

    // Create window
    let window_options = MdrWindowOptions {
      width: 800,
      height: 600,
      resizable: true,
      title: "MD Renderer",
    };
    let window = MdrWindow::new(&instance, event_loop, window_options);
    debug!("Created window");

    // Select physical device and queue
    let (physical_device, queue_family_index) =
      Self::pick_physical_device(&instance, window.surface.clone());

    let device_name = &physical_device.properties().device_name;
    let device_type = physical_device.properties().device_type;
    info!("Using device: {device_name} (type: {device_type:?})");

    // Create logical device
    let device_extensions = DeviceExtensions {
      khr_swapchain: true,
      ..DeviceExtensions::empty()
    };
    let (logical_device, queue) = Self::create_logical_device(
      physical_device.clone(),
      device_extensions,
      queue_family_index,
    );
    debug!("Created logical device");

    // Create allocators
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(logical_device.clone()));
    let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
      logical_device.clone(),
      StandardCommandBufferAllocatorCreateInfo {
        primary_buffer_count: 1,
        ..Default::default()
      }),
    );
    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(logical_device.clone());

    // Create swapchain
    let (swapchain, swapchain_images) =
      Self::create_swapchain(&window, &logical_device, &physical_device);
    debug!("Created swapchain");

    // Create render pass
    let render_pass = MdrRenderPass::new(&logical_device, swapchain.image_format());
    debug!("Created render pass");

    // Create viewport
    let viewport = window.create_viewport();
    debug!("Created viewport");

    // Create mesh pipeline
    let pipeline = MdrMeshPipeline::new(&logical_device, &render_pass, &viewport);
    debug!("Created pipeline");

    // Create framebuffers
    let framebuffers =
      Self::create_framebuffers(&memory_allocator, &swapchain_images, &render_pass);
    debug!("Created framebuffers");

    // Create vector of futures corresponding to each swapchain image
    let frame_futures = Self::set_up_frame_futures(swapchain_images.len());

    // Create resource manager
    let resource_manager = MdrResourceManager::new(
      logical_device.clone(),
      memory_allocator.clone(),
      command_buffer_allocator.clone(),
      queue.clone(),
    );

    Self {
      window,

      logical_device,
      queue,
      swapchain,
      swapchain_images,
      render_pass,
      viewport,
      pipeline,
      framebuffers,

      resource_manager,
      memory_allocator,
      command_buffer_allocator,
      descriptor_set_allocator,

      window_was_resized: false,
      should_recreate_swapchain: false,
      updated_aspect_ratio: true,
      frame_futures,
      previous_frame_index: 0,
    }
  }

  /// Submits a draw command buffer based on the `MdrScene` referenced.
  pub fn draw(&mut self, scene: &MdrScene) {
    trace!("Starting draw");

    // Skip draw for minimized windows
    if self.window.is_minimized() {
      trace!("Window minimized");
      return;
    }

    self.size_dependent_updates();

    // First, we acquire the index of the image to draw to
    let (image_index, is_suboptimal, acquire_future) =
      match swapchain::acquire_next_image(self.swapchain.clone(), None) {
        Ok(r) => r,
        Err(AcquireError::OutOfDate) => {
          debug!("Swapchain out of date, flagging for recreation");
          self.should_recreate_swapchain = true;
          return; // No render this frame
        }
        Err(e) => panic!("Failed to acquire next swapchain image: {:?}", e),
      };

    // The swapchain can be suboptimal but not out of date
    if is_suboptimal {
      trace!("Swapchain suboptimal, flagging for recreation");
      // We'll use it but recreate the swapchain on the next loop
      self.should_recreate_swapchain = true;
    }

    // Get last frame's end-of-command future (or present moment if no frame waiting)
    let mut previous_frame_end = match self.frame_futures[self.previous_frame_index].take() {
      Some(future) => future,
      None => sync::now(self.logical_device.clone()).boxed(),
    };
    // If we're waiting for any resources to load, chain that in
    if let Some(resource_future) = self.resource_manager.take_upload_futures() {
      previous_frame_end = previous_frame_end.join(resource_future).boxed();
    }
    // Clean up lingering finished futures
    previous_frame_end.cleanup_finished();

    let command_buffer = self.create_command_buffer(
      &self.queue,
      &self.pipeline,
      &self.framebuffers[image_index as usize],
      scene,
    );

    let future = previous_frame_end
      .join(acquire_future)
      .then_execute(self.queue.clone(), command_buffer)
      .unwrap()
      .then_swapchain_present(
        self.queue.clone(),
        SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_index),
      )
      .then_signal_fence_and_flush();

    let end_of_frame_future = match future {
      Ok(future) => future.boxed(),
      Err(FlushError::OutOfDate) => {
        self.should_recreate_swapchain = true;
        sync::now(self.logical_device.clone()).boxed()
      }
      Err(e) => {
        error!("Failed to flush future: {e}");
        sync::now(self.logical_device.clone()).boxed()
      }
    };

    // Store future and index for this frame's completion
    self.frame_futures[image_index as usize] = Some(end_of_frame_future);
    self.previous_frame_index = image_index as usize;
    trace!("Completed draw")
  }

  /// Performs updates based on the render surface's size.
  fn size_dependent_updates(&mut self) {
    if self.window_was_resized || self.should_recreate_swapchain {
      self.should_recreate_swapchain = false;

      // Recreate swapchain and framebuffers
      trace!("Recreating swapchain");
      let mut recreate_info = self.swapchain.create_info();
      recreate_info.image_extent = self.window.dimensions().into();
      (self.swapchain, self.swapchain_images) = self.swapchain.recreate(recreate_info).unwrap();
      self.framebuffers = Self::create_framebuffers(
        &self.memory_allocator,
        &self.swapchain_images,
        &self.render_pass,
      );

      if self.window_was_resized {
        self.window_was_resized = false;

        // Recreate viewport and pipeline
        trace!("Window resized, recreating pipeline");
        self.viewport.dimensions = self.window.dimensions().into();
        self.pipeline.recreate(&self.render_pass, &self.viewport);

        self.updated_aspect_ratio = true;
      }
    }
  }

  /// Returns the aspect ratio of the framebuffer, equal to `width / height`.
  fn aspect_ratio(&self) -> f32 {
    let framebuffer = self.framebuffers[0].clone();
    framebuffer.extent()[0] as f32 / framebuffer.extent()[1] as f32
  }

  /// Set context to trigger size-dependent reinitialization
  pub fn notify_resized(&mut self) {
    self.window_was_resized = true;
  }

  /// Updates a scene's camera's aspect ratio to match the swapchain.
  pub fn update_scene_aspect_ratio(&mut self, scene: &mut MdrScene) {
    if self.updated_aspect_ratio {
      scene.camera.aspect_ratio = self.aspect_ratio();
      self.updated_aspect_ratio = false;
    }
  }

  /// Generate a command buffer for drawing a `MdrScene`.
  fn create_command_buffer(
    &self,
    queue: &Arc<Queue>,
    pipeline: &MdrMeshPipeline,
    framebuffer: &Arc<Framebuffer>,
    scene: &MdrScene,
  ) -> Arc<PrimaryAutoCommandBuffer> {
    // Create command buffer builder
    let mut builder = AutoCommandBufferBuilder::primary(
      &self.command_buffer_allocator,
      queue.queue_family_index(),
      CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    // Clear color used when drawing bacground
    let clear_color_value = ClearValue::Float([0.1, 0.1, 0.1, 1.0]);
    let clear_depth_value = ClearValue::Depth(1.0);
    let mut begin_render_pass_info = RenderPassBeginInfo::framebuffer(framebuffer.clone());
    begin_render_pass_info.clear_values = vec![Some(clear_color_value), Some(clear_depth_value)];

    // Build command buffer
    // Begin render pass
    builder
      .begin_render_pass(begin_render_pass_info, SubpassContents::Inline)
      .unwrap();

    // Bind object pipeline
    builder.bind_pipeline_graphics(pipeline.graphics_pipeline.clone());

    // Upload camera transforms
    let scene_buffer = Self::upload_scene_data(&self.memory_allocator, scene);
    let scene_descriptor_set = PersistentDescriptorSet::new(
      &self.descriptor_set_allocator,
      pipeline
        .graphics_pipeline
        .layout()
        .set_layouts()
        .get(0)
        .unwrap()
        .clone(),
      [WriteDescriptorSet::buffer(0, scene_buffer)],
    )
    .unwrap();
    builder.bind_descriptor_sets(
      PipelineBindPoint::Graphics,
      pipeline.graphics_pipeline.layout().clone(),
      0,
      scene_descriptor_set,
    );

    // Render objects
    for object in scene.scene_objects.iter() {
      // Get handle to the mesh buffers from the resource manager
      let mesh_handle = self.resource_manager.get_mesh_handle(&object.mesh);
      // Get handle to the material buffer from the resource manager
      let material_handle = self.resource_manager.get_material_handle(&object.material);

      // Upload object's world transform as a push constant
      let push_constants = MdrPushConstants {
        transformation_matrix: object.transform.matrix().into(),
      };

      // Bind vertex data
      builder
        .bind_vertex_buffers(
          0,
          (
            mesh_handle.positions_chunk.clone(),
            mesh_handle.normals_chunk.clone(),
            mesh_handle.uvs_chunk.clone(),
            mesh_handle.tangents_chunk.clone(),
          ),
        )
        .bind_index_buffer(mesh_handle.index_chunk.clone());

      // Upload material data
      // TODO Order by material and bind once per mat
      let material_descriptor_set = PersistentDescriptorSet::new(
        &self.descriptor_set_allocator,
        pipeline
          .graphics_pipeline
          .layout()
          .set_layouts()
          .get(1)
          .unwrap()
          .clone(),
        [
          // Material uniform data
          WriteDescriptorSet::buffer(0, material_handle.material_data.clone()),
          // Diffuse map image sampler
          WriteDescriptorSet::image_view_sampler(
            1,
            material_handle.diffuse_map.image_view.clone(),
            material_handle.diffuse_map.sampler.clone(),
          ),
          // Roughness map image sampler
          WriteDescriptorSet::image_view_sampler(
            2,
            material_handle.roughness_map.image_view.clone(),
            material_handle.roughness_map.sampler.clone(),
          ),
          // Normal map image sampler
          WriteDescriptorSet::image_view_sampler(
            3,
            material_handle.normal_map.image_view.clone(),
            material_handle.normal_map.sampler.clone(),
          ),
        ],
      )
      .unwrap();
      builder.bind_descriptor_sets(
        PipelineBindPoint::Graphics,
        pipeline.graphics_pipeline.layout().clone(),
        1,
        material_descriptor_set.clone(),
      );

      // Push constants for object transform
      builder.push_constants(
        pipeline.graphics_pipeline.layout().clone(),
        0,
        push_constants,
      );

      // Draw call
      builder
        .draw_indexed(mesh_handle.index_count, 1, 0, 0, 0)
        .unwrap();
    }

    // End render pass and build
    builder.end_render_pass().unwrap();
    let command_buffer = Arc::new(builder.build().unwrap());

    trace!("Created command buffer");
    command_buffer
  }

  /// Uploads data representing a scene's non-object data, i.e., the camera and lights.
  fn upload_scene_data(
    memory_allocator: &StandardMemoryAllocator,
    scene: &MdrScene,
  ) -> Arc<CpuAccessibleBuffer<MdrSceneData>> {
    // Camera data
    let view_matrix = scene.camera.get_view_matrix();
    let projection_matrix = scene.camera.get_projection_matrix();

    let view_transform_column = view_matrix.column(3);
    let position_vector = Vector3::new(
      view_transform_column.x,
      view_transform_column.y,
      view_transform_column.z,
    );
    // Camera data object
    let camera = CameraData {
      position: position_vector.into(),
      _dummy0: [0; 4],

      view: view_matrix.into(),
      proj: projection_matrix.into(),
    };

    // Lighting data
    let point_lights: [PointLightData; MAX_POINT_LIGHTS] =
      scene.lights.get_light_array().map(|light| PointLightData {
        color: light.color.into(),
        _dummy0: [0; 4],
        position: light.translation.into(),
        brightness: light.brightness,
      });
    CpuAccessibleBuffer::from_data(
      memory_allocator,
      BufferUsage {
        storage_buffer: true,
        ..BufferUsage::empty()
      },
      false,
      MdrSceneData {
        camera,
        point_lights,
        point_light_count: scene.lights.get_count(),
      },
    )
    .unwrap()
  }

  /// Create a Vulkan instance with optional debug extensions.
  fn create_instance(library: Arc<VulkanLibrary>, debug_enabled: bool) -> Arc<Instance> {
    let required_extensions = {
      let mut extensions = vulkano_win::required_extensions(&library);

      // If debugging is enabled, add the debug utility extension
      if debug_enabled {
        info!("Debug enabled");
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
        debug!("Available debugging layers:");
        let available_layers = library.layer_properties().unwrap();

        let mut available_layers_str = String::new();
        for layer in available_layers {
          let layer_str = format!("\t{}\n", layer.name());
          available_layers_str.push_str(layer_str.as_str())
        }
        available_layers_str.pop();
        debug!("Available layers: \n{}", available_layers_str.as_str());

        // Push validation layer
        output_layers.push("VK_LAYER_KHRONOS_validation".to_owned());
        debug!("Enabled layer: VK_LAYER_KHRONOS_validation")
      }

      output_layers
    };

    match Instance::new(
      library,
      InstanceCreateInfo {
        enabled_extensions: required_extensions,
        enumerate_portability: true, // This bool makes MacOS work
        enabled_layers,
        ..Default::default()
      },
    ) {
      Ok(instance) => instance,
      Err(e) => {
        panic!("Failed to create instance: {e}");
      }
    }
  }

  /// Select a physical device to use. Returns the device and associated queue family index.
  fn pick_physical_device(
    instance: &Arc<Instance>,
    surface: Arc<Surface>,
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
            q.queue_flags.graphics && p.surface_support(i as u32, &surface).unwrap_or(false)
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

    match device_creation_results {
      Some(value) => value,
      None => {
        panic!("Failed to find physical device and queue family.");
      }
    }
  }

  /// Create a Vulkan logical device and queue.
  fn create_logical_device(
    physical_device: Arc<PhysicalDevice>,
    device_extensions: DeviceExtensions,
    queue_family_index: u32,
  ) -> (Arc<Device>, Arc<Queue>) {
    let device_creation_results = Device::new(
      physical_device,
      DeviceCreateInfo {
        enabled_extensions: device_extensions,
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

  fn create_swapchain(
    window: &Arc<MdrWindow>,
    logical_device: &Arc<Device>,
    physical_device: &Arc<PhysicalDevice>,
  ) -> (Arc<Swapchain>, Vec<Arc<SwapchainImage>>) {
    // Retrieve surface capabilities with respect to the physical device
    let surface = &window.surface;
    let surface_capabilities = physical_device
      .surface_capabilities(surface, Default::default())
      .expect("Failed to retrieve surface capabilities");
    // Get other settings
    let dimensions = window.dimensions();
    let vk_image_format = Some(
      physical_device
        .surface_formats(surface, Default::default())
        .unwrap()[0]
        .0,
    );

    let swapchain_result = Swapchain::new(
      logical_device.clone(),
      surface.clone(),
      SwapchainCreateInfo {
        min_image_count: surface_capabilities.min_image_count + 1,
        image_format: vk_image_format,
        image_extent: dimensions.into(),
        image_usage: ImageUsage {
          color_attachment: true,
          ..ImageUsage::empty()
        },
        composite_alpha: surface_capabilities
          .supported_composite_alpha
          .iter()
          .next()
          .unwrap(),
        ..Default::default()
      },
    );

    match swapchain_result {
      Ok(value) => value,
      Err(e) => {
        panic!("Failed to generate swapchain: {}", e);
      }
    }
  }

  fn create_framebuffers(
    memory_allocator: &GenericMemoryAllocator<Arc<FreeListAllocator>>,
    swapchain_images: &Vec<Arc<SwapchainImage>>,
    render_pass: &MdrRenderPass,
  ) -> Vec<Arc<Framebuffer>> {
    let dimensions = swapchain_images[0].dimensions().width_height();
    // Create depth buffer
    let depth_buffer_image =
      AttachmentImage::transient(memory_allocator, dimensions, Format::D16_UNORM).unwrap();
    let depth_buffer_view = ImageView::new_default(depth_buffer_image).unwrap();

    // Create and return framebuffers
    return swapchain_images
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
      .collect::<Vec<_>>();
  }

  /// Sets up a vector of futures corresponding to each framebuffer. These futures will be used to chain
  /// draw commands and ensure that frames are processed in the order the swapchain acquires them.
  fn set_up_frame_futures(frame_count: usize) -> Vec<Option<Box<dyn GpuFuture>>> {
    // Frames in flight setup
    let mut frame_futures: Vec<Option<Box<dyn GpuFuture>>> = Vec::new();
    for _ in 0..frame_count {
      frame_futures.push(None);
    }

    frame_futures
  }
}
