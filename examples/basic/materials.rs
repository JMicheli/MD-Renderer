use mdr_engine::{
  MdrEngine,
  resources::{
    MdrColorType, MdrMaterial, MdrMaterialCreateInfo, MdrRgb,
    texture::{MdrSamplerMode, MdrTextureCreateInfo},
  },
};

use crate::utils::asset;

pub fn metal_plates(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  // Metal plates
  let metal_plates_base_color = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &asset("textures/metal_plates/base_color.png"),
        color_type: MdrColorType::SRGBA,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "metal_plates_base_color",
    )
    .unwrap();
  let metal_plates_roughness = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &asset("textures/metal_plates/roughness.png"),
        color_type: MdrColorType::NonColorData,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "metal_plates_roughness",
    )
    .unwrap();
  let metal_plates_normal = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &asset("textures/metal_plates/normal.png"),
        color_type: MdrColorType::NonColorData,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "metal_plates_normal",
    )
    .unwrap();

  engine
    .manage_resources()
    .create_material(
      &MdrMaterialCreateInfo {
        diffuse: metal_plates_base_color,
        roughness: metal_plates_roughness,
        normal: metal_plates_normal,
        specular_color: MdrRgb::white(),
        shininess: 20.0,
      },
      name,
    )
    .unwrap()
}

pub fn blue_tile(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  // Blue tiles
  let blue_tiles_base_color = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &asset("textures/blue_tiles/base_color.png"),
        color_type: MdrColorType::SRGBA,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "blue_tiles_base_color",
    )
    .unwrap();
  let blue_tiles_roughness = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &asset("textures/blue_tiles/roughness.png"),
        color_type: MdrColorType::NonColorData,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "blue_tiles_roughness",
    )
    .unwrap();
  let blue_tiles_normal = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &asset("textures/blue_tiles/normal.png"),
        color_type: MdrColorType::NonColorData,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "blue_tiles_normal",
    )
    .unwrap();

  engine
    .manage_resources()
    .create_material(
      &MdrMaterialCreateInfo {
        diffuse: blue_tiles_base_color,
        roughness: blue_tiles_roughness,
        normal: blue_tiles_normal,
        specular_color: MdrRgb::white(),
        shininess: 20.0,
      },
      name,
    )
    .unwrap()
}

pub fn wood_planks(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  // Wood planks
  let wood_planks_base_color = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &asset("textures/wood_planks/base_color.png"),
        color_type: MdrColorType::SRGBA,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "wood_planks_base_color",
    )
    .unwrap();
  let wood_planks_roughness = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &asset("textures/wood_planks/roughness.png"),
        color_type: MdrColorType::NonColorData,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "wood_planks_roughness",
    )
    .unwrap();
  let wood_planks_normal = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &asset("textures/wood_planks/normal.png"),
        color_type: MdrColorType::NonColorData,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "wood_planks_normal",
    )
    .unwrap();

  engine
    .manage_resources()
    .create_material(
      &MdrMaterialCreateInfo {
        diffuse: wood_planks_base_color,
        roughness: wood_planks_roughness,
        normal: wood_planks_normal,
        specular_color: MdrRgb::white(),
        shininess: 20.0,
      },
      name,
    )
    .unwrap()
}

pub fn white_bricks(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  let white_bricks_base_color = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &asset("textures/white_bricks/base_color.png"),
        color_type: MdrColorType::SRGBA,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "white_bricks_base_color",
    )
    .unwrap();
  let white_bricks_roughness = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &asset("textures/white_bricks/roughness.png"),
        color_type: MdrColorType::NonColorData,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "white_bricks_roughness",
    )
    .unwrap();
  let white_bricks_normal = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &asset("textures/white_bricks/normal.png"),
        color_type: MdrColorType::NonColorData,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      "white_bricks_normal",
    )
    .unwrap();

  engine
    .manage_resources()
    .create_material(
      &MdrMaterialCreateInfo {
        diffuse: white_bricks_base_color,
        roughness: white_bricks_roughness,
        normal: white_bricks_normal,
        specular_color: MdrRgb::white(),
        shininess: 2.0,
      },
      name,
    )
    .unwrap()
}
