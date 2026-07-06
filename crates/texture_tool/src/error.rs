/// An error thrown by the texture-tool library.
#[derive(Debug, thiserror::Error)]
pub enum TextureToolError {
  #[error("I/O error when loading texture: {0}")]
  IoFailed(#[from] std::io::Error),
  #[error("Processing image failed: {0}")]
  ImageError(#[from] image::ImageError),
  #[error("Expected roughness and metallic maps to have equal dimensions")]
  WidthHeightMismatch,
  #[error("Unsupported color format: {0:?}")]
  GltfColorFormatUnsupported(gltf::image::Format),
  #[error("Failed to convert gLTF image to DynamicImage")]
  GltfImageConversionFailed,
}
