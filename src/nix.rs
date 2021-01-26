use super::Result;

use std::{
    path::Path,
    process::{Command, Stdio},
};

#[tracing::instrument]
pub fn crate2nix(path: &Path) -> Result<()> {
    Command::new("crate2nix")
        .args(&["generate"])
        .current_dir(path)
        .status()?;

    Ok(())
}

#[tracing::instrument]
pub fn nix_build(path: &Path) -> Result<Vec<u8>> {
    let output = Command::new("nix-build")
        .args(&["-A", "rootCrate.build", "Cargo.nix"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .current_dir(path)
        .output()?;

    Ok(output.stdout)
}

#[tracing::instrument]
pub fn install(path: &Path) -> Result<()> {
    Command::new("nix-env")
        .args(&["-i".as_ref(), path])
        .status()?;

    Ok(())
}
