use std::sync::Arc;

use vulkano::image::{sampler::Sampler, view::ImageView};

use super::MdrColorType;

#[derive(Debug)]
pub struct MdrTexture {
  pub name: String,
}

pub struct MdrTextureCreateInfo<'a> {
  pub source: &'a str,
  pub color_type: MdrColorType,
  pub sampler_mode: MdrSamplerMode,
}

#[derive(Clone)]
pub struct MdrGpuTextureHandle {
  pub(crate) image_view: Arc<ImageView>,
  pub(crate) sampler: Arc<Sampler>,
}

/// Refers to various texture sampling options supported by the engine.
#[derive(Eq, Hash, PartialEq)]
pub enum MdrSamplerMode {
  /// The texture will repeat when u, v, w > 1.0
  Repeat,

  /// The texture will use the edge pixel at u, v, w > 1.0.
  ClampToEdge,
}
