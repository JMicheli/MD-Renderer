use mdr_engine::{MdrApplication, MdrEngine, MdrInputState, MdrRunOptions, scene::MdrScene};

use mdr_example_utils::{DEBUG_ENABLED, scene_asset};

// Consts for this example
const SCALE_RATE: f32 = 1.10;
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

    // Scaling of root objects
    let scale_delta = if input_state.up {
      dt * SCALE_RATE
    } else if input_state.down {
      dt * -SCALE_RATE
    } else {
      return;
    };

    for (_, root_obj) in scene.scene_objects.iter_mut() {
      let old_x = root_obj.transform.scale.0.x;
      let old_y = root_obj.transform.scale.0.y;
      let old_z = root_obj.transform.scale.0.z;

      root_obj.transform.scale.set(
        old_x + old_x * scale_delta,
        old_y + old_y * scale_delta,
        old_z + old_z * scale_delta,
      );
    }
  }

  fn shutdown(&mut self, _: &mut MdrEngine) {}
}
