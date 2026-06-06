use mdr_engine::{
  MdrEngine,
  resources::{MdrMaterial, MdrMaterialCreateInfo, MdrRgb, MdrTexture},
};

pub fn make_material(
  engine: &mut MdrEngine,
  name: &str,
  diffuse: MdrTexture,
  roughness: MdrTexture,
  normal: MdrTexture,
  specular_color: MdrRgb,
  shininess: f32,
) -> MdrMaterial {
  engine
    .manage_resources()
    .create_material(
      &MdrMaterialCreateInfo {
        diffuse,
        roughness,
        normal,
        specular_color,
        shininess,
      },
      name,
    )
    .unwrap()
}
