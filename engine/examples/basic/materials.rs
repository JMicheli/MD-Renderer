use mdr_engine::{
  MdrEngine,
  resources::{
    MdrColorType, MdrMaterial, MdrMeshMaterialCreateInfo, MdrTexture,
    texture::{MdrSamplerMode, MdrTextureCreateInfo},
  },
};

use mdr_example_utils::{make_material, texture_asset};

pub fn metal_plates(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  let (metal_plates_base_color, metal_plates_roughness, metal_plates_normal) =
    load_textures(engine, "metal_plates");

  make_material(
    engine,
    name,
    &MdrMeshMaterialCreateInfo {
      diffuse: Some(metal_plates_base_color),
      metallic_roughness: Some(metal_plates_roughness),
      normal: Some(metal_plates_normal),
      ..Default::default()
    },
  )
}

pub fn blue_tile(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  let (blue_tiles_base_color, blue_tiles_roughness, blue_tiles_normal) =
    load_textures(engine, "blue_tiles");

  make_material(
    engine,
    name,
    &MdrMeshMaterialCreateInfo {
      diffuse: Some(blue_tiles_base_color),
      metallic_roughness: Some(blue_tiles_roughness),
      normal: Some(blue_tiles_normal),
      ..Default::default()
    },
  )
}

pub fn wood_planks(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  let (wood_planks_base_color, wood_planks_roughness, wood_planks_normal) =
    load_textures(engine, "wood_planks");

  make_material(
    engine,
    name,
    &MdrMeshMaterialCreateInfo {
      diffuse: Some(wood_planks_base_color),
      metallic_roughness: Some(wood_planks_roughness),
      normal: Some(wood_planks_normal),
      ..Default::default()
    },
  )
}

pub fn white_bricks(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  let (white_bricks_base_color, white_bricks_roughness, white_bricks_normal) =
    load_textures(engine, "white_bricks");
  make_material(
    engine,
    name,
    &MdrMeshMaterialCreateInfo {
      diffuse: Some(white_bricks_base_color),
      metallic_roughness: Some(white_bricks_roughness),
      normal: Some(white_bricks_normal),
      base_roughness: 1.0,
      base_metallic: 0.0,
      ..Default::default()
    },
  )
}

fn load_textures(
  engine: &mut MdrEngine,
  name_preamble: &str,
) -> (MdrTexture, MdrTexture, MdrTexture) {
  let base_color = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &texture_asset(&format!("{name_preamble}/base_color.png")),
        color_type: MdrColorType::SRGBA,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      &format!("{name_preamble}_base_color"),
    )
    .unwrap();
  let roughness = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &texture_asset(&format!("{name_preamble}/roughness.png")),
        color_type: MdrColorType::NonColorData,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      &format!("{name_preamble}_roughness"),
    )
    .unwrap();
  let normal = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &texture_asset(&format!("{name_preamble}/normal.png")),
        color_type: MdrColorType::NonColorData,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      &format!("{name_preamble}_normal"),
    )
    .unwrap();

  (base_color, roughness, normal)
}
