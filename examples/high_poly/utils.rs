use std::path::{Path, PathBuf};

use tracing::Level;

// Build debug configuration
#[cfg(debug_assertions)]
pub const LOG_LEVEL: Level = Level::DEBUG;
#[cfg(not(debug_assertions))]
pub const LOG_LEVEL: Level = Level::INFO;
#[cfg(debug_assertions)]
pub const DEBUG_ENABLED: bool = true;
#[cfg(not(debug_assertions))]
pub const DEBUG_ENABLED: bool = false;

// Asset handling
#[cfg(debug_assertions)]
const ASSET_PREFIX: &str = "examples/high_poly/assets/";
#[cfg(not(debug_assertions))]
const ASSET_PREFIX: &str = "assets/";

/// A helper function for linking assets in both debug and release versions
/// of the example.
pub fn asset(asset_path: &str) -> PathBuf {
  Path::new(ASSET_PREFIX).join(Path::new(asset_path))
}
