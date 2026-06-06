use crate::{config::MAX_POINT_LIGHTS, resources::MdrRgb, scene::transform::MdrTranslation};

#[derive(Clone, Copy)]
pub struct MdrLight {
  pub color: MdrRgb,
  pub brightness: f32,

  pub translation: MdrTranslation,
}

impl MdrLight {
  pub fn new(r: f32, g: f32, b: f32, brightness: f32) -> Self {
    Self {
      color: MdrRgb { r, g, b },
      brightness,

      translation: MdrTranslation::identity(),
    }
  }

  pub fn white(brightness: f32) -> Self {
    Self::new(1.0, 1.0, 1.0, brightness)
  }

  pub fn unused() -> Self {
    Self {
      color: MdrRgb {
        r: 0.0,
        g: 0.0,
        b: 0.0,
      },
      brightness: 0.0,
      translation: MdrTranslation::identity(),
    }
  }
}

/// A set of lights in a scene. Currently only supports point lights.
/// Up to `MAX_POINT_LIGHTS` can be added to a scene.
pub struct MdrLightSet {
  lights: Vec<MdrLight>,
  light_count: usize,
}

impl MdrLightSet {
  /// Create an empty light set.
  pub fn new() -> Self {
    Self::default()
  }

  /// Add a light to the scene's light set. Will panic if `MAX_POINT_LIGHTS` are already present.
  pub fn add_light(&mut self, light: MdrLight) {
    assert!(
      self.light_count != MAX_POINT_LIGHTS,
      "You added more than {MAX_POINT_LIGHTS} lights and now everything broke, be careful.",
    );

    self.lights.push(light);
    self.light_count += 1;
  }

  /// Remove a light from the light set.
  pub fn remove_light(&mut self, light_index: usize) {
    assert!(light_index < MAX_POINT_LIGHTS);

    self.lights.remove(light_index);
    self.light_count -= 1;
  }

  /// Get a reference to a particular light by index. Returns `None` if no light exists at that index.
  pub fn get_light(&self, light_index: usize) -> Option<&MdrLight> {
    self.lights.get(light_index)
  }

  /// Get a mutable reference to a particular light by index. Returns `None` if no light exists at that index.
  pub fn get_light_mut(&mut self, light_index: usize) -> Option<&mut MdrLight> {
    self.lights.get_mut(light_index)
  }

  /// Returns an array containing a copy of the light set's data.
  pub fn get_light_array(&self) -> [MdrLight; MAX_POINT_LIGHTS] {
    let mut light_array = [MdrLight::unused(); MAX_POINT_LIGHTS];
    light_array[..self.light_count].copy_from_slice(&self.lights[..self.light_count]);

    light_array
  }

  /// Get the number of lights in the set.
  pub const fn get_count(&self) -> u32 {
    self.light_count as u32
  }
}

impl Default for MdrLightSet {
  fn default() -> Self {
    Self {
      lights: Vec::<MdrLight>::with_capacity(MAX_POINT_LIGHTS),
      light_count: 0,
    }
  }
}
