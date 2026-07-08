use std::fmt::Display;

use nalgebra::{Matrix4, Translation3, UnitQuaternion, Vector3};

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

  pub fn from_matrix(matrix: [[f32; 4]; 4]) -> Self {
    let m = Matrix4::from_fn(|r, c| matrix[r][c]);

    // 4th column contains trainslation
    let translation = MdrTranslation::new(m[(0, 3)], m[(1, 3)], m[(2, 3)]);

    // Get scale components
    let scale_x = Vector3::new(m[(0, 0)], m[(1, 0)], m[(2, 0)]).norm();
    let scale_y = Vector3::new(m[(0, 1)], m[(1, 1)], m[(2, 1)]).norm();
    let scale_z = Vector3::new(m[(0, 2)], m[(1, 2)], m[(2, 2)]).norm();

    let mut scale = MdrScale::identity();
    scale.set(scale_x, scale_y, scale_z);

    // Get rotation from upper-left 3x3 portion
    let m3x3 = nalgebra::Matrix3::new(
      m[(0, 0)],
      m[(0, 1)],
      m[(0, 2)],
      m[(1, 0)],
      m[(1, 1)],
      m[(1, 2)],
      m[(2, 0)],
      m[(2, 1)],
      m[(2, 2)],
    );
    let rot3 = nalgebra::Rotation3::from_matrix(&m3x3);
    let quat = UnitQuaternion::from(rot3);
    let rotation = MdrRotation(quat);

    Self {
      translation,
      rotation,
      scale,
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
pub struct MdrTranslation(Translation3<f32>);

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

/// Represents a rotation in **degrees** around the x, y, and z axes.
#[derive(Debug, Clone, Copy)]

pub struct MdrRotation(UnitQuaternion<f32>);

impl MdrRotation {
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
