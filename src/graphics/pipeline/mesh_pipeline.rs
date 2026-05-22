use std::sync::Arc;

use vulkano::{
  device::Device,
  pipeline::{
    graphics::{
      depth_stencil::DepthStencilState,
      input_assembly::InputAssemblyState,
      rasterization::{CullMode, FrontFace, RasterizationState},
      vertex_input::Vertex,
      viewport::{Viewport, ViewportState},
    },
    GraphicsPipeline,
  },
  shader::ShaderModule,
};

use crate::{
  graphics::{
    render_pass::MdrRenderPass,
    resources::{MdrVertex_norm, MdrVertex_pos, MdrVertex_uv},
    shaders,
  },
  resources::vertex::MdrVertex_tan,
};

/// The pipeline used for mesh drawing.
pub struct MdrMeshPipeline {
  logical_device: Arc<Device>,
  pub graphics_pipeline: Arc<GraphicsPipeline>,

  pub vertex_shader: Arc<ShaderModule>,
  pub fragment_shader: Arc<ShaderModule>,
}

impl MdrMeshPipeline {
  pub fn new(
    logical_device: &Arc<Device>,
    render_pass: &MdrRenderPass,
    viewport: &Viewport,
  ) -> Self {
    // Load shader modules to GPU
    let (vertex_shader, fragment_shader) = shaders::load_mesh_shaders(logical_device);

    Self {
      logical_device: logical_device.clone(),

      graphics_pipeline: Self::create_graphics_pipeline(
        logical_device,
        render_pass,
        &vertex_shader,
        &fragment_shader,
        viewport,
      ),
      vertex_shader,
      fragment_shader,
    }
  }

  pub fn recreate(&mut self, render_pass: &MdrRenderPass, viewport: &Viewport) {
    self.graphics_pipeline = Self::create_graphics_pipeline(
      &self.logical_device,
      render_pass,
      &self.vertex_shader,
      &self.fragment_shader,
      viewport,
    );
  }

  fn create_graphics_pipeline(
    logical_device: &Arc<Device>,
    render_pass: &MdrRenderPass,
    vertex_shader: &Arc<ShaderModule>,
    fragment_shader: &Arc<ShaderModule>,
    viewport: &Viewport,
  ) -> Arc<GraphicsPipeline> {
    GraphicsPipeline::start()
      // Define what vertex structure the pipeline will expect
      .vertex_input_state([
        // Position data, looks like [f32; 3] in memory
        MdrVertex_pos::per_vertex(),
        // // Normal data, looks like [f32; 3] in memory
        MdrVertex_norm::per_vertex(),
        // // UV data, looks like [f32; 2] in memory
        MdrVertex_uv::per_vertex(),
        // // Tangent space basis data, looks like [f32; 3] in memory
        MdrVertex_tan::per_vertex(),
      ])
      // Link the vertex shader
      .vertex_shader(vertex_shader.entry_point("main").unwrap(), ())
      // Input assembly settings (we use the defaults)
      .input_assembly_state(InputAssemblyState::new())
      // Define the viewport to be used for this render
      .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([
        viewport.clone()
      ]))
      // Fixed functions of the rasterizer
      .rasterization_state(
        RasterizationState::new()
          // Clockwise-winding faces will be treated as front-facing
          .front_face(FrontFace::Clockwise)
          // We cull back-facing faces to avoid unnecessary fragment threads
          .cull_mode(CullMode::Back),
      )
      // Link the fragment shader
      .fragment_shader(fragment_shader.entry_point("main").unwrap(), ())
      // Settings for depth testing (to ensure correct ordering of fragments)
      .depth_stencil_state(DepthStencilState::simple_depth_test())
      // The render pass to use for this pipeline
      .render_pass(render_pass.get_subpass())
      // Build and unwrap to get the pipeline object
      .build(logical_device.clone())
      .unwrap()
  }
}
