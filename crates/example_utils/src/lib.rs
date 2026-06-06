use std::path::{Path, PathBuf};

use tracing::{Level, subscriber::SetGlobalDefaultError};

mod materials;
pub use materials::make_material;

// Build debug configuration
#[cfg(debug_assertions)]
pub const LOG_LEVEL: Level = Level::DEBUG;
#[cfg(not(debug_assertions))]
pub const LOG_LEVEL: Level = Level::INFO;
#[cfg(debug_assertions)]
pub const DEBUG_ENABLED: bool = true;
#[cfg(not(debug_assertions))]
pub const DEBUG_ENABLED: bool = false;

/// Set up a [`tracing_subscriber`] fmt subscriber with a filter set for [`LOG_LEVEL`].
pub fn initialize_logger() -> Result<(), SetGlobalDefaultError> {
  let subscriber = tracing_subscriber::fmt()
    .with_max_level(LOG_LEVEL)
    .without_time()
    .compact()
    .finish();
  tracing::subscriber::set_global_default(subscriber)
}

// Asset handling
#[cfg(debug_assertions)]
const ASSET_PREFIX: &str = "engine/examples/assets/";
#[cfg(not(debug_assertions))]
const ASSET_PREFIX: &str = "assets/";

/// A helper function for linking assets in both debug and release versions
/// of the example.
pub fn asset(asset_path: &str) -> PathBuf {
  Path::new(ASSET_PREFIX).join(asset_path)
}

/// A helper function for linking mesh assets in both debug and release versions
/// of the example.
///
/// Gets assets from the meshes subdirectory.
pub fn mesh_asset(asset_path: &str) -> PathBuf {
  Path::new(ASSET_PREFIX).join("meshes").join(asset_path)
}

/// A helper function for linking mesh assets in both debug and release versions
/// of the example.
///
/// Gets assets from the textures subdirectory.
pub fn texture_asset(asset_path: &str) -> PathBuf {
  Path::new(ASSET_PREFIX).join("textures").join(asset_path)
}
