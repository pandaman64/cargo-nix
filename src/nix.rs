use super::Result;

use std::{path::Path, process::Command};

pub fn crate2nix<P: AsRef<Path>>(path: P) -> Result<()> {
    Command::new("crate2nix")
        .args(&["generate"])
        .current_dir(path)
        .status()?;

    Ok(())
}

pub fn nix_build<P: AsRef<Path>>(path: P) -> Result<()> {
    Command::new("nix-build")
        .args(&["-A", "rootCrate.build", "Cargo.nix"])
        .current_dir(path)
        .status()?;

    Ok(())
}
