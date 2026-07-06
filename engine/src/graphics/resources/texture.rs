use std::sync::Arc;

use image::DynamicImage;
use vulkano::image::{sampler::Sampler, view::ImageView};

use super::MdrColorType;

// TODO - Should switch to using indices or something instead of strings
#[derive(Debug, Clone)]
pub struct MdrTexture {
  pub name: String,
}

pub struct MdrTextureCreateInfo {
  pub image: DynamicImage,
  pub color_type: MdrColorType,
  pub sampler_mode: MdrSamplerMode,
}

#[derive(Clone)]
pub struct MdrGpuTextureHandle {
  pub(crate) image_view: Arc<ImageView>,
  pub(crate) sampler: Arc<Sampler>,
}

/// Refers to various texture sampling options supported by the engine.
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum MdrSamplerMode {
  /// The texture will repeat when u, v, w > 1.0
  Repeat,

  /// The texture will use the edge pixel at u, v, w > 1.0.
  ClampToEdge,
}
