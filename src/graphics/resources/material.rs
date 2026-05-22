use std::sync::Arc;

use vulkano::buffer::cpu_pool::CpuBufferPoolChunk;

use super::{color::MdrRgb, MdrGpuTextureHandle, MdrTexture};

pub use crate::graphics::shaders::mesh_fragment_shader::ty::MdrMaterialUniformData;

#[derive(Debug)]
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
  pub(crate) material_data: Arc<CpuBufferPoolChunk<MdrMaterialUniformData>>,
  pub(crate) diffuse_map: MdrGpuTextureHandle,
  pub(crate) roughness_map: MdrGpuTextureHandle,
  pub(crate) normal_map: MdrGpuTextureHandle,
}
