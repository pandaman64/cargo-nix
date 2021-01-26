mod crates_io;
mod nix;
mod opts;

use std::path::{Path, PathBuf};

use bstr::ByteSlice;
pub use color_eyre::eyre as anyhow;
pub type Result<T, E = anyhow::Error> = anyhow::Result<T, E>;

fn install_hooks() -> Result<()> {
    use tracing_subscriber::prelude::*;

    // Install hook for tracing.
    // Nice formatter
    let fmt = tracing_subscriber::fmt::layer();
    // Filtering with RUST_LOG
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    // SpanTrace for the error location
    let trace = tracing_error::ErrorLayer::default();
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt)
        .with(trace)
        .init();

    // Install color-eyre
    color_eyre::install()?;

    Ok(())
}

#[tracing::instrument]
fn find_version(crate_name: &str, version: Option<&str>) -> Result<crates_io::Version> {
    let versions = crates_io::retrieve_crate_versions(crate_name)?;
    match version {
        // By default, pick the latest version.
        None => versions
            .versions
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("latest version not found")),
        Some(s) => versions
            .versions
            .into_iter()
            .find(|v| v.num == s)
            .ok_or_else(|| anyhow::anyhow!("specified version not found")),
    }
}

#[tracing::instrument]
fn unpack_crate(build_dir: &Path, version: &crates_io::Version) -> Result<PathBuf> {
    crates_io::unpack_crate(&build_dir, &version)?;
    let crate_path = crates_io::crate_path(&build_dir, &version);

    Ok(crate_path)
}

fn main() -> Result<()> {
    install_hooks()?;

    let opts = opts::parse();
    let crate_name = &opts.crate_name;
    let tempdir;
    let build_dir = match &opts.build_dir {
        None => {
            tempdir = tempfile::tempdir()?;
            tempdir.path()
        }
        Some(path) => path,
    };

    let version = find_version(crate_name, opts.version.as_deref())?;
    let crate_path = unpack_crate(build_dir, &version)?;

    let nixpkgs = opts.nixpkgs.map(|p| std::fs::canonicalize(p)).transpose()?;
    nix::crate2nix(&crate_path, nixpkgs.as_deref())?;
    let output = nix::nix_build(&crate_path)?;
    println!("{}", String::from_utf8_lossy(&output));
    if opts.install {
        nix::install(output.trim().to_path()?)?;
    }

    Ok(())
}
