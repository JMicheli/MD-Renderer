use std::sync::Arc;

use vulkano::{Validated, VulkanError, device::Device, shader::ShaderModule};

pub mod mesh_vertex_shader {
  vulkano_shaders::shader! {
    ty: "vertex",
    path: "src/graphics/shaders/mesh.vert",
  }
}

pub mod mesh_fragment_shader {
  vulkano_shaders::shader! {
    ty: "fragment",
    path: "src/graphics/shaders/mesh.frag",
  }
}

pub fn load_mesh_shaders(logical_device: &Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
  // Vertex shader
  let vs = validate_load_result(mesh_vertex_shader::load(logical_device.clone()));
  // Fragment shader
  let fs = validate_load_result(mesh_fragment_shader::load(logical_device.clone()));

  (vs, fs)
}

pub mod light_vertex_shader {
  vulkano_shaders::shader! {
    ty: "vertex",
    path: "src/graphics/shaders/light.vert",
  }
}

pub mod light_fragment_shader {
  vulkano_shaders::shader! {
    ty: "fragment",
    path: "src/graphics/shaders/light.frag",
  }
}

#[allow(dead_code)]
pub fn load_light_shaders(logical_device: &Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
  // Vertex shader
  let vs = validate_load_result(light_vertex_shader::load(logical_device.clone()));
  // Fragment shader
  let fs = validate_load_result(light_fragment_shader::load(logical_device.clone()));

  (vs, fs)
}

fn validate_load_result(
  output: Result<Arc<ShaderModule>, Validated<VulkanError>>,
) -> Arc<ShaderModule> {
  match output {
    Ok(value) => value,
    Err(e) => {
      panic!("Failed to load shader module: {e}");
    }
  }
}
