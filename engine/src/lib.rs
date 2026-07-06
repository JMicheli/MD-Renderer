#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
  clippy::missing_panics_doc,
  clippy::missing_errors_doc,
  clippy::cast_precision_loss,
  clippy::cast_possible_truncation,
  clippy::default_trait_access,
  clippy::cast_sign_loss,
  clippy::must_use_candidate,
  clippy::struct_excessive_bools
)]

mod application;
mod config;
mod engine;
mod graphics;
mod input;
mod load_gltf;
pub mod scene;

pub use application::{MdrApplication, MdrRunOptions, run_application};
pub use engine::MdrEngine;
pub use graphics::resources;
pub use input::MdrInputState;
