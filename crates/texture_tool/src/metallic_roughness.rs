use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgb};

use crate::TextureToolError;

pub fn merge_metallic_and_roughness(
  metal_source: &Path,
  roughness_source: &Path,
) -> Result<DynamicImage, TextureToolError> {
  // Load image data from disk
  let roughness_image = ImageReader::open(roughness_source)?.decode()?;
  let metallic_image = ImageReader::open(metal_source)?.decode()?;

  // Confirm that dimensions match
  let width = roughness_image.width();
  let height = roughness_image.height();
  if (width != metallic_image.width()) || (height != metallic_image.height()) {
    return Err(TextureToolError::WidthHeightMismatch);
  }

  // Create the composite image
  let mut buffer = ImageBuffer::new(width, height);
  for (x, y, pixel) in buffer.enumerate_pixels_mut() {
    let p1 = roughness_image.get_pixel(x, y);
    let p2 = metallic_image.get_pixel(x, y);

    *pixel = Rgb([0, p1[0], p2[0]]);
  }

  Ok(DynamicImage::ImageRgb8(buffer))
}
