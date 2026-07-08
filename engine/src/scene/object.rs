use crate::graphics::resources::{MdrMaterial, MdrMesh};

use super::transform::MdrTransform;

pub struct MdrObject {
  pub transform: MdrTransform,
  pub render_data: Option<MdrRenderData>,

  children: Vec<Self>,
}

impl MdrObject {
  pub fn new(render_data: Option<MdrRenderData>) -> Self {
    Self {
      transform: MdrTransform::identity(),
      render_data,
      children: Vec::new(),
    }
  }

  pub fn children(&self) -> impl Iterator<Item = &Self> {
    self.children.iter()
  }

  pub fn children_mut(&mut self) -> impl Iterator<Item = &mut Self> {
    self.children.iter_mut()
  }

  pub fn add_child(&mut self, child: Self) {
    self.children.push(child);
  }
}

pub struct MdrRenderData {
  pub mesh: MdrMesh,
  pub material: MdrMaterial,
}

impl MdrRenderData {
  pub const fn new(mesh: MdrMesh, material: MdrMaterial) -> Self {
    Self { mesh, material }
  }
}
