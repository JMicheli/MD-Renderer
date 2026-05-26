use mdr_engine::{
  MdrApplication, MdrEngine, MdrInputState, MdrRunOptions,
  resources::{MdrMaterialCreateInfo, MdrRgb, color::MdrColor},
  scene::{MdrLight, MdrRenderObject, MdrScene},
};

use utils::{DEBUG_ENABLED, LOG_LEVEL, asset};

mod utils;

// Consts for this example
const LIGHT_BRIGHTNESS: f32 = 0.75;

fn main() {
  // Set up logging
  let subscriber = tracing_subscriber::fmt()
    .with_max_level(LOG_LEVEL)
    .without_time()
    .compact()
    .finish();
  tracing::subscriber::set_global_default(subscriber).unwrap();

  mdr_engine::run_application(
    HighPolyExampleApp {},
    MdrRunOptions {
      debug: DEBUG_ENABLED,
    },
  );
}

struct HighPolyExampleApp;

impl MdrApplication for HighPolyExampleApp {
  fn initialize(&self, engine: &mut MdrEngine) {
    // Load dragon mesh
    let dragon_mesh = engine
      .manage_resources()
      .load_mesh(&asset("dragon.obj"), "dragon")
      .unwrap();

    // Create textures for material
    let diffuse = engine
      .manage_resources()
      .create_solid_texture(MdrColor::rgba(0.608, 0.067, 0.118, 1.0), "diffuse_tex")
      .unwrap();
    let roughness = engine
      .manage_resources()
      .create_solid_texture(MdrColor::rgba(0.6, 0.6, 0.6, 1.0), "roughness_tex")
      .unwrap();
    let normal = engine
      .manage_resources()
      .create_solid_texture(MdrColor::rgba(0.0, 0.0, 0.0, 1.0), "normal_tex")
      .unwrap();

    // Create dragon material
    let dragon_mat = engine
      .manage_resources()
      .create_material(
        &MdrMaterialCreateInfo {
          diffuse,
          roughness,
          normal,
          specular_color: MdrRgb::white(),
          shininess: 30.0,
        },
        "dragon_mat",
      )
      .unwrap();

    // Add dragon
    let mut dragon = MdrRenderObject::new(dragon_mesh, dragon_mat);
    dragon.transform.rotation.set(0.0, 22.0, 30.0);
    engine.scene.add_object(dragon);
    // Add light
    let mut white_light = MdrLight::white(LIGHT_BRIGHTNESS);
    white_light.translation.set(1.0, 13.0, 3.0);
    engine.scene.lights.add_light(white_light);

    engine
      .scene
      .camera
      .transform
      .translation
      .translate_by(0.0, 0.0, -0.75);
  }

  fn update(&self, _: &mut MdrScene, _: &MdrInputState, _: f32) {}

  fn shutdown(&self, _: &mut MdrEngine) {}
}
