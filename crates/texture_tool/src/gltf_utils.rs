use image::DynamicImage;

use crate::TextureToolError;

pub fn gltf_image_to_dynamic_image(
  gltf_img: &gltf::image::Data,
) -> Result<DynamicImage, TextureToolError> {
  let width = gltf_img.width;
  let height = gltf_img.height;
  let bytes = &gltf_img.pixels;

  match gltf_img.format {
    gltf::image::Format::R8 => {
      image::GrayImage::from_raw(width, height, bytes.to_vec()).map(DynamicImage::ImageLuma8)
    }
    gltf::image::Format::R8G8 => {
      image::GrayAlphaImage::from_raw(width, height, bytes.to_vec()).map(DynamicImage::ImageLumaA8)
    }
    gltf::image::Format::R8G8B8 => {
      image::RgbImage::from_raw(width, height, bytes.to_vec()).map(DynamicImage::ImageRgb8)
    }
    gltf::image::Format::R8G8B8A8 => {
      image::RgbaImage::from_raw(width, height, bytes.to_vec()).map(DynamicImage::ImageRgba8)
    }
    _ => None,
  }
  .ok_or(TextureToolError::GltfImageConversionFailed)
}
