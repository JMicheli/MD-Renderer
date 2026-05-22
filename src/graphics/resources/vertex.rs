use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

#[repr(C)]
#[derive(BufferContents, Vertex, Default, Copy, Clone)]
pub struct MdrVertex_pos {
  #[format(R32G32B32_SFLOAT)]
  pub a_position: [f32; 3],
}

#[repr(C)]
#[derive(BufferContents, Vertex, Default, Copy, Clone)]
pub struct MdrVertex_norm {
  #[format(R32G32B32_SFLOAT)]
  pub a_normal: [f32; 3],
}

#[repr(C)]
#[derive(BufferContents, Vertex, Default, Copy, Clone)]
pub struct MdrVertex_uv {
  #[format(R32G32_SFLOAT)]
  pub a_uv: [f32; 2],
}

#[repr(C)]
#[derive(BufferContents, Vertex, Default, Copy, Clone)]
pub struct MdrVertex_tan {
  #[format(R32G32B32_SFLOAT)]
  pub a_tangent: [f32; 3],
}
