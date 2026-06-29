use mdr_engine::{
  MdrEngine,
  resources::{
    MdrColorType, MdrMaterial, MdrMeshMaterialCreateInfo, MdrTexture,
    texture::{MdrSamplerMode, MdrTextureCreateInfo},
  },
};

use mdr_example_utils::{make_material, texture_asset};

pub fn metal_plates(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  let textures = load_textures(engine, "metal_plates");

  make_material(
    engine,
    name,
    &MdrMeshMaterialCreateInfo {
      diffuse: Some(textures.base_color),
      metallic_roughness: Some(textures.metallic_roughness),
      normal: Some(textures.normal),
      occlusion: Some(textures.occlusion),
      ..Default::default()
    },
  )
}

pub fn blue_tile(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  let textures = load_textures(engine, "blue_tiles");

  make_material(
    engine,
    name,
    &MdrMeshMaterialCreateInfo {
      diffuse: Some(textures.base_color),
      metallic_roughness: Some(textures.metallic_roughness),
      normal: Some(textures.normal),
      occlusion: Some(textures.occlusion),
      ..Default::default()
    },
  )
}

pub fn wood_planks(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  let textures = load_textures(engine, "wood_planks");

  make_material(
    engine,
    name,
    &MdrMeshMaterialCreateInfo {
      diffuse: Some(textures.base_color),
      metallic_roughness: Some(textures.metallic_roughness),
      normal: Some(textures.normal),
      occlusion: Some(textures.occlusion),
      ..Default::default()
    },
  )
}

pub fn white_bricks(name: &str, engine: &mut MdrEngine) -> MdrMaterial {
  let textures = load_textures(engine, "white_bricks");
  make_material(
    engine,
    name,
    &MdrMeshMaterialCreateInfo {
      diffuse: Some(textures.base_color),
      metallic_roughness: Some(textures.metallic_roughness),
      normal: Some(textures.normal),
      occlusion: Some(textures.occlusion),
      base_roughness: 1.0,
      base_metallic: 0.0,
      ..Default::default()
    },
  )
}

struct Textures {
  pub base_color: MdrTexture,
  pub metallic_roughness: MdrTexture,
  pub normal: MdrTexture,
  pub occlusion: MdrTexture,
}

fn load_textures(engine: &mut MdrEngine, name_preamble: &str) -> Textures {
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
  let occlusion = engine
    .manage_resources()
    .load_texture(
      MdrTextureCreateInfo {
        source: &texture_asset(&format!("{name_preamble}/occlusion.png")),
        color_type: MdrColorType::NonColorData,
        sampler_mode: MdrSamplerMode::Repeat,
      },
      &format!("{name_preamble}_occlusion"),
    )
    .unwrap();

  // Roughness and metallic maps are separate, but the engine uses a single metallic-roughness map.
  // A helper function is used to load and combine them if both are available, otherwise just
  // roughness will be used.
  let name = format!("{name_preamble}_metallic_roughness");
  let metallic_path = texture_asset(&format!("{name_preamble}/metalness.png"));
  let roughness_path = texture_asset(&format!("{name_preamble}/roughness.png"));
  let metallic_roughness = if metallic_path.exists() {
    engine
      .manage_resources()
      .load_metal_and_roughness_texture(
        &roughness_path,
        &metallic_path,
        MdrColorType::NonColorData,
        MdrSamplerMode::Repeat,
        &name,
      )
      .unwrap()
  } else {
    engine
      .manage_resources()
      .load_texture(
        MdrTextureCreateInfo {
          source: &roughness_path,
          color_type: MdrColorType::NonColorData,
          sampler_mode: MdrSamplerMode::Repeat,
        },
        &name,
      )
      .unwrap()
  };

  Textures {
    base_color,
    metallic_roughness,
    normal,
    occlusion,
  }
}
