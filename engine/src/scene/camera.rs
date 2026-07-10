use std::f32::consts::FRAC_PI_2;

use nalgebra::{Matrix4, Perspective3, Vector3, Vector4};

use super::transform::MdrTransform;

// TODO Reimplement as trait and split out ortho/perspective cameras
pub struct MdrCamera {
  pub transform: MdrTransform,

  pub field_of_view: f32,
  pub aspect_ratio: f32,
  pub near_plane: f32,
  pub far_plane: f32,
}

impl MdrCamera {
  pub fn get_view_matrix(&self) -> Matrix4<f32> {
    let translation_matrix = self.transform.translation.matrix();
    let rotation_matrix = self.transform.rotation.matrix();

    rotation_matrix * translation_matrix
  }

  pub fn get_projection_matrix(&self) -> Matrix4<f32> {
    Perspective3::new(
      self.aspect_ratio,
      self.field_of_view,
      self.near_plane,
      self.far_plane,
    )
    .to_homogeneous()
  }

  pub fn get_forward_vector(&self) -> Vector3<f32> {
    let local_forward_vector: Vector4<f32> = Vector4::<f32>::new(0.0, 0.0, 1.0, 1.0);
    let world_forward_vector: Vector4<f32> =
      self.transform.rotation.inverse_matrix() * local_forward_vector;

    Vector3::new(
      world_forward_vector.x,
      world_forward_vector.y,
      world_forward_vector.z,
    )
  }

  pub fn get_sideways_vector(&self) -> Vector3<f32> {
    let local_sideways_vector: Vector4<f32> = Vector4::<f32>::new(1.0, 0.0, 0.0, 1.0);
    let world_sideways_vector: Vector4<f32> =
      self.transform.rotation.inverse_matrix() * local_sideways_vector;

    Vector3::new(
      world_sideways_vector.x,
      world_sideways_vector.y,
      world_sideways_vector.z,
    )
  }
}

impl Default for MdrCamera {
  fn default() -> Self {
    Self {
      transform: MdrTransform::identity(),

      field_of_view: FRAC_PI_2 / 3.0,
      near_plane: 0.01,
      aspect_ratio: 1.0,
      far_plane: 1000.0,
    }
  }
}
