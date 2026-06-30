use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};

use crate::{TextureToolError, load_dynamic_image};

/// Loads a texture from `source` and flips the green channel (corresponding to the Y component
/// of a normal map) to convert OpenGL-style normal maps to DirectX-style normal maps and visa-
/// versa. Outputs a [`DynamicImage`] containing the adjusted map.
pub fn invert_normal_map<P: AsRef<Path>>(source: P) -> Result<DynamicImage, TextureToolError> {
  // Load image data from disk
  let source_image = load_dynamic_image(source)?;

  // Create output image
  let width = source_image.width();
  let height = source_image.height();
  let mut buffer = ImageBuffer::new(width, height);
  for (x, y, pixel) in buffer.enumerate_pixels_mut() {
    let sp = source_image.get_pixel(x, y);
    *pixel = Rgb([sp[0], 255 - sp[1], sp[2]]);
  }

  Ok(DynamicImage::ImageRgb8(buffer))
}
