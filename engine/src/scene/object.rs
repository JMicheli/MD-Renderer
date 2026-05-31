use crate::graphics::resources::{MdrMaterial, MdrMesh};

use super::transform::MdrTransform;

pub struct MdrRenderObject {
  pub mesh: MdrMesh,
  pub transform: MdrTransform,
  pub material: MdrMaterial,
}

impl MdrRenderObject {
  pub const fn new(mesh: MdrMesh, material: MdrMaterial) -> Self {
    Self {
      mesh,
      transform: MdrTransform::identity(),
      material,
    }
  }
}
