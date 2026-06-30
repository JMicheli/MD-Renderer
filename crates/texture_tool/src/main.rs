//! This module defines a CLI for interacting with the texture-tool during
//! development as needed. Currently, it exposes the following functionality:
//!
//! * Generate combined metal-roughness maps from separate maps.
//! * Flip the green (Y) channel of normal maps to convert GL to DX-style maps.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use mdr_texture_tool::{invert_normal_map, merge_metallic_and_roughness};

#[derive(Parser)]
#[command(name = "texture-tool")]
#[command(about = "A CLI tool for processing textures", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// Combine metal and roughness maps into a single file
  CombineMetalRoughness {
    /// Path to the metallic map
    #[arg(long)]
    metallic: PathBuf,

    /// Path to the roughness map
    #[arg(long)]
    roughness: PathBuf,

    /// Output path for the combined map
    #[arg(long)]
    output: PathBuf,
  },

  /// Invert the Y channel of a normal map
  InvertNormalY {
    /// Path to the source normal map
    #[arg(long)]
    input: PathBuf,

    /// Output path for the inverted map
    #[arg(long)]
    output: PathBuf,
  },
}

fn main() {
  let cli = Cli::parse();

  match &cli.command {
    // Combine Metal and Roughness Command
    // ///////////////////////////////////
    Commands::CombineMetalRoughness {
      metallic,
      roughness,
      output,
    } => {
      println!("Combining metal: {metallic:?} and roughness: {roughness:?}");
      let combined_image = match merge_metallic_and_roughness(metallic, roughness) {
        Ok(res) => res,
        Err(e) => {
          eprintln!("Failed to combine maps: {e}");
          return;
        }
      };

      // Write combined image out
      match combined_image.save(output) {
        Ok(_) => println!("Saved combined map to {output:?}"),
        Err(e) => eprintln!("Failed to save combined map: {e}"),
      }
    }

    // Invert Normal Y Command
    // ///////////////////////
    Commands::InvertNormalY { input, output } => {
      println!("Inverting Y channel of {input:?}");

      let inverted_map = match invert_normal_map(input) {
        Ok(res) => res,
        Err(e) => {
          eprintln!("Failed to invert normal map: {e}");
          return;
        }
      };

      match inverted_map.save(output) {
        Ok(_) => println!("Saved inverted normal map to {output:?}"),
        Err(e) => eprintln!("Failed to save inverted normal map: {e}"),
      }
    }
  }
}
