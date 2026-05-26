use std::path::Path;

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
const ASSET_PREFIX: &str = "examples/basic/assets/";
#[cfg(not(debug_assertions))]
const ASSET_PREFIX: &str = "assets/";

/// A helper function for linking assets in both debug and release versions
/// of the example.
pub fn asset(asset_path: &str) -> String {
  let asset_path_prefix = Path::new(ASSET_PREFIX);
  asset_path_prefix
    .join(Path::new(asset_path))
    .to_str()
    .unwrap()
    .to_string()
}
