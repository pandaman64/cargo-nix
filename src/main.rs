use flate2::read::GzDecoder;
use serde::Deserialize;
use tar::Archive;

use std::{path::Path, process::Command};

#[derive(Debug, Deserialize)]
struct Version {
    dl_path: String,
    num: String,
}

#[derive(Debug, Deserialize)]
struct Versions {
    versions: Vec<Version>,
}

const API_ROOT: &str = "https://crates.io";

fn versions(crate_name: &str) -> String {
    format!("{}/api/v1/crates/{}/versions", API_ROOT, crate_name)
}

fn downloads(dl_path: &str) -> String {
    format!("{}{}", API_ROOT, dl_path)
}

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

    let versions: Versions = ureq::get(&versions(crate_name))
        .set("User-Agent", "cargo-nix")
        .call()?
        .into_json()?;
    let latest = &versions.versions[0];

    let tempdir = tempfile::tempdir()?;
    let crate_path = {
        let mut path = tempdir.path().to_owned();
        path.push(format!("{}-{}", crate_name, latest.num));
        path
    };
    println!("{}", crate_path.display());

    let krate = ureq::get(&downloads(&latest.dl_path))
        .set("User-Agent", "cargo-nix")
        .call()?
        .into_reader();
    let mut krate = Archive::new(GzDecoder::new(krate));
    krate.unpack(tempdir.path())?;

    run_crate2nix(&crate_path);
    nix_build(&crate_path);
    Ok(())
}
