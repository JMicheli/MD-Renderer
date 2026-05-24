use std::sync::Arc;

use vulkano::{
  device::Device,
  format::Format,
  render_pass::{RenderPass, Subpass},
};

pub struct MdrRenderPass {
  render_pass: Arc<RenderPass>,
}

impl MdrRenderPass {
  pub fn new(logical_device: &Arc<Device>, swapchain_image_format: Format) -> Self {
    let render_pass = vulkano::single_pass_renderpass!(
      logical_device.clone(),
      attachments: {
        color: {
          format: swapchain_image_format,
          samples: 1,
          load_op: Clear,
          store_op: Store,
        },
        depth: {
          format: Format::D16_UNORM,
          samples: 1,
          load_op: Clear,
          store_op: DontCare,
        }
      },
      pass: {
        color: [color],
        depth_stencil: {depth}
      }
    )
    .unwrap();

    Self { render_pass }
  }

  pub fn get_pass(&self) -> Arc<RenderPass> {
    self.render_pass.clone()
  }

  pub fn get_subpass(&self) -> Subpass {
    Subpass::from(self.render_pass.clone(), 0).unwrap()
  }
}
