use nalgebra::{Matrix4, Rotation3, Scale3, Translation3};

#[derive(Clone, Copy)]
pub struct MdrTransform {
  pub translation: MdrTranslation,
  pub rotation: MdrRotation,
  pub scale: MdrScale,
}

impl MdrTransform {
  pub fn matrix(&self) -> Matrix4<f32> {
    let translation = self.translation.matrix();
    let rotation = self.rotation.matrix();
    let scale = self.scale.matrix();

    translation * rotation * scale
  }

  pub fn inverse_matrix(&self) -> Matrix4<f32> {
    let translation_inv = self.translation.inverse_matrix();
    let rotation_inv = self.rotation.inverse_matrix();
    let scale_inv = self.scale.inverse_matrix();

    scale_inv * rotation_inv * translation_inv
  }

  pub const fn identity() -> Self {
    Self {
      translation: MdrTranslation::identity(),
      rotation: MdrRotation::identity(),
      scale: MdrScale::identity(),
    }
  }
}

/// Represents a translation along the x, y, and z axes.
#[derive(Clone, Copy)]
pub struct MdrTranslation {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl MdrTranslation {
  pub const fn set(&mut self, x: f32, y: f32, z: f32) {
    self.x = x;
    self.y = y;
    self.z = z;
  }

  pub fn translate_by(&mut self, x: f32, y: f32, z: f32) {
    self.x += x;
    self.y += y;
    self.z += z;
  }

  pub const fn identity() -> Self {
    Self {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }

  pub(crate) fn matrix(&self) -> Matrix4<f32> {
    Translation3::new(self.x, self.y, self.z).to_homogeneous()
  }

  pub(crate) fn inverse_matrix(&self) -> Matrix4<f32> {
    Translation3::new(self.x, self.y, self.z)
      .inverse()
      .to_homogeneous()
  }
}

impl From<MdrTranslation> for [f32; 3] {
  fn from(translation: MdrTranslation) -> Self {
    [translation.x, translation.y, translation.z]
  }
}

/// Represents a rotation in **degrees** around the x, y, and z axes.
#[derive(Clone, Copy)]

pub struct MdrRotation {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl MdrRotation {
  pub const fn set(&mut self, x: f32, y: f32, z: f32) {
    self.x = x;
    self.y = y;
    self.z = z;
  }

  pub const fn identity() -> Self {
    Self {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }

  pub(crate) fn matrix(&self) -> Matrix4<f32> {
    Rotation3::from_euler_angles(self.x, self.z, self.y).to_homogeneous()
  }

  pub(crate) fn inverse_matrix(&self) -> Matrix4<f32> {
    Rotation3::from_euler_angles(self.x, self.z, self.y)
      .inverse()
      .to_homogeneous()
  }
}

/// Represents a scale along the x, y, and z axes.
#[derive(Clone, Copy)]

pub struct MdrScale {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl MdrScale {
  pub const fn set(&mut self, x: f32, y: f32, z: f32) {
    self.x = x;
    self.y = y;
    self.z = z;
  }

  pub const fn identity() -> Self {
    Self {
      x: 1.0,
      y: 1.0,
      z: 1.0,
    }
  }

  pub(crate) fn matrix(&self) -> Matrix4<f32> {
    Scale3::new(self.x, self.y, self.z).to_homogeneous()
  }

  pub(crate) fn inverse_matrix(&self) -> Matrix4<f32> {
    Scale3::new(self.x, self.y, self.z)
      .try_inverse()
      .unwrap()
      .to_homogeneous()
  }
}
