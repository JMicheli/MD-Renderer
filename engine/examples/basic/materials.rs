use mdr_engine::{
  MdrEngine,
  resources::{
    MdrColorType, MdrMaterial, MdrRgb, MdrTexture,
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
    metal_plates_base_color,
    metal_plates_roughness,
    metal_plates_normal,
    MdrRgb::white(),
    20.0,
  )
}

pub fn blue_tile(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  let (blue_tiles_base_color, blue_tiles_roughness, blue_tiles_normal) =
    load_textures(engine, "blue_tiles");

  make_material(
    engine,
    name,
    blue_tiles_base_color,
    blue_tiles_roughness,
    blue_tiles_normal,
    MdrRgb::white(),
    20.0,
  )
}

pub fn wood_planks(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  let (wood_planks_base_color, wood_planks_roughness, wood_planks_normal) =
    load_textures(engine, "wood_planks");

  make_material(
    engine,
    name,
    wood_planks_base_color,
    wood_planks_roughness,
    wood_planks_normal,
    MdrRgb::white(),
    10.0,
  )
}

pub fn white_bricks(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  let (white_bricks_base_color, white_bricks_roughness, white_bricks_normal) =
    load_textures(engine, "white_bricks");
  make_material(
    engine,
    name,
    white_bricks_base_color,
    white_bricks_roughness,
    white_bricks_normal,
    MdrRgb::white(),
    2.0,
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
