use super::Result;

use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use serde::Deserialize;
use tar::Archive;

#[derive(Debug, Deserialize)]
pub struct Version {
    pub dl_path: String,
    #[serde(rename = "crate")]
    pub krate: String,
    pub num: String,
}

#[derive(Debug, Deserialize)]
pub struct Versions {
    pub versions: Vec<Version>,
}

const CRATES_IO_ROOT: &str = "https://crates.io";
const USER_AGENT: &str = "cargo-nix";

fn versions(crate_name: &str) -> String {
    format!("{}/api/v1/crates/{}/versions", CRATES_IO_ROOT, crate_name)
}

fn downloads(dl_path: &str) -> String {
    format!("{}{}", CRATES_IO_ROOT, dl_path)
}

#[tracing::instrument]
pub fn retrieve_crate_versions(crate_name: &str) -> Result<Versions> {
    ureq::get(&versions(crate_name))
        .set("User-Agent", USER_AGENT)
        .call()?
        .into_json()
        .map_err(Into::into)
}

#[tracing::instrument]
pub fn unpack_crate(base: &Path, version: &Version) -> Result<()> {
    let krate = ureq::get(&downloads(&version.dl_path))
        .set("User-Agent", USER_AGENT)
        .call()?
        .into_reader();
    let mut krate = Archive::new(GzDecoder::new(krate));
    krate.unpack(base).map_err(Into::into)
}

#[tracing::instrument]
pub fn crate_path(base: &Path, version: &Version) -> PathBuf {
    let mut buf = base.to_path_buf();
    buf.push(format!("{}-{}", version.krate, version.num));
    buf
}
