use std::sync::Arc;

use vulkano::{
  device::Device,
  pipeline::{
    GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    graphics::{
      GraphicsPipelineCreateInfo,
      color_blend::{ColorBlendAttachmentState, ColorBlendState},
      depth_stencil::{DepthState, DepthStencilState},
      input_assembly::InputAssemblyState,
      multisample::MultisampleState,
      rasterization::{CullMode, FrontFace, RasterizationState},
      vertex_input::{Vertex, VertexDefinition},
      viewport::{Viewport, ViewportState},
    },
    layout::PipelineDescriptorSetLayoutCreateInfo,
  },
  shader::ShaderModule,
};

use crate::graphics::{render_pass::MdrRenderPass, resources::MdrVertex_pos, shaders};

/// The pipeline used for drawing lights.
pub struct MdrLightPipeline {
  logical_device: Arc<Device>,
  pub graphics_pipeline: Arc<GraphicsPipeline>,

  pub vertex_shader: Arc<ShaderModule>,
  pub fragment_shader: Arc<ShaderModule>,
}

#[allow(dead_code)]
impl MdrLightPipeline {
  pub fn new(
    logical_device: &Arc<Device>,
    render_pass: &MdrRenderPass,
    viewport: &Viewport,
  ) -> Self {
    // Load shader modules to GPU
    let (vertex_shader, fragment_shader) = shaders::load_light_shaders(logical_device);

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
    let vertex_shader = vertex_shader.entry_point("main").unwrap();
    let fragment_shader = fragment_shader.entry_point("main").unwrap();
    let vertex_input_state = MdrVertex_pos::per_vertex()
      .definition(&vertex_shader)
      .unwrap();

    let stages = [
      PipelineShaderStageCreateInfo::new(vertex_shader),
      PipelineShaderStageCreateInfo::new(fragment_shader),
    ];

    let layout = PipelineLayout::new(
      logical_device.clone(),
      PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
        .into_pipeline_layout_create_info(logical_device.clone())
        .unwrap(),
    )
    .unwrap();

    let subpass = render_pass.get_subpass();
    GraphicsPipeline::new(
      logical_device.clone(),
      None,
      GraphicsPipelineCreateInfo {
        // Link shader stages
        stages: stages.into_iter().collect(),
        // Inform the pipeline how vertices are layed out
        vertex_input_state: Some(vertex_input_state),
        // Input assembly settings (we use the defaults)
        input_assembly_state: Some(InputAssemblyState::default()),
        // Define the viewport to be used for this render
        // TODO - See if defaults work here, the prior value was from viewport_fixed_scissor_irrelevant (deprecated)
        viewport_state: Some(ViewportState {
          viewports: [viewport.clone()].into(),
          ..Default::default()
        }),
        // Fixed functions of the rasterizer
        rasterization_state: Some(RasterizationState {
          // Clockwise-winding faces will be treated as front-facing
          // The inverted y axis that we use necessitates a change to counter-clockwise
          front_face: FrontFace::CounterClockwise,
          // We cull back-facing faces to avoid unnecessary fragment threads
          cull_mode: CullMode::Back,
          ..Default::default()
        }),
        // Settings for depth testing (to ensure correct ordering of fragments)
        depth_stencil_state: Some(DepthStencilState {
          depth: Some(DepthState::simple()),
          ..Default::default()
        }),
        // TODO - Document
        multisample_state: Some(MultisampleState::default()),
        // TODO - See if this should be changed in someway, it's minimal right now
        color_blend_state: Some(ColorBlendState::with_attachment_states(
          subpass.num_color_attachments(),
          ColorBlendAttachmentState::default(),
        )),
        // The render pass subpass to use for this pipeline
        subpass: Some(subpass.into()),
        ..GraphicsPipelineCreateInfo::layout(layout)
      },
    )
    .unwrap()
  }
}
