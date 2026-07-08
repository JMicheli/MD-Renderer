/// Layers that the engine will attempt to enable in debug
pub const VULKAN_DEBUG_LAYERS: &[&str] = &[
  // Validates vulkan usage during runtime
  "VK_LAYER_KHRONOS_validation",
];

/// Maximum number of point lights allowed in a scene
pub const MAX_POINT_LIGHTS: usize = 10;

/// Maximum length of textures[] in mesh.frag
pub const MAX_MESH_FRAG_TEXTURES: u32 = 5;
