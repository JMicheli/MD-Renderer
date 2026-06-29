use std::sync::Arc;

use vulkano::descriptor_set::DescriptorSet;

pub use crate::graphics::shaders::mesh_fragment_shader::MdrMeshMaterialData;
use crate::resources::{MdrRgba, MdrTexture};

#[derive(Debug, Clone)]
pub struct MdrMaterial {
  pub name: String,
}

pub struct MdrMeshMaterialCreateInfo {
  // Base color tint/multiplier
  pub base_color: MdrRgba,
  // Surface roughness multiplier
  pub base_roughness: f32,
  // Metallic property multiplier
  pub base_metallic: f32,

  // Texture binding index for diffuse color
  pub diffuse: Option<MdrTexture>,
  // Texture binding index for metallic/roughness map (standard
  // is to put metallic in the blue channel and roughness in the
  // green channel).
  pub metallic_roughness: Option<MdrTexture>,
  // Texture binding index for normal maps
  pub normal: Option<MdrTexture>,
  // Texture binding index for ambient occlusion
  pub occlusion: Option<MdrTexture>,
  // Texture binding index for emissive maps
  pub emissive: Option<MdrTexture>,
}

impl Default for MdrMeshMaterialCreateInfo {
  fn default() -> Self {
    Self {
      base_color: MdrRgba::white(),
      base_roughness: 1.0,
      base_metallic: 1.0,
      diffuse: None,
      metallic_roughness: None,
      normal: None,
      occlusion: None,
      emissive: None,
    }
  }
}

#[derive(Clone)]
pub struct MdrGpuMaterialHandle {
  pub(crate) descriptor_set: Arc<DescriptorSet>,
}
