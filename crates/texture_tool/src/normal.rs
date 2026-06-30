use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgb};

use crate::TextureToolError;

pub fn invert_normal_map(source: &Path) -> Result<DynamicImage, TextureToolError> {
  // Load image data from disk
  let source_image = ImageReader::open(source)?.decode()?;

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
