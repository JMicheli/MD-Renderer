mod camera;
mod lighting;
mod object;
pub mod transform;

use std::collections::HashMap;

pub use camera::MdrCamera;
pub use lighting::MdrLight;
pub use object::MdrRenderObject;

use self::lighting::MdrLightSet;

pub struct MdrScene {
  pub camera: MdrCamera,
  pub lights: MdrLightSet,
  pub scene_objects: HashMap<String, MdrRenderObject>,
}

impl MdrScene {
  pub(crate) fn new() -> Self {
    Self {
      camera: MdrCamera::default(),
      lights: MdrLightSet::new(),
      scene_objects: HashMap::new(),
    }
  }

  pub fn add_object(&mut self, name: &str, object: MdrRenderObject) {
    self.scene_objects.insert(name.to_string(), object);
  }

  pub fn find_object(&mut self, name: &str) -> Option<&mut MdrRenderObject> {
    self.scene_objects.get_mut(name)
  }
}
