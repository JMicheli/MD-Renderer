use std::sync::Arc;

use vulkano::{Validated, VulkanError, device::Device, shader::ShaderModule};

pub mod mesh_vertex_shader {
  vulkano_shaders::shader! {
    ty: "vertex",
    include: ["src/graphics/shaders/include"],
    path: "src/graphics/shaders/mesh.vert",
  }
}

pub mod mesh_fragment_shader {
  vulkano_shaders::shader! {
    ty: "fragment",
    include: ["src/graphics/shaders/include"],
    path: "src/graphics/shaders/mesh.frag",
  }
}

impl std::fmt::Debug for mesh_fragment_shader::MdrMeshMaterialData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("MdrMeshMaterialData")
      .field("base_color_factor", &self.base_color_factor)
      .field("roughness_factor", &self.roughness_factor)
      .field("metallic_factor", &self.metallic_factor)
      .field("diffuse_texture_set", &self.diffuse_texture_set)
      .field(
        "metallic_roughness_texture_set",
        &self.metallic_roughness_texture_set,
      )
      .field("normal_texture_set", &self.normal_texture_set)
      .field("occlusion_texture_set", &self.occlusion_texture_set)
      .field("emissive_texture_set", &self.emissive_texture_set)
      .finish()
  }
}

pub fn load_mesh_shaders(logical_device: &Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
  // Vertex shader
  let vs = validate_load_result(mesh_vertex_shader::load(logical_device.clone()));
  // Fragment shader
  let fs = validate_load_result(mesh_fragment_shader::load(logical_device.clone()));

  (vs, fs)
}

fn validate_load_result(
  output: Result<Arc<ShaderModule>, Validated<VulkanError>>,
) -> Arc<ShaderModule> {
  match output {
    Ok(value) => value,
    Err(e) => match e {
      Validated::Error(e) => panic!("Failed to load shader module due to Vulkan error: {e}"),
      Validated::ValidationError(e) => {
        panic!("Failed to load shader module due to validation error: {e}")
      }
    },
  }
}
