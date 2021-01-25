use super::Result;

use std::{path::Path, process::Command};

#[tracing::instrument]
pub fn crate2nix(path: &Path) -> Result<()> {
    Command::new("crate2nix")
        .args(&["generate"])
        .current_dir(path)
        .status()?;

    Ok(())
}

#[tracing::instrument]
pub fn nix_build(path: &Path) -> Result<()> {
    Command::new("nix-build")
        .args(&["-A", "rootCrate.build", "Cargo.nix"])
        .current_dir(path)
        .status()?;

    Ok(())
}
