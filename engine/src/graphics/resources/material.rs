use std::sync::Arc;

use vulkano::descriptor_set::DescriptorSet;

pub use crate::graphics::shaders::mesh_fragment_shader::MdrMaterialUniformData;
use crate::resources::{MdrRgb, MdrTexture};

#[derive(Debug, Clone)]
pub struct MdrMaterial {
  pub name: String,
}

pub struct MdrMaterialCreateInfo {
  pub diffuse: MdrTexture,
  pub roughness: MdrTexture,
  pub normal: MdrTexture,

  pub specular_color: MdrRgb,
  pub shininess: f32,
}

#[derive(Clone)]
pub struct MdrGpuMaterialHandle {
  pub(crate) descriptor_set: Arc<DescriptorSet>,
}
