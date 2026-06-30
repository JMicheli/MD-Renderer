//! This library defines various functions for manipulating textures. They can
//! be used in the engine directly by depending on this library, or they can
//! be used via the CLI defined in the binary for this crate.

mod error;
mod metallic_roughness;
mod normal;

pub use error::TextureToolError;
pub use metallic_roughness::merge_metallic_and_roughness;
pub use normal::invert_normal_map;

use std::path::Path;

use image::{DynamicImage, ImageReader};

/// A helper function to open the `source` path and load the contents as a [`DynamicImage`].
pub fn load_dynamic_image<P: AsRef<Path>>(source: P) -> Result<DynamicImage, TextureToolError> {
  Ok(ImageReader::open(source)?.decode()?)
}
