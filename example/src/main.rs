mod logger;

use cgmath::Vector3;
use log::info;

use mdr_engine::{MdrEngine, MdrEngineOptions, MdrMaterial, MdrSceneObject};

fn main() {
  logger::init().expect("Failed to initialize logger");

  let opts = MdrEngineOptions { debug: false };
  let (mut engine, event_loop) = MdrEngine::new(opts);

  // Suzanne
  let mut monkey = MdrSceneObject::from_obj("example/src/assets/suzanne.obj");
  monkey.material = MdrMaterial::red();
  engine.scene.add_object(monkey);
  // Sphere
  let mut sphere = MdrSceneObject::from_obj("example/src/assets/sphere.obj");
  sphere.transform.position = Vector3::new(2.0, -2.0, -1.0);
  sphere.material = MdrMaterial::green();
  engine.scene.add_object(sphere);
  // Ground plane
  let mut ground_plane = MdrSceneObject::from_obj("example/src/assets/plane.obj");
  ground_plane.transform.position = Vector3::new(0.0, 1.0, 0.0);
  ground_plane.material = MdrMaterial::grey();
  engine.scene.add_object(ground_plane);

  // Start event loop
  info!("Starting event loop");
  event_loop.run(
    move |event, _, control_flow| match engine.handle_event(event) {
      Some(flow) => *control_flow = flow,
      None => (),
    },
  );
}
