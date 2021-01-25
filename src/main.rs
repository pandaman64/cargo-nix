mod crates_io;
mod nix;
mod opts;

pub type Result<T, E = anyhow::Error> = anyhow::Result<T, E>;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

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
