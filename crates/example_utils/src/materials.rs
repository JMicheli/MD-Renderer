use mdr_engine::{
  MdrEngine,
  resources::{MdrMaterial, MdrMeshMaterialCreateInfo},
};

pub fn make_material(
  engine: &mut MdrEngine,
  name: &str,
  create_info: &MdrMeshMaterialCreateInfo,
) -> MdrMaterial {
  engine
    .manage_resources()
    .create_material(create_info, name)
    .unwrap()
}
