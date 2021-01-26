use super::{anyhow, Result};

use std::{
    path::Path,
    process::{Command, Stdio},
};

use color_eyre::Help;

#[tracing::instrument]
pub fn crate2nix(package_dir: &Path, nixpkgs: Option<&Path>) -> Result<()> {
    let mut command = Command::new("crate2nix");
    command.arg("generate").current_dir(package_dir);
    if let Some(nixpkgs) = nixpkgs {
        command.args(&["--nixpkgs-path".as_ref(), nixpkgs]);
    }

    anyhow::ensure!(
        command.status()?.success(),
        anyhow::anyhow!("failed to complete crate2nix").suggestion(
            "if you find a crate that cannot be built with `cargo-nix`,
please file an issue at https://github.com/pandaman64/cargo-nix/issues/new."
        )
    );

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

    anyhow::ensure!(
        output.status.success(),
        anyhow::anyhow!("failed to complete nix-build").suggestion(
            "if you find a crate that cannot be built with `cargo-nix`,
please file an issue at https://github.com/pandaman64/cargo-nix/issues/new."
        )
    );

    Ok(output.stdout)
}

#[tracing::instrument]
pub fn install(path: &Path) -> Result<()> {
    let status = Command::new("nix-env")
        .args(&["-i".as_ref(), path])
        .status()?;

    anyhow::ensure!(status.success(), "failed to run `nix-env -i`");

    Ok(())
}
