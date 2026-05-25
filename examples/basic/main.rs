use std::env;

use mdr_engine::{
  logger,
  scene::{MdrLight, MdrRenderObject, MdrScene},
  MdrApplication, MdrEngine, MdrInputState, MdrRunOptions,
};

// Some functions and constants extraneous to the example
mod utils;
use utils::{asset, DEBUG_ENABLED, MDR_LOG_LEVEL};
mod materials;

// Consts for this example
const LIGHT_MOV_SPEED: f32 = 1.0;
const LIGHT_BRIGHTNESS: f32 = 0.75;
const CAMERA_MOV_SPEED: f32 = 0.5;
const CAMERA_ROT_SPEED: f32 = 0.01;

fn main() {
  env::set_var("MDR_LOG_LEVEL", MDR_LOG_LEVEL);
  logger::init_from_env().expect("Failed to initialize logger");

  mdr_engine::run_application(
    ExampleApp::new(),
    MdrRunOptions {
      debug: DEBUG_ENABLED,
    },
  );
}

struct ExampleApp;

impl ExampleApp {
  pub fn new() -> Self {
    Self {}
  }
}

impl MdrApplication for ExampleApp {
  fn initialize(&self, engine: &mut MdrEngine) {
    // Create object meshes
    let monkey_mesh = engine
      .manage_resources()
      .load_mesh(asset("meshes/suzanne.obj").as_str(), "monkey")
      .unwrap();
    let sphere_mesh = engine
      .manage_resources()
      .load_mesh(asset("meshes/sphere.obj").as_str(), "sphere")
      .unwrap();
    let cube_mesh = engine
      .manage_resources()
      .load_mesh(asset("meshes/cube.obj").as_str(), "cube")
      .unwrap();
    let plane_mesh = engine
      .manage_resources()
      .load_mesh(asset("meshes/plane.obj").as_str(), "plane")
      .unwrap();

    // Create object materials
    let monkey_mat = materials::white_bricks("monkey_mat", engine);
    let sphere_mat = materials::blue_tile("sphere_mat", engine);
    let plane_mat = materials::metal_plates("plane_mat", engine);
    let cube_mat = materials::wood_planks("cube_mat", engine);

    // Add suzanne
    let mut monkey = MdrRenderObject::new(monkey_mesh, monkey_mat);
    monkey.transform.translation.set(0.0, 0.0, -2.0);
    engine.scene.add_object(monkey);
    // Add sphere
    let mut sphere = MdrRenderObject::new(sphere_mesh, sphere_mat);
    sphere.transform.translation.set(2.0, -2.0, -3.0);
    engine.scene.add_object(sphere);
    // Add cube
    let mut cube = MdrRenderObject::new(cube_mesh, cube_mat);
    cube.transform.translation.set(-2.0, -2.0, -3.0);
    engine.scene.add_object(cube);
    // Add ground plane
    let mut ground_plane = MdrRenderObject::new(plane_mesh, plane_mat);
    ground_plane.transform.translation.set(0.0, 1.0, 0.0);
    engine.scene.add_object(ground_plane);

    // Add white light
    let mut white_light = MdrLight::white(LIGHT_BRIGHTNESS);
    white_light.translation.set(1.0, 3.0, 3.0);
    engine.scene.lights.add_light(white_light);
  }

  fn update(&self, scene: &mut MdrScene, input_state: &MdrInputState, dt: f32) {
    // Camera WASD movement
    if input_state.w {
      let move_magnitude = dt * CAMERA_MOV_SPEED;
      let move_vector = scene.camera.get_forward_vector() * move_magnitude;
      scene
        .camera
        .transform
        .translation
        .translate_by(move_vector.x, 0.0, move_vector.z);
    }
    if input_state.d {
      let move_magnitude = dt * -CAMERA_MOV_SPEED;
      let move_vector = scene.camera.get_sideways_vector() * move_magnitude;
      scene
        .camera
        .transform
        .translation
        .translate_by(move_vector.x, 0.0, move_vector.z);
    }
    if input_state.a {
      let move_magnitude = dt * CAMERA_MOV_SPEED;
      let move_vector = scene.camera.get_sideways_vector() * move_magnitude;
      scene
        .camera
        .transform
        .translation
        .translate_by(move_vector.x, 0.0, move_vector.z);
    }
    if input_state.s {
      let move_magnitude = dt * -CAMERA_MOV_SPEED;
      let move_vector = scene.camera.get_forward_vector() * move_magnitude;
      scene
        .camera
        .transform
        .translation
        .translate_by(move_vector.x, 0.0, move_vector.z);
    }

    // Light movement with arrow keys
    if scene.lights.get_count() > 0 {
      let light = scene.lights.get_light_mut(0).unwrap();

      if input_state.up {
        light.translation.z += dt * LIGHT_MOV_SPEED;
      }
      if input_state.down {
        light.translation.z += dt * -LIGHT_MOV_SPEED;
      }
      if input_state.right {
        light.translation.x += dt * LIGHT_MOV_SPEED;
      }
      if input_state.left {
        light.translation.x += dt * -LIGHT_MOV_SPEED;
      }
    }

    // Camera rotation with mouse when right-button pressed
    if input_state.mouse_right {
      let delta_x = input_state.mouse_delta[0];
      let delta_y = input_state.mouse_delta[1];

      scene.camera.transform.rotation.z += delta_x * CAMERA_ROT_SPEED;
      scene.camera.transform.rotation.x += delta_y * -CAMERA_ROT_SPEED;
    }
  }

  fn shutdown(&self, _: &mut MdrEngine) {}
}
