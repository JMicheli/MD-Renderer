use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};

use crate::{TextureToolError, load_dynamic_image};

/// Loads the `metal_source` and `roughness_source` textures and then combines them into a single
/// [`DynamicImage`] such that the roughness is stored in the green channel and the metalness is
/// stored in the blue channel. The resulting [`DynamicImage`] is then output.
///
/// This corresponds to a convention used in many engines (e.g., Unity) which conserves memory since
/// roughness and metalness are single-channel values and a 3-channel texture is overkill for them.
pub fn merge_metallic_and_roughness<P: AsRef<Path>>(
  metal_source: P,
  roughness_source: P,
) -> Result<DynamicImage, TextureToolError> {
  // Load image data from disk
  let roughness_image = load_dynamic_image(roughness_source)?;
  let metallic_image = load_dynamic_image(metal_source)?;

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
