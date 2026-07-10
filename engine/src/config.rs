//! This module contains high-level configuration variables for the engine as a whole.
//! If a device feature or Vulkan extension needs to be enabled, it is added here.
//!
//! This module also defines certain `const` values such as maximum number of lights and
//! textures that can be sent to a shader.

use vulkano::device::{DeviceExtensions, DeviceFeatures};

/// Layers that the engine will attempt to enable in debug
pub const VULKAN_DEBUG_LAYERS: &[&str] = &[
  // Validates vulkan usage during runtime, requires installation of
  // the Vulkan debug layers on the system running it.
  "VK_LAYER_KHRONOS_validation",
];

/// Enabled device features
pub const ENABLED_FEATURES: DeviceFeatures = DeviceFeatures {
  // Allow indexing into descriptor indexing so that we can use an array for
  // textures.
  runtime_descriptor_array: true,
  // Allows descriptor arrays to have variable counts across shader invocations,
  // which is required for certain PBR textures to be optional.
  descriptor_binding_variable_descriptor_count: true,
  ..DeviceFeatures::empty()
};

/// Enabled device extensions
pub const ENABLED_EXTENSIONS: DeviceExtensions = DeviceExtensions {
  // Enable swapchain support, required for realtime rendering.
  khr_swapchain: true,
  // Allows setting negative viewport height, we set a negative viewport
  // height in order to invert the Y axis and align it with the GL convention.
  khr_maintenance1: true,
  ..DeviceExtensions::empty()
};

/// Maximum number of point lights allowed in a scene.
pub const MAX_POINT_LIGHTS: usize = 10;

/// Maximum length of `textures[]` in `mesh.frag`.
pub const MAX_MESH_FRAG_TEXTURES: u32 = 5;
