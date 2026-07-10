use std::fmt::Display;

use nalgebra::{Matrix4, Quaternion, Translation3, UnitQuaternion, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct MdrTransform {
  pub translation: MdrTranslation,
  pub rotation: MdrRotation,
  pub scale: MdrScale,
}

impl MdrTransform {
  pub fn identity() -> Self {
    Self {
      translation: MdrTranslation::identity(),
      rotation: MdrRotation::identity(),
      scale: MdrScale::identity(),
    }
  }

  pub fn matrix(&self) -> Matrix4<f32> {
    let translation = self.translation.matrix();
    let rotation = self.rotation.matrix();
    let scale = self.scale.matrix();

    translation * rotation * scale
  }

  pub fn inverse_matrix(&self) -> Matrix4<f32> {
    let inverse_scale = self.scale.inverse_matrix();
    let inverse_rotation = self.rotation.inverse_matrix();
    let inverse_translation = self.translation.inverse_matrix();

    inverse_scale * inverse_rotation * inverse_translation
  }

  pub fn translate_by(&mut self, x: f32, y: f32, z: f32) {
    self.translation += MdrTranslation::new(x, y, z);
  }
}

/// Represents a translation along the x, y, and z axes.
#[derive(Debug, Clone, Copy)]
pub struct MdrTranslation(pub Translation3<f32>);

impl MdrTranslation {
  pub fn identity() -> Self {
    Self(Translation3::identity())
  }

  pub const fn new(x: f32, y: f32, z: f32) -> Self {
    Self(Translation3::new(x, y, z))
  }

  pub fn set(&mut self, x: f32, y: f32, z: f32) {
    self.0.x = x;
    self.0.y = y;
    self.0.z = z;
  }

  pub(crate) fn matrix(&self) -> Matrix4<f32> {
    self.0.to_homogeneous()
  }

  pub(crate) fn inverse_matrix(&self) -> Matrix4<f32> {
    self.0.inverse().to_homogeneous()
  }
}

// TODO - Consider removing
impl From<MdrTranslation> for [f32; 3] {
  fn from(translation: MdrTranslation) -> Self {
    [translation.0.x, translation.0.y, translation.0.z]
  }
}

impl Display for MdrTranslation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl std::ops::Add for MdrTranslation {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self(Translation3::new(
      self.0.x + rhs.0.x,
      self.0.y + rhs.0.y,
      self.0.z + rhs.0.z,
    ))
  }
}

impl std::ops::AddAssign for MdrTranslation {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

/// Represents a rotation around the x, y, and z axes as a quaternion.
#[derive(Debug, Clone, Copy)]

pub struct MdrRotation(pub UnitQuaternion<f32>);

impl MdrRotation {
  /// Create a new [`MdrRotation`] from quaternion components.
  pub fn from_quaternion(w: f32, i: f32, j: f32, k: f32) -> Self {
    Self(UnitQuaternion::new_normalize(Quaternion::new(w, i, j, k)))
  }

  /// Set the rotation to correspond to the result of three rotations around the x, y, and z axes
  /// (applied in that order).
  pub fn set(&mut self, x: f32, y: f32, z: f32) {
    let x_rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), x);
    let y_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), y);
    let z_rot = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), z);

    *self = Self(UnitQuaternion::identity() * x_rot * y_rot * z_rot);
  }

  pub fn identity() -> Self {
    Self(UnitQuaternion::identity())
  }

  pub(crate) fn matrix(&self) -> Matrix4<f32> {
    self.0.to_homogeneous()
  }

  pub(crate) fn inverse_matrix(&self) -> Matrix4<f32> {
    self.0.inverse().to_homogeneous()
  }

  pub fn rotate_x(&mut self, angle: f32) {
    self.0 *= UnitQuaternion::from_axis_angle(&Vector3::x_axis(), angle);
  }

  pub fn rotate_y(&mut self, angle: f32) {
    self.0 *= UnitQuaternion::from_axis_angle(&Vector3::y_axis(), angle);
  }

  pub fn rotate_z(&mut self, angle: f32) {
    self.0 *= UnitQuaternion::from_axis_angle(&Vector3::z_axis(), angle);
  }
}

impl Display for MdrRotation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

/// Represents a scale along the x, y, and z axes.
#[derive(Debug, Clone, Copy)]

pub struct MdrScale(pub Vector3<f32>);

impl MdrScale {
  pub const fn new(x: f32, y: f32, z: f32) -> Self {
    Self(Vector3::new(x, y, z))
  }

  pub fn set(&mut self, x: f32, y: f32, z: f32) {
    self.0.x = x;
    self.0.y = y;
    self.0.z = z;
  }

  pub const fn identity() -> Self {
    Self(Vector3::new(1.0, 1.0, 1.0))
  }

  pub(crate) fn matrix(&self) -> Matrix4<f32> {
    Matrix4::new_nonuniform_scaling(&self.0)
  }

  pub(crate) fn inverse_matrix(&self) -> Matrix4<f32> {
    Matrix4::new_nonuniform_scaling(&Vector3::new(
      1.0 / self.0.x,
      1.0 / self.0.y,
      1.0 / self.0.z,
    ))
  }
}

impl Display for MdrScale {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
