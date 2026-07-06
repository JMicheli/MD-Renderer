use mdr_engine::{MdrApplication, MdrEngine, MdrInputState, MdrRunOptions, scene::MdrScene};

use mdr_example_utils::{DEBUG_ENABLED, scene_asset};

fn main() {
  // Set up logging
  mdr_example_utils::initialize_logger().unwrap();

  mdr_engine::run_application(
    GltfBoomboxExampleApp {},
    MdrRunOptions {
      debug: DEBUG_ENABLED,
    },
  );
}

struct GltfBoomboxExampleApp;

impl MdrApplication for GltfBoomboxExampleApp {
  fn initialize(&mut self, engine: &mut MdrEngine) {
    engine.load_scene_from_gltf(scene_asset("boombox_with_axes/BoomBoxWithAxes.gltf"));
  }

  fn update(&mut self, _: &mut MdrScene, _: &MdrInputState, _: f32) {}

  fn shutdown(&mut self, _: &mut MdrEngine) {}
}
