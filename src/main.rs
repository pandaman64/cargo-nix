use std::{path::Path, process::Command};

pub type Result<T, E = anyhow::Error> = anyhow::Result<T, E>;

mod crates_io;
mod nix;

fn run_crate2nix<P: AsRef<Path>>(path: P) {
    Command::new("crate2nix")
        .args(&["generate"])
        .current_dir(path)
        .status()
        .unwrap();
}

fn nix_build<P: AsRef<Path>>(path: P) {
    Command::new("nix-build")
        .args(&["-A", "rootCrate.build", "Cargo.nix"])
        .current_dir(path)
        .status()
        .unwrap();
}

fn main() -> anyhow::Result<()> {
    let crate_name = "ripgrep";

    let versions = crates_io::retrieve_crate_versions(crate_name)?;
    let latest = &versions.versions[0];

    let tempdir = tempfile::tempdir()?;
    let crate_path = crates_io::crate_path(tempdir.path(), latest);
    println!("{}", crate_path.display());

    crates_io::unpack_crate(tempdir.path(), latest)?;

    run_crate2nix(&crate_path);
    nix_build(&crate_path);
    Ok(())
}
