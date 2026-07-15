pub mod mesh_pipeline;

use std::sync::Arc;

pub use mesh_pipeline::MdrMeshPipeline;
use vulkano::{device::Device, pipeline::graphics::viewport::Viewport};

use crate::graphics::render_pass::MdrRenderPass;

#[derive(Clone)]
pub struct MdrEnginePipelines {
  pub mesh: MdrMeshPipeline,
}

impl MdrEnginePipelines {
  pub fn new(
    logical_device: &Arc<Device>,
    render_pass: &MdrRenderPass,
    viewport: &Viewport,
  ) -> Self {
    let mesh = MdrMeshPipeline::new(logical_device, render_pass, viewport);

    Self { mesh }
  }
}
