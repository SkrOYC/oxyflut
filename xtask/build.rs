//! Generates test-only modules that compile the authoritative Rust contracts verbatim.

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_directory = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let workspace_root = manifest_directory
        .parent()
        .ok_or_else(|| io::Error::other("xtask has no workspace parent"))?;
    let substrate_path =
        workspace_root.join(".constitution/tech-spec/contracts/oxyflut-substrate.rs");
    let public_path = workspace_root.join(".constitution/tech-spec/contracts/oxyflut-public.rs");
    let substrate = fs::read_to_string(&substrate_path)?;
    let public = fs::read_to_string(&public_path)?;
    let generated =
        format!("pub mod substrate {{\n{substrate}\n}}\npub mod public {{\n{public}\n}}\n");
    let output = PathBuf::from(env::var("OUT_DIR")?).join("native-authoritative-contracts.rs");
    fs::write(output, generated)?;
    println!("cargo::rerun-if-changed={}", substrate_path.display());
    println!("cargo::rerun-if-changed={}", public_path.display());
    Ok(())
}
