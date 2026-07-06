use std::path::Path;

use image::DynamicImage;
use mdr_texture_tool::gltf_utils::gltf_image_to_dynamic_image;

use crate::{
  graphics::MdrResourceManager,
  resources::{
    MdrColorType, MdrMaterial, MdrMesh, MdrMeshData, MdrMeshMaterialCreateInfo, MdrTexture,
    MdrVertex_norm, MdrVertex_pos, MdrVertex_uv,
    texture::{MdrSamplerMode, MdrTextureCreateInfo},
    vertex::MdrVertex_tan,
  },
  scene::{MdrRenderObject, MdrScene, transform::MdrTransform},
};

pub fn from_path<P: AsRef<Path>>(
  path: P,
  resource_manager: &mut MdrResourceManager,
) -> Result<MdrScene, GltfLoadError> {
  tracing::info!("Loading {:?}", path.as_ref());
  let (document, buffers, images) = gltf::import(path)?;

  // Load textures
  let texture_list = load_gltf_textures(&images, resource_manager)?;
  // Load materials and associate them with the textures
  let material_list = load_gltf_materials(&document, resource_manager, &texture_list)?;

  // Load meshes
  let mesh_list = load_gltf_meshes(&document, &buffers, resource_manager)?;

  // Load objects
  let scene = load_gltf_objects(&document, &material_list, &mesh_list)?;

  Ok(scene)
}

fn load_gltf_textures(
  images: &[gltf::image::Data],
  resource_manager: &mut MdrResourceManager,
) -> Result<Vec<MdrTexture>, GltfLoadError> {
  let mut texture_list = Vec::with_capacity(images.len());

  for (index, image_data) in images.iter().enumerate() {
    let color_type = match image_data.format {
      gltf::image::Format::R8G8B8 | gltf::image::Format::R8G8B8A8 => MdrColorType::SRGBA,
      _ => return Err(GltfLoadError::UnsupportedColorFormat(image_data.format)),
    };

    let image: DynamicImage = gltf_image_to_dynamic_image(image_data)?;

    let texture = resource_manager
      .load_texture(
        &MdrTextureCreateInfo {
          color_type,
          image,
          sampler_mode: MdrSamplerMode::Repeat,
        },
        &format!("texture_{index}"),
      )
      .unwrap();

    texture_list.push(texture);
  }

  Ok(texture_list)
}

fn load_gltf_materials(
  document: &gltf::Document,
  resource_manager: &mut MdrResourceManager,
  texture_list: &[MdrTexture],
) -> Result<Vec<MdrMaterial>, GltfLoadError> {
  let mut material_list = Vec::new();

  for gltf_material in document.materials() {
    let pbr = gltf_material.pbr_metallic_roughness();
    let mut create_info = MdrMeshMaterialCreateInfo {
      base_color: pbr.base_color_factor().into(),
      base_roughness: pbr.roughness_factor(),
      base_metallic: pbr.metallic_factor(),
      ..Default::default()
    };

    if let Some(diffuse_info) = pbr.base_color_texture() {
      let texture = texture_list.get(diffuse_info.texture().index()).unwrap();
      create_info.diffuse = Some(texture.clone());
    }

    if let Some(mr_info) = gltf_material
      .pbr_metallic_roughness()
      .metallic_roughness_texture()
    {
      let texture = texture_list.get(mr_info.texture().index()).unwrap();
      create_info.metallic_roughness = Some(texture.clone());
    }

    if let Some(normal_info) = gltf_material.normal_texture() {
      let texture = texture_list.get(normal_info.texture().index()).unwrap();
      create_info.normal = Some(texture.clone());
    }

    if let Some(occlusion_info) = gltf_material.occlusion_texture() {
      let texture = texture_list.get(occlusion_info.texture().index()).unwrap();
      create_info.occlusion = Some(texture.clone());
    }

    if let Some(emissive_info) = gltf_material.emissive_texture() {
      let texture = texture_list.get(emissive_info.texture().index()).unwrap();
      create_info.emissive = Some(texture.clone());
    }

    // Name material either based on index or default_material if no index present
    let name = gltf_material.index().map_or_else(
      || "default_material".to_string(),
      |i| format!("material_{i}"),
    );

    let material = resource_manager.create_material(&create_info, &name)?;
    material_list.push(material);
  }

  Ok(material_list)
}

fn load_gltf_meshes(
  document: &gltf::Document,
  buffers: &[gltf::buffer::Data],
  resource_manager: &mut MdrResourceManager,
) -> Result<Vec<MdrMesh>, GltfLoadError> {
  let mut mesh_list = Vec::new();

  for gltf_mesh in document.meshes() {
    // TODO - Currently only grabs the first primitive, I might eventually want to grab them all
    let primitive = gltf_mesh.primitives().next().unwrap();
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

    let positions = reader
      .read_positions()
      .map(|iter| {
        iter
          .map(|a_position| MdrVertex_pos { a_position })
          .collect()
      })
      .ok_or(GltfLoadError::MeshPrimitiveMissingPositions)?;

    let normals = reader
      .read_normals()
      .map(|iter| iter.map(|a_normal| MdrVertex_norm { a_normal }).collect())
      .unwrap_or_default();

    let tangents = reader
      .read_tangents()
      .map(|iter| {
        iter
          .map(|tan| MdrVertex_tan {
            a_tangent: [tan[0], tan[1], tan[2]],
          })
          .collect()
      })
      .unwrap_or_default();

    let uvs = reader
      .read_tex_coords(0)
      .map(|iter| iter.into_f32().map(|a_uv| MdrVertex_uv { a_uv }).collect())
      .unwrap_or_default();

    let indices: Vec<_> = reader
      .read_indices()
      .map(|iter| iter.into_u32().collect())
      .ok_or(GltfLoadError::MeshPrimitiveMissingIndices)?;

    let mesh_data = MdrMeshData {
      positions,
      normals,
      uvs,
      tangents,
      index_count: indices.len() as u32,
      indices,
    };

    let name = format!("mesh_{}", gltf_mesh.index());
    let mesh = resource_manager.load_mesh_data(&name, &mesh_data)?;
    mesh_list.push(mesh);
  }

  Ok(mesh_list)
}

fn load_gltf_objects(
  document: &gltf::Document,
  material_list: &[MdrMaterial],
  mesh_list: &[MdrMesh],
) -> Result<MdrScene, GltfLoadError> {
  let mut scene = MdrScene::new();

  let Some(gltf_scene) = document
    .default_scene()
    .or_else(|| document.scenes().next())
  else {
    return Err(GltfLoadError::NoGltfScene);
  };

  for (index, node) in gltf_scene.nodes().enumerate() {
    tracing::debug!("Processing node {index}");

    if let Some(render_obj) = process_node(&node, material_list, mesh_list) {
      let name = format!("Node_{index}");
      scene.add_object(&name, render_obj);
      tracing::debug!("Added object {name} to scene");
    }
  }

  Ok(scene)
}

// Recursively processes nodes
fn process_node(
  node: &gltf::Node,
  material_list: &[MdrMaterial],
  mesh_list: &[MdrMesh],
) -> Option<MdrRenderObject> {
  // Only objects with meshes will be rendered
  node.mesh().map(|gltf_mesh| {
    let mesh = mesh_list[gltf_mesh.index()].clone();

    // Grab the material assigned to the first primitive, or a default
    let primitive = gltf_mesh.primitives().next().unwrap();
    let material = primitive.material().index().map_or_else(
      || material_list[0].clone(),
      |mat_idx| material_list[mat_idx].clone(),
    );

    let mut render_object = MdrRenderObject::new(mesh, material);

    // Apply the node's local transform and recursively attach children
    render_object.transform = MdrTransform::from_matrix(node.transform().matrix());
    for (index, child_node) in node.children().enumerate() {
      if let Some(child_obj) = process_node(&child_node, material_list, mesh_list) {
        render_object.add_child(child_obj);
        tracing::debug!("Added child object (idx: {index}) to parent");
      }
    }

    render_object
  })
}

#[derive(Debug, thiserror::Error)]
pub enum GltfLoadError {
  #[error("Error loading gLTF file: {0}")]
  FileIoFailed(#[from] std::io::Error),
  #[error("gltf crate threw an error: {0}")]
  GltfError(#[from] gltf::Error),
  #[error("Error loading texture: {0}")]
  TextureLoadError(#[from] mdr_texture_tool::TextureToolError),
  #[error("Unsupported color format: {0:?}")]
  UnsupportedColorFormat(gltf::image::Format),
  #[error("Error loading resource: {0}")]
  ResourceError(#[from] crate::graphics::MdrResourceError),

  #[error("Mesh primitive is missing position attributes")]
  MeshPrimitiveMissingPositions,
  #[error("Mesh primitive is missing indices")]
  MeshPrimitiveMissingIndices,
  #[error("Unable to identify default or first scene in gLTF")]
  NoGltfScene,
}
