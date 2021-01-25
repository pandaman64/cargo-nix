pub type Result<T, E = anyhow::Error> = anyhow::Result<T, E>;

mod crates_io;
mod nix;

fn main() -> Result<()> {
    let crate_name = "ripgrep";

    let versions = crates_io::retrieve_crate_versions(crate_name)?;
    let latest = &versions.versions[0];

    let tempdir = tempfile::tempdir()?;
    let crate_path = crates_io::crate_path(tempdir.path(), latest);
    println!("{}", crate_path.display());

    crates_io::unpack_crate(tempdir.path(), latest)?;

    nix::crate2nix(&crate_path)?;
    nix::nix_build(&crate_path)?;

    Ok(())
}
