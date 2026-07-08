use mdr_engine::{
  MdrApplication, MdrEngine, MdrInputState, MdrRunOptions,
  resources::{MdrMeshMaterialCreateInfo, color::MdrColor},
  scene::{MdrLight, MdrObject, MdrRenderData, MdrScene},
};

use mdr_example_utils::{DEBUG_ENABLED, mesh_asset};

/// How bright the scene's light will be.
const LIGHT_BRIGHTNESS: f32 = 10.0;
/// Initial offset of the camera from the origin.
const CAMERA_DISTANCE: f32 = 7.75;
/// Speed that the dragon will rotate *in degrees* when input is provided.
const DRAGON_ROTATION_SPEED: f32 = 7.0;

fn main() {
  // Set up logging
  mdr_example_utils::initialize_logger().unwrap();

  mdr_engine::run_application(
    HighPolyExampleApp {},
    MdrRunOptions {
      debug: DEBUG_ENABLED,
    },
  );
}

struct HighPolyExampleApp;

impl MdrApplication for HighPolyExampleApp {
  fn initialize(&mut self, engine: &mut MdrEngine) {
    // Load dragon mesh
    let dragon_mesh = engine
      .manage_resources()
      .load_mesh_obj(&mesh_asset("dragon.obj"), "dragon_mesh")
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
      .create_solid_texture(MdrColor::rgba(0.5, 0.5, 0.5, 1.0), "normal_tex")
      .unwrap();

    // Create dragon material
    let dragon_mat = engine
      .manage_resources()
      .create_material(
        &MdrMeshMaterialCreateInfo {
          diffuse: Some(diffuse),
          metallic_roughness: Some(roughness),
          normal: Some(normal),
          ..Default::default()
        },
        "dragon_mat",
      )
      .unwrap();

    // Add dragon
    let mut dragon = MdrObject::new(Some(MdrRenderData::new(dragon_mesh, dragon_mat)));
    dragon.transform.scale.set(12.0, 12.0, 12.0);
    engine.scene.add_object("dragon", dragon);
    // Add light
    let mut white_light = MdrLight::white(LIGHT_BRIGHTNESS);
    white_light.translation.set(1.0, 13.0, 3.0);
    engine.scene.lights.add_light(white_light);

    engine
      .scene
      .camera
      .transform
      .translate_by(0.0, 0.0, -CAMERA_DISTANCE);
  }

  fn update(&mut self, scene: &mut MdrScene, input: &MdrInputState, dt: f32) {
    let Some(dragon) = scene.find_object("dragon") else {
      return;
    };

    // Incorporate wasd/arrow-key rotation
    let rot_amount = dt * DRAGON_ROTATION_SPEED;
    if input.a || input.left {
      dragon.transform.rotation.rotate_z(-rot_amount);
    }
    if input.w || input.up {
      dragon.transform.rotation.rotate_x(-rot_amount);
    }
    if input.d || input.right {
      dragon.transform.rotation.rotate_z(rot_amount);
    }
    if input.s || input.down {
      dragon.transform.rotation.rotate_x(rot_amount);
    }
  }

  fn shutdown(&mut self, _: &mut MdrEngine) {}
}
