pub mod light_pipeline;
pub mod mesh_pipeline;

use std::sync::Arc;

#[allow(unused_imports)]
pub use light_pipeline::MdrLightPipeline;
pub use mesh_pipeline::MdrMeshPipeline;
use vulkano::{device::Device, pipeline::graphics::viewport::Viewport};

use crate::graphics::render_pass::MdrRenderPass;

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
