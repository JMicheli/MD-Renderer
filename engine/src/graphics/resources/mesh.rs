use std::path::Path;

use vulkano::buffer::Subbuffer;

use super::{MdrVertex_norm, MdrVertex_pos, MdrVertex_uv, vertex::MdrVertex_tan};

#[derive(Default)]
pub struct MdrMeshData {
  pub positions: Vec<MdrVertex_pos>,
  pub normals: Vec<MdrVertex_norm>,
  pub uvs: Vec<MdrVertex_uv>,
  pub tangents: Vec<MdrVertex_tan>,

  pub indices: Vec<u32>,
  pub index_count: u32,
}

#[derive(Debug, Clone)]
pub struct MdrMesh {
  pub name: String,
}

pub struct MdrGpuMeshHandle {
  pub(crate) positions_buffer: Subbuffer<[MdrVertex_pos]>,
  pub(crate) normals_buffer: Subbuffer<[MdrVertex_norm]>,
  pub(crate) uvs_buffer: Subbuffer<[MdrVertex_uv]>,
  pub(crate) tangents_buffer: Subbuffer<[MdrVertex_tan]>,

  pub(crate) index_buffer: Subbuffer<[u32]>,
  pub(crate) index_count: u32,
}

pub fn open_obj(path: &Path) -> Option<MdrMeshData> {
  // Load data from disk
  let options = tobj::GPU_LOAD_OPTIONS;
  let load_result = tobj::load_obj(path, &options);
  let (models, _) = match load_result {
    Ok(value) => value,
    Err(e) => {
      tracing::error!("Failed to load obj file: {path:?}, reason: {e}");
      return None;
    }
  };

  // Take only the first model
  let model = &models[0];
  let model_positions = &model.mesh.positions;
  let model_normals = &model.mesh.normals;
  let model_uvs = &model.mesh.texcoords;

  // Prepare data structures
  let vertex_count = model.mesh.positions.len() / 3;
  let mut mesh_positions = Vec::<MdrVertex_pos>::with_capacity(vertex_count);
  let mut mesh_normals = Vec::<MdrVertex_norm>::with_capacity(vertex_count);
  let mut mesh_uvs = Vec::<MdrVertex_uv>::with_capacity(vertex_count);

  // Loop over model vertices
  for vertex_n in 0..vertex_count {
    let index_3d = vertex_n * 3;
    let index_2d = vertex_n * 2;

    mesh_positions.push(MdrVertex_pos {
      a_position: [
        model_positions[index_3d],
        model_positions[index_3d + 1],
        model_positions[index_3d + 2],
      ],
    });
    mesh_normals.push(MdrVertex_norm {
      a_normal: [
        model_normals[index_3d],
        model_normals[index_3d + 1],
        model_normals[index_3d + 2],
      ],
    });

    // Handle missing UVs by writting dummy UV values
    // TODO - Is this the right solution here?
    if !model_uvs.is_empty() {
      mesh_uvs.push(MdrVertex_uv {
        a_uv: [model_uvs[index_2d], model_uvs[index_2d + 1]],
      });
    } else {
      mesh_uvs.push(MdrVertex_uv { a_uv: [0.0, 0.0] })
    }
  }

  let mesh_tangents = calculate_mesh_tangents(&mesh_positions, &mesh_uvs, &model.mesh.indices);

  Some(MdrMeshData {
    positions: mesh_positions,
    normals: mesh_normals,
    uvs: mesh_uvs,
    tangents: mesh_tangents,

    indices: model.mesh.indices.clone(),
    index_count: model.mesh.indices.len() as u32,
  })
}

/// Calculates a matrix of tangent vector values from input vectors containing
/// a mesh's positions, uvs, and indices.
fn calculate_mesh_tangents(
  mesh_positions: &[MdrVertex_pos],
  mesh_uvs: &[MdrVertex_uv],
  mesh_indices: &[u32],
) -> Vec<MdrVertex_tan> {
  // Allocate memory for averaging tangent values
  let mut tangent_vals = Vec::<Vec<MdrVertex_tan>>::with_capacity(mesh_positions.len());
  for _ in 0..mesh_positions.len() {
    tangent_vals.push(Vec::<MdrVertex_tan>::new());
  }

  // Count up all our tangent vectors
  for i in (0..mesh_indices.len()).step_by(3) {
    // Get the vertex index for each triangle vertex
    let (tri_v1, tri_v2, tri_v3) = (
      mesh_indices[i] as usize,
      mesh_indices[i + 1] as usize,
      mesh_indices[i + 2] as usize,
    );

    // Push tangent values onto averaging vectors
    // TODO this is insane, there's gotta be a better way
    tangent_vals[tri_v1].push(calculate_tangent(
      (mesh_positions[tri_v1], mesh_uvs[tri_v1]),
      (mesh_positions[tri_v2], mesh_uvs[tri_v2]),
      (mesh_positions[tri_v3], mesh_uvs[tri_v3]),
    ));
    tangent_vals[tri_v2].push(calculate_tangent(
      (mesh_positions[tri_v2], mesh_uvs[tri_v2]),
      (mesh_positions[tri_v3], mesh_uvs[tri_v3]),
      (mesh_positions[tri_v1], mesh_uvs[tri_v1]),
    ));
    tangent_vals[tri_v3].push(calculate_tangent(
      (mesh_positions[tri_v3], mesh_uvs[tri_v3]),
      (mesh_positions[tri_v1], mesh_uvs[tri_v1]),
      (mesh_positions[tri_v2], mesh_uvs[tri_v2]),
    ));
  }

  // Create the final output by averaging our values
  let mut tangent_data = Vec::<MdrVertex_tan>::with_capacity(mesh_positions.len());
  for vals in &tangent_vals {
    let mut sum: [f32; 3] = [0.0; 3];
    for tan in vals {
      sum[0] += tan.a_tangent[0];
      sum[1] += tan.a_tangent[1];
      sum[2] += tan.a_tangent[2];
    }

    let recip_len = 1.0 / (vals.len() as f32);
    tangent_data.push(MdrVertex_tan {
      a_tangent: [sum[0] / recip_len, sum[1] / recip_len, sum[2] / recip_len],
    });
  }

  tangent_data
}

/// Calculates the tangent vector for `target_vert` based on the
/// co-triangular vertices `v2` and `v3`.
fn calculate_tangent(
  target_vert: (MdrVertex_pos, MdrVertex_uv),
  v2: (MdrVertex_pos, MdrVertex_uv),
  v3: (MdrVertex_pos, MdrVertex_uv),
) -> MdrVertex_tan {
  // Edge from target vertex to v2
  let edge1 = [
    v2.0.a_position[0] - target_vert.0.a_position[0],
    v2.0.a_position[1] - target_vert.0.a_position[1],
    v2.0.a_position[2] - target_vert.0.a_position[2],
  ];
  // Edge from target vertex to v3
  let edge2 = [
    v3.0.a_position[0] - target_vert.0.a_position[0],
    v3.0.a_position[1] - target_vert.0.a_position[1],
    v3.0.a_position[2] - target_vert.0.a_position[2],
  ];
  // UV edge from target vertex to v2
  let delta_uv1 = [
    v2.1.a_uv[0] - target_vert.1.a_uv[0],
    v2.1.a_uv[1] - target_vert.1.a_uv[1],
  ];
  // UV edge from target vertex to v3
  let delta_uv2 = [
    v3.1.a_uv[0] - target_vert.1.a_uv[0],
    v3.1.a_uv[1] - target_vert.1.a_uv[1],
  ];

  // Compute the fractional part of the tangent equation
  let frac = 1.0 / delta_uv1[0].mul_add(delta_uv2[1], -(delta_uv2[0] * delta_uv1[1]));
  // Compute each component
  let tan_x = frac * delta_uv2[1].mul_add(edge1[0], -(delta_uv1[1] * edge2[0]));
  let tan_y = frac * delta_uv2[1].mul_add(edge1[1], -(delta_uv1[1] * edge2[1]));
  let tan_z = frac * delta_uv2[1].mul_add(edge1[2], -(delta_uv1[1] * edge2[2]));

  MdrVertex_tan {
    a_tangent: [tan_x, tan_y, tan_z],
  }
}
