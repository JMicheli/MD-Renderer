use mdr_engine::{
  MdrApplication, MdrEngine, MdrInputState, MdrRunOptions,
  scene::{MdrLight, MdrRenderObject, MdrScene},
};

use mdr_example_utils::{DEBUG_ENABLED, mesh_asset};

mod materials;

// Consts for this example
const LIGHT_BRIGHTNESS: f32 = 60.0;
const CAMERA_MOV_SPEED: f32 = 0.5;
const CAMERA_ROT_SPEED: f32 = 0.01;

fn main() {
  // Set up logging
  mdr_example_utils::initialize_logger().unwrap();

  mdr_engine::run_application(
    BasicExampleApp {},
    MdrRunOptions {
      debug: DEBUG_ENABLED,
    },
  );
}

struct BasicExampleApp;

impl MdrApplication for BasicExampleApp {
  fn initialize(&mut self, engine: &mut MdrEngine) {
    // Create object meshes
    let monkey_mesh = engine
      .manage_resources()
      .load_mesh_obj(&mesh_asset("suzanne.obj"), "monkey")
      .unwrap();
    let sphere_mesh = engine
      .manage_resources()
      .load_mesh_obj(&mesh_asset("sphere.obj"), "sphere")
      .unwrap();
    let cube_mesh = engine
      .manage_resources()
      .load_mesh_obj(&mesh_asset("cube.obj"), "cube")
      .unwrap();
    let plane_mesh = engine
      .manage_resources()
      .load_mesh_obj(&mesh_asset("plane.obj"), "plane")
      .unwrap();

    // Create object materials
    let monkey_mat = materials::white_bricks("monkey_mat", engine);
    let sphere_mat = materials::blue_tile("sphere_mat", engine);
    let plane_mat = materials::metal_plates("plane_mat", engine);
    let cube_mat = materials::wood_planks("cube_mat", engine);

    // Add Suzanne
    let mut monkey = MdrRenderObject::new(monkey_mesh, monkey_mat);
    monkey.transform.translation.set(0.0, 0.0, -2.0);
    engine.scene.add_object("suzanne", monkey);
    // Add sphere
    let mut sphere = MdrRenderObject::new(sphere_mesh, sphere_mat);
    sphere.transform.translation.set(2.0, -2.0, -3.0);
    engine.scene.add_object("sphere", sphere);
    // Add cube
    let mut cube = MdrRenderObject::new(cube_mesh, cube_mat);
    cube.transform.translation.set(-2.0, -2.0, -3.0);
    engine.scene.add_object("cube", cube);
    // Add ground plane
    let mut ground_plane = MdrRenderObject::new(plane_mesh, plane_mat);
    ground_plane.transform.translation.set(0.0, 1.0, 0.0);
    engine.scene.add_object("ground", ground_plane);

    // Add white light
    let mut white_light = MdrLight::white(LIGHT_BRIGHTNESS);
    white_light.translation.set(1.0, 3.0, 3.0);
    engine.scene.lights.add_light(white_light);
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
  }

  fn shutdown(&mut self, _: &mut MdrEngine) {}
}
