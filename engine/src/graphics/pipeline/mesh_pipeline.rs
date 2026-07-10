use std::sync::Arc;

use vulkano::{
  descriptor_set::layout::{DescriptorBindingFlags, DescriptorSetLayout},
  device::Device,
  pipeline::{
    GraphicsPipeline, Pipeline, PipelineLayout, PipelineShaderStageCreateInfo,
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

use crate::{
  config::MAX_MESH_FRAG_TEXTURES,
  graphics::{
    render_pass::MdrRenderPass,
    resources::{MdrVertex_norm, MdrVertex_pos, MdrVertex_uv},
    shaders,
  },
  resources::vertex::MdrVertex_tan,
};

/// The pipeline used for mesh drawing.
#[derive(Clone)]
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

      graphics_pipeline: create_graphics_pipeline(
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
    self.graphics_pipeline = create_graphics_pipeline(
      &self.logical_device,
      render_pass,
      &self.vertex_shader,
      &self.fragment_shader,
      viewport,
    );
  }

  pub fn descriptor_set_layout(&self) -> Arc<DescriptorSetLayout> {
    self
      .graphics_pipeline
      .layout()
      .set_layouts()
      .get(1)
      .unwrap()
      .clone()
  }
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
  let vertex_input_state = [
    // Position data, looks like [f32; 3] in memory
    MdrVertex_pos::per_vertex(),
    // Normal data, looks like [f32; 3] in memory
    MdrVertex_norm::per_vertex(),
    // UV data, looks like [f32; 2] in memory
    MdrVertex_uv::per_vertex(),
    // Tangent space basis data, looks like [f32; 3] in memory
    MdrVertex_tan::per_vertex(),
  ]
  .definition(&vertex_shader)
  .unwrap();

  let stages = [
    PipelineShaderStageCreateInfo::new(vertex_shader),
    PipelineShaderStageCreateInfo::new(fragment_shader),
  ];

  // The use of a dynamic array in the fragment shader makes the automatic layout
  // generation partially inaccurate. We will manually create the part of the layout
  // that pertains to the texture array.
  let layout = {
    let mut layout_create_info = PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages);

    // Adjust the info for set 1, binding 1 to make it variable
    // This corresponds to the `layout(set = 1, binding = 1) uniform sampler2D textures[];` glsl
    // in mesh.frag
    let binding = layout_create_info.set_layouts[1]
      .bindings
      .get_mut(&1)
      .unwrap();
    binding.binding_flags |= DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT;
    binding.descriptor_count = MAX_MESH_FRAG_TEXTURES; // Maximum number of textures

    PipelineLayout::new(
      logical_device.clone(),
      layout_create_info
        .into_pipeline_layout_create_info(logical_device.clone())
        .unwrap(),
    )
    .unwrap()
  };

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
      // Settings for multisampling
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
