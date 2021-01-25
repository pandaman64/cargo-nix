mod crates_io;
mod nix;
mod opts;

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

fn main() -> Result<()> {
    install_hooks()?;

    let opts = opts::parse();
    let crate_name = &opts.crate_name;

    let versions = crates_io::retrieve_crate_versions(crate_name)?;
    let version = match opts.version {
        // By default, pick the latest version.
        None => &versions.versions[0],
        Some(s) => versions
            .versions
            .iter()
            .find(|v| v.num == s)
            .ok_or_else(|| anyhow::anyhow!("specified version not found"))?,
    };

    let tempdir = tempfile::tempdir()?;
    let crate_path = crates_io::crate_path(tempdir.path(), version);

    crates_io::unpack_crate(tempdir.path(), version)?;

    nix::crate2nix(&crate_path)?;
    nix::nix_build(&crate_path)?;

    Ok(())
}
