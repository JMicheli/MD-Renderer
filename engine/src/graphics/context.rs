use std::{collections::VecDeque, fmt::Write, sync::Arc, time::Instant};

use nalgebra::{Matrix4, Vector3};
use winit::event_loop::ActiveEventLoop;

use vulkano::{
  Validated, VulkanError, VulkanLibrary,
  buffer::Subbuffer,
  command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, RenderPassBeginInfo,
    SubpassBeginInfo, SubpassContents, SubpassEndInfo,
  },
  descriptor_set::{DescriptorSet, WriteDescriptorSet},
  device::{Device, Queue},
  format::ClearValue,
  image::Image,
  memory::allocator::StandardMemoryAllocator,
  padded::Padded,
  pipeline::{Pipeline, PipelineBindPoint, graphics::viewport::Viewport},
  render_pass::Framebuffer,
  swapchain::{self, Swapchain, SwapchainPresentInfo},
  sync::{self, GpuFuture},
};

use crate::{
  config::{ENABLED_EXTENSIONS, ENABLED_FEATURES, MAX_POINT_LIGHTS},
  graphics::{
    pipeline::{MdrEnginePipelines, MdrMeshPipeline},
    render_pass::MdrRenderPass,
    shaders::mesh_vertex_shader::{MdrPushConstants, MdrSceneData},
    vulkan_constructors::{
      create_framebuffers, create_instance, create_logical_device, create_swapchain,
      pick_physical_device, set_up_frame_futures,
    },
    window::{MdrWindow, MdrWindowOptions},
  },
  scene::{MdrObject, MdrScene},
};

use super::{
  resources::MdrResourceManager,
  shaders::mesh_vertex_shader::{CameraData, PointLightData},
};

/// A Vulkan graphics context, contains [`vulkano`] members.
pub struct MdrGraphicsContext {
  /// The window that allows a user to interact with the application and provides a surface
  /// for displaying rendered frames.
  pub(crate) window: Arc<MdrWindow>,

  /// The Vulkan device responsible for rendering operations.
  logical_device: Arc<Device>,
  /// A command queue in the `logical_device` that executes incoming rendering commands.
  queue: Arc<Queue>,
  /// A structure that contains buffer images used as render targets for rendering and
  /// presentation on the display.
  swapchain: Arc<Swapchain>,
  /// The image buffers in the `swapchain`.
  swapchain_images: Vec<Arc<Image>>,
  /// A render pass describes how to use attach and use image buffers when rendering.
  render_pass: MdrRenderPass, // TODO - Deprecated in Vulkan 1.4
  /// A viewport that serves as a render target.
  viewport: Viewport,
  /// The pipelines used for rendering various objects in a scene.
  pipelines: MdrEnginePipelines,
  /// Image buffers that serve as attachments in the renderpass.
  framebuffers: Vec<Arc<Framebuffer>>,

  /// Owns and organizes resources for use by applications using the engine. Stores meshes,
  /// textures, and materials.
  pub(crate) resource_manager: MdrResourceManager,

  /// Tracks the state of the context across invocations of [`Self::draw`].
  state: ContextState,
}

pub struct ContextState {
  window_was_resized: bool,
  should_recreate_swapchain: bool,
  updated_aspect_ratio: bool,
  frame_futures: Vec<Option<Box<dyn GpuFuture>>>,
  previous_frame_index: usize,

  last_draw_call: Option<Instant>,
  title_decorator: String,
}

impl MdrGraphicsContext {
  /// Create a new MD Renderer Graphics context with optional debug.
  pub fn new(event_loop: &ActiveEventLoop, debug_enabled: bool) -> Self {
    tracing::debug!("Creating graphics context");

    // Get a vulkan library
    let library = VulkanLibrary::new().expect("Failed to acquire vulkan library");
    // Create instance containing Vulkan function pointers
    let instance = create_instance(library, event_loop, debug_enabled);
    tracing::debug!("Created vulkan instance");

    // Create window
    let window_options = MdrWindowOptions {
      width: 800,
      height: 600,
      resizable: true,
      title: "MD Renderer",
    };
    let window = MdrWindow::new(&instance, event_loop, &window_options);
    tracing::debug!("Created window");

    // Select physical device and queue
    let (physical_device, queue_family_index) =
      pick_physical_device(&instance, &ENABLED_EXTENSIONS, &window.surface);

    let device_name = &physical_device.properties().device_name;
    let device_type = physical_device.properties().device_type;
    tracing::info!("Using device: {device_name} (type: {device_type:?})");

    // Create logical device
    let (logical_device, queue) = create_logical_device(
      physical_device.clone(),
      &ENABLED_EXTENSIONS,
      &ENABLED_FEATURES,
      queue_family_index,
    );
    tracing::debug!("Created logical device");

    // Create swapchain
    let (swapchain, swapchain_images) =
      create_swapchain(&window, &logical_device, &physical_device);
    tracing::debug!("Created swapchain");

    // Create render pass
    let render_pass = MdrRenderPass::new(&logical_device, swapchain.image_format());
    tracing::debug!("Created render pass");

    // Create viewport
    let viewport = window.create_viewport();
    tracing::debug!("Created viewport");

    // Create pipelines
    let pipelines = MdrEnginePipelines::new(&logical_device, &render_pass, &viewport);
    tracing::debug!("Created pipelines");

    // Create memory allocator
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(logical_device.clone()));

    // Create framebuffers
    let framebuffers = create_framebuffers(&memory_allocator, &swapchain_images, &render_pass);
    tracing::debug!("Created framebuffers");

    // Create vector of futures corresponding to each swapchain image
    let frame_futures = set_up_frame_futures(swapchain_images.len());

    // Create resource manager
    let resource_manager = MdrResourceManager::new(
      logical_device.clone(),
      memory_allocator,
      pipelines.clone(),
      queue.clone(),
    );

    Self {
      window,
      resource_manager,

      logical_device,
      queue,
      swapchain,
      swapchain_images,
      render_pass,
      viewport,
      pipelines,
      framebuffers,

      state: ContextState {
        window_was_resized: false,
        should_recreate_swapchain: false,
        updated_aspect_ratio: true,
        frame_futures,
        previous_frame_index: 0,

        last_draw_call: None,
        title_decorator: String::with_capacity(64),
      },
    }
  }

  /// Submits a draw command buffer based on the [`MdrScene`] referenced.
  pub fn draw(&mut self, scene: &MdrScene) {
    tracing::trace!("Starting draw");

    // Skip draw for minimized windows
    if self.window.is_minimized() {
      tracing::trace!("Window minimized");
      return;
    }

    // Update window title to show statistics
    self.display_engine_statistics();

    self.size_dependent_updates();

    // First, we acquire the index of the swapchain image to draw to
    let (image_index, is_suboptimal, acquire_future) =
      match swapchain::acquire_next_image(self.swapchain.clone(), None) {
        Ok(r) => r,
        Err(Validated::Error(VulkanError::OutOfDate)) => {
          tracing::debug!("Swapchain out of date, flagging for recreation");
          self.state.should_recreate_swapchain = true;
          return; // No render this frame
        }
        Err(e) => panic!("Failed to acquire next swapchain image: {e}"),
      };

    // The swapchain can sometimes be suboptimal but not out of date
    if is_suboptimal {
      tracing::trace!("Swapchain suboptimal, flagging for recreation");
      // We'll use it but recreate the swapchain on the next loop
      self.state.should_recreate_swapchain = true;
    }

    // Get last frame's end-of-command-execution future (or present moment if no frame waiting)
    let mut previous_frame_end =
      match self.state.frame_futures[self.state.previous_frame_index].take() {
        Some(future) => future,
        None => sync::now(self.logical_device.clone()).boxed(),
      };
    // If we're waiting for any resources to load, chain those in
    if let Some(resource_future) = self.resource_manager.take_upload_futures() {
      previous_frame_end = previous_frame_end.join(resource_future).boxed();
    }
    // Clean up lingering finished futures
    previous_frame_end.cleanup_finished();

    let command_buffer = self.create_command_buffer(
      &self.queue,
      &self.pipelines.mesh,
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
      Err(Validated::Error(VulkanError::OutOfDate)) => {
        self.state.should_recreate_swapchain = true;
        sync::now(self.logical_device.clone()).boxed()
      }
      Err(e) => {
        tracing::error!("Failed to flush future: {e}");
        sync::now(self.logical_device.clone()).boxed()
      }
    };

    // Store future and index for this frame's completion
    self.state.frame_futures[image_index as usize] = Some(end_of_frame_future);
    self.state.previous_frame_index = image_index as usize;
    tracing::trace!("Completed draw");
  }

  fn display_engine_statistics(&mut self) {
    let Some(last_draw_call) = &self.state.last_draw_call else {
      self.state.last_draw_call = Some(Instant::now());
      return;
    };

    let delta_t = (*last_draw_call).elapsed().as_millis();
    let fps = 1000 / delta_t;

    self.state.title_decorator.clear();
    write!(
      &mut self.state.title_decorator,
      "draw time {delta_t} ms | {fps} fps"
    )
    .unwrap();

    self.window.decorate_title(&self.state.title_decorator);
    self.state.last_draw_call = Some(Instant::now());
  }

  /// Performs updates based on the render surface's size.
  fn size_dependent_updates(&mut self) {
    if self.state.window_was_resized || self.state.should_recreate_swapchain {
      self.state.should_recreate_swapchain = false;

      // Recreate swapchain and framebuffers
      tracing::trace!("Recreating swapchain");
      let mut recreate_info = self.swapchain.create_info();
      recreate_info.image_extent = self.window.dimensions().into();
      (self.swapchain, self.swapchain_images) = self.swapchain.recreate(recreate_info).unwrap();
      self.framebuffers = create_framebuffers(
        &self.resource_manager.memory_allocator,
        &self.swapchain_images,
        &self.render_pass,
      );

      if self.state.window_was_resized {
        self.state.window_was_resized = false;

        // Recreate viewport and pipeline
        tracing::trace!("Window resized, recreating pipeline");
        let wd: [f32; 2] = self.window.dimensions().into();
        self.viewport.extent = [wd[0], -wd[1]];
        self.viewport.offset = [0.0, wd[1]];
        self
          .pipelines
          .mesh
          .recreate(&self.render_pass, &self.viewport);

        self.state.updated_aspect_ratio = true;
      }
    }
  }

  /// Returns the aspect ratio of the framebuffer, equal to `width / height`.
  fn aspect_ratio(&self) -> f32 {
    let framebuffer = self.framebuffers[0].clone();
    framebuffer.extent()[0] as f32 / framebuffer.extent()[1] as f32
  }

  /// Set context to trigger size-dependent reinitialization
  pub const fn notify_resized(&mut self) {
    self.state.window_was_resized = true;
  }

  /// Updates a scene's camera's aspect ratio to match the swapchain.
  pub fn update_scene_aspect_ratio(&mut self, scene: &mut MdrScene) {
    if self.state.updated_aspect_ratio {
      scene.camera.aspect_ratio = self.aspect_ratio();
      self.state.updated_aspect_ratio = false;
    }
  }

  /// Generate a command buffer for drawing an [`MdrScene`].
  fn create_command_buffer(
    &self,
    queue: &Arc<Queue>,
    pipeline: &MdrMeshPipeline,
    framebuffer: &Arc<Framebuffer>,
    scene: &MdrScene,
  ) -> Arc<PrimaryAutoCommandBuffer> {
    // Create command buffer builder
    let mut builder = AutoCommandBufferBuilder::primary(
      self.resource_manager.command_buffer_allocator.clone(),
      queue.queue_family_index(),
      CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    // Clear color used when drawing background
    let clear_color_value = ClearValue::Float([0.1, 0.1, 0.1, 1.0]);
    let clear_depth_value = ClearValue::Depth(1.0);
    let mut begin_render_pass_info = RenderPassBeginInfo::framebuffer(framebuffer.clone());
    begin_render_pass_info.clear_values = vec![Some(clear_color_value), Some(clear_depth_value)];

    // Build command buffer
    builder
      // Begin render pass
      .begin_render_pass(
        begin_render_pass_info,
        SubpassBeginInfo {
          contents: SubpassContents::Inline,
          ..Default::default()
        },
      )
      .unwrap();

    // Bind object pipeline
    builder
      .bind_pipeline_graphics(pipeline.graphics_pipeline.clone())
      .unwrap();

    // Upload camera transforms
    let scene_buffer = self.upload_scene_data(scene);
    let scene_descriptor_set = DescriptorSet::new(
      self.resource_manager.descriptor_set_allocator.clone(),
      pipeline
        .graphics_pipeline
        .layout()
        .set_layouts()
        .first()
        .unwrap()
        .clone(),
      [WriteDescriptorSet::buffer(0, scene_buffer)],
      [],
    )
    .unwrap();
    builder
      .bind_descriptor_sets(
        PipelineBindPoint::Graphics,
        pipeline.graphics_pipeline.layout().clone(),
        0,
        scene_descriptor_set,
      )
      .unwrap();

    // Render objects
    self.render_objects(scene, pipeline, &mut builder);

    // End render pass and build
    builder.end_render_pass(SubpassEndInfo::default()).unwrap();
    let command_buffer = builder.build().unwrap();

    tracing::trace!("Created command buffer");
    command_buffer
  }

  /// Uploads data representing a scene's non-object data, i.e., the camera and lights.
  fn upload_scene_data(&self, scene: &MdrScene) -> Subbuffer<MdrSceneData> {
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
      position: Padded(position_vector.into()),
      view: view_matrix.into(),
      proj: projection_matrix.into(),
    };

    // Lighting data
    let point_lights: [PointLightData; MAX_POINT_LIGHTS] =
      scene.lights.get_light_array().map(|light| PointLightData {
        color: Padded(light.color.into()),
        position: light.translation.into(),
        brightness: light.brightness,
      });

    let subbuffer = self
      .resource_manager
      .buffer_allocator
      .allocate_sized()
      .unwrap();
    *subbuffer.write().unwrap() = MdrSceneData {
      camera,
      point_lights,
      point_light_count: scene.lights.get_count(),
    };

    subbuffer
  }

  fn render_objects(
    &self,
    scene: &MdrScene,
    pipeline: &MdrMeshPipeline,
    builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
  ) {
    // TODO - Eliminate allocation here - should yield a significant speed up
    // The render list contains tuples of a reference to an object and its parent transform
    // (if any). This is used to ensure that objects properly inherit their parents'
    // transforms in the next loop. The list initially just contains all root-level
    // objects in the scene.
    let mut render_list: VecDeque<(&MdrObject, Option<Matrix4<f32>>)> = scene
      .scene_objects
      .iter()
      .map(|item| (item.1, None))
      .collect();

    // We walk over the list of render objects and their parent transforms. This is a four stage process:
    // 1. We compute the object transform by applying the parent transform, if any.
    // 2. The children of that current object are pushed onto the end of the render list to be walked
    //    in future iterations of the loop.
    // 3. If the object does not have a mesh and material, we continue to the next iteration.
    // 4. If the object does have render data, we bind it and issue a draw call.
    while let Some((object, parent_transform)) = render_list.pop_front() {
      let object_transform = parent_transform.map_or_else(
        || object.transform.matrix(),
        |pt| object.transform.matrix() * pt,
      );

      render_list.extend(
        object
          .children()
          .map(|child| (child, Some(object_transform))),
      );

      let Some(render_data) = &object.render_data else {
        // This object doesn't have render data, so it's just organizational.
        continue;
      };

      // Get handle to the mesh buffers from the resource manager
      let mesh_handle = self.resource_manager.get_mesh_handle(&render_data.mesh);
      // Get handle to the material buffer from the resource manager
      let material_handle = self
        .resource_manager
        .get_material_handle(&render_data.material);

      // Bind vertex data
      builder
        .bind_vertex_buffers(
          0,
          (
            mesh_handle.positions_buffer.clone(),
            mesh_handle.normals_buffer.clone(),
            mesh_handle.uvs_buffer.clone(),
            mesh_handle.tangents_buffer.clone(),
          ),
        )
        .unwrap()
        .bind_index_buffer(mesh_handle.index_buffer.clone())
        .unwrap();

      builder
        .bind_descriptor_sets(
          PipelineBindPoint::Graphics,
          pipeline.graphics_pipeline.layout().clone(),
          1,
          material_handle.descriptor_set.clone(),
        )
        .unwrap();

      // Upload object's world transform as a push constant
      builder
        .push_constants(
          pipeline.graphics_pipeline.layout().clone(),
          0,
          MdrPushConstants {
            transformation_matrix: object_transform.into(),
          },
        )
        .unwrap();

      // Draw call
      // This call is unsafe because we are responsible for ensuring that valid indicies are provided
      unsafe { builder.draw_indexed(mesh_handle.index_count, 1, 0, 0, 0) }.unwrap();
    }
  }
}
