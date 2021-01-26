use std::{ffi::OsString, path::PathBuf};

use clap::Clap;

#[derive(Clap)]
pub struct Opts {
    pub crate_name: String,
    #[clap(short, long)]
    pub install: bool,
    #[clap(long)]
    pub version: Option<String>,
    #[clap(long)]
    pub build_dir: Option<PathBuf>,
}

#[tracing::instrument]
pub fn parse() -> Opts {
    let mut args: Vec<OsString> = std::env::args_os().collect();
    tracing::debug!(?args);

    // Skip the subcommand name if invoked as a cargo subcommand.
    // `nix` crate cannot be a bin crate, so no need to worry.
    if args.len() > 1 && args[1] == "nix" {
        args.remove(1);
    }

    Opts::parse_from(args)
}
