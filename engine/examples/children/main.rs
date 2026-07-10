use mdr_engine::{
  MdrApplication, MdrEngine, MdrInputState, MdrRunOptions,
  resources::{MdrMaterial, MdrMeshMaterialCreateInfo, color::MdrColor},
  scene::{MdrLight, MdrObject, MdrRenderData, MdrScene},
};

use mdr_example_utils::{DEBUG_ENABLED, make_material, mesh_asset};

// Consts for this example
const LIGHT_BRIGHTNESS: f32 = 10.0;
const MOVE_RATE: f32 = 1.0;

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
    let sphere_mesh = engine
      .manage_resources()
      .load_mesh_obj(&mesh_asset("sphere.obj"), "sphere")
      .unwrap();
    let cube_mesh = engine
      .manage_resources()
      .load_mesh_obj(&mesh_asset("cube.obj"), "cube")
      .unwrap();

    // Add spheres
    let red_mat = red_material("red_mat", engine);
    let mut sphere = MdrObject::new(Some(MdrRenderData::new(
      sphere_mesh.clone(),
      red_mat.clone(),
    )));
    sphere.transform.translation.set(2.0, -2.0, -3.0);
    let mut sphere2 = MdrObject::new(Some(MdrRenderData::new(
      sphere_mesh.clone(),
      red_mat.clone(),
    )));
    sphere2.transform.translation.set(2.0, -4.0, -3.0);
    sphere2.transform.scale.set(0.5, 0.5, 0.5);
    let mut sphere3 = MdrObject::new(Some(MdrRenderData::new(sphere_mesh, red_mat)));
    sphere3.transform.translation.set(2.0, -5.0, -3.0);
    sphere3.transform.scale.set(0.25, 0.25, 0.25);
    sphere2.add_child(sphere3);
    sphere.add_child(sphere2);
    engine.scene.add_object("sphere", sphere);

    // Add cubes
    let green_mat = green_material("green_mat", engine);
    let mut cube = MdrObject::new(Some(MdrRenderData::new(
      cube_mesh.clone(),
      green_mat.clone(),
    )));
    cube.transform.translation.set(-2.0, -2.0, -3.0);
    let mut cube2 = MdrObject::new(Some(MdrRenderData::new(
      cube_mesh.clone(),
      green_mat.clone(),
    )));
    cube2.transform.translation.set(-2.0, -3.0, -3.0);
    cube2.transform.scale.set(0.5, 0.5, 0.5);
    let mut cube3 = MdrObject::new(Some(MdrRenderData::new(cube_mesh, green_mat)));
    cube3.transform.translation.set(-2.0, -4.0, -3.0);
    cube3.transform.scale.set(0.4, 0.4, 0.4);
    cube2.add_child(cube3);
    cube.add_child(cube2);
    engine.scene.add_object("cube", cube);

    // Add white light
    let mut white_light = MdrLight::white(LIGHT_BRIGHTNESS);
    white_light.translation.set(1.0, 3.0, 3.0);
    engine.scene.lights.add_light(white_light);

    // Move camera into position
    engine
      .scene
      .camera
      .transform
      .translation
      .set(1.0, 3.0, -10.0);
  }

  fn update(&mut self, scene: &mut MdrScene, input_state: &MdrInputState, dt: f32) {
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

pub fn red_material(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  let base_color = engine
    .manage_resources()
    .create_solid_texture(MdrColor::rgba(0.8, 0.0, 0.0, 1.0), "red_base_color")
    .unwrap();
  let roughness = engine
    .manage_resources()
    .create_solid_texture(MdrColor::rgba(0.3, 0.3, 0.3, 1.0), "red_roughness")
    .unwrap();
  let normal = engine
    .manage_resources()
    .create_solid_texture(MdrColor::rgba(0.0, 0.0, 0.0, 1.0), "red_normal")
    .unwrap();

  make_material(
    engine,
    name,
    &MdrMeshMaterialCreateInfo {
      diffuse: Some(base_color),
      metallic_roughness: Some(roughness),
      normal: Some(normal),
      ..Default::default()
    },
  )
}

pub fn green_material(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  let base_color = engine
    .manage_resources()
    .create_solid_texture(MdrColor::rgba(0.0, 0.6, 0.0, 1.0), "green_base_color")
    .unwrap();
  let roughness = engine
    .manage_resources()
    .create_solid_texture(MdrColor::rgba(0.8, 0.8, 0.8, 1.0), "green_roughness")
    .unwrap();
  let normal = engine
    .manage_resources()
    .create_solid_texture(MdrColor::rgba(0.0, 0.0, 0.0, 1.0), "green_normal")
    .unwrap();

  make_material(
    engine,
    name,
    &MdrMeshMaterialCreateInfo {
      diffuse: Some(base_color),
      metallic_roughness: Some(roughness),
      normal: Some(normal),
      ..Default::default()
    },
  )
}
