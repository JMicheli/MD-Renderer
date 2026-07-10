use mdr_engine::{MdrApplication, MdrEngine, MdrInputState, MdrRunOptions, scene::MdrScene};

use mdr_example_utils::{DEBUG_ENABLED, scene_asset};

// Consts for this example
const MOVE_RATE: f32 = 0.1;
const CAMERA_MOV_SPEED: f32 = 0.5;
const CAMERA_ROT_SPEED: f32 = 0.01;

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
    engine
      .scene
      .camera
      .transform
      .translate_by(0.0, -0.007, -0.073);
  }

  fn update(&mut self, scene: &mut MdrScene, input_state: &MdrInputState, dt: f32) {
    // Camera WASD movement
    if input_state.w {
      let move_magnitude = dt * CAMERA_MOV_SPEED;
      let move_vector = scene.camera.get_forward_vector() * move_magnitude;
      scene
        .camera
        .transform
        .translate_by(move_vector.x, 0.0, move_vector.z);
    }
    if input_state.d {
      let move_magnitude = dt * -CAMERA_MOV_SPEED;
      let move_vector = scene.camera.get_sideways_vector() * move_magnitude;
      scene
        .camera
        .transform
        .translate_by(move_vector.x, 0.0, move_vector.z);
    }
    if input_state.a {
      let move_magnitude = dt * CAMERA_MOV_SPEED;
      let move_vector = scene.camera.get_sideways_vector() * move_magnitude;
      scene
        .camera
        .transform
        .translate_by(move_vector.x, 0.0, move_vector.z);
    }
    if input_state.s {
      let move_magnitude = dt * -CAMERA_MOV_SPEED;
      let move_vector = scene.camera.get_forward_vector() * move_magnitude;
      scene
        .camera
        .transform
        .translate_by(move_vector.x, 0.0, move_vector.z);
    }

    // Camera rotation with mouse when right-button pressed
    if input_state.mouse_right {
      let z_angle = input_state.mouse_delta[0] * CAMERA_ROT_SPEED;
      let x_angle = input_state.mouse_delta[1] * -CAMERA_ROT_SPEED;

      scene.camera.transform.rotation.rotate_x(x_angle);
      scene.camera.transform.rotation.rotate_y(z_angle);
    }

    if input_state.up {
      scene
        .camera
        .transform
        .translate_by(0.0, dt * -MOVE_RATE, 0.0);
    }
    if input_state.down {
      scene
        .camera
        .transform
        .translate_by(0.0, dt * MOVE_RATE, 0.0);
    }
  }

  fn shutdown(&mut self, _: &mut MdrEngine) {}
}
