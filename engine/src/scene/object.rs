use crate::graphics::resources::{MdrMaterial, MdrMesh};

use super::transform::MdrTransform;

pub struct MdrRenderObject {
  pub mesh: MdrMesh,
  pub transform: MdrTransform,
  pub material: MdrMaterial,

  children: Vec<MdrRenderObject>,
}

impl MdrRenderObject {
  pub fn new(mesh: MdrMesh, material: MdrMaterial) -> Self {
    Self {
      mesh,
      transform: MdrTransform::identity(),
      material,
      children: Vec::new(),
    }
  }

  pub fn children(&self) -> impl Iterator<Item = &MdrRenderObject> {
    self.children.iter()
  }

  pub fn children_mut(&mut self) -> impl Iterator<Item = &mut MdrRenderObject> {
    self.children.iter_mut()
  }

  pub fn add_child(&mut self, child: MdrRenderObject) {
    self.children.push(child);
  }
}
