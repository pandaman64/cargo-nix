use super::{anyhow, Result};

use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
};

use color_eyre::Help;

const SUGGESTION: &str = "if you find a crate that cannot be built with `cargo-nix`,
please file an issue at https://github.com/pandaman64/cargo-nix/issues/new.";

#[tracing::instrument]
pub fn crate2nix(package_dir: &Path, nixpkgs: Option<&Path>) -> Result<()> {
    let mut command = Command::new("crate2nix");
    command.arg("generate").current_dir(package_dir);
    if let Some(nixpkgs) = nixpkgs {
        command.args(&["--nixpkgs-path".as_ref(), nixpkgs]);
    }

    anyhow::ensure!(
        command.status()?.success(),
        anyhow::anyhow!("failed to complete crate2nix").suggestion(SUGGESTION)
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
        anyhow::anyhow!("failed to complete nix-build").suggestion(SUGGESTION)
    );

    Ok(output.stdout)
}

#[tracing::instrument]
pub fn install(path: &Path) -> Result<()> {
    let status = Command::new("nix-env")
        .args(&["-i".as_ref(), path])
        .status()?;

    anyhow::ensure!(status.success(), "failed to complete `nix-env -i`");

    Ok(())
}

#[tracing::instrument]
pub fn cargo2nix(package_dir: &Path, nixpkgs: Option<&Path>, crate_name: &str) -> Result<()> {
    let status = Command::new("cargo2nix")
        .arg("-f")
        .current_dir(package_dir)
        .status()?;
    anyhow::ensure!(
        status.success(),
        anyhow::anyhow!("failed to complete cargo2nix").suggestion(SUGGESTION)
    );

    // CR pandaman: consider using env::var_os
    let rust_overlay_path = env::var("RUST_OVERLAY_PATH")?;
    let cargo2nix_overlay_path = env::var("CARGO2NIX_OVERLAY_PATH")?;

    let content = format!(
        r#"let
  rustOverlay = import {};
  cargo2nixOverlay = import {};
  pkgs = import {} {{
    overlays = [ rustOverlay cargo2nixOverlay ];
  }};

  rustPkgs = pkgs.rustBuilder.makePackageSet' {{
    rustChannel = "stable";
    packageFun = import ./Cargo.nix;
  }};
in rustPkgs.workspace.{} {{ }}
"#,
        rust_overlay_path,
        cargo2nix_overlay_path,
        match nixpkgs {
            Some(nixpkgs) => nixpkgs.display().to_string(),
            None => "<nixpkgs>".to_string(),
        },
        crate_name
    );

    fs::write(package_dir.join("default.nix"), content)?;

    Ok(())
}

#[tracing::instrument]
pub fn cargo2nix_build(path: &Path) -> Result<Vec<u8>> {
    let output = Command::new("nix-build")
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .current_dir(path)
        .output()?;

    anyhow::ensure!(
        output.status.success(),
        anyhow::anyhow!("failed to complete nix-build").suggestion(SUGGESTION)
    );

    Ok(output.stdout)
}
