use std::{path};

use clap::Clap;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Jernej S.")]
pub struct Opts {
    pub plan: path::PathBuf,
    pub log: path::PathBuf,
}

pub fn get() -> Opts {
    Opts::parse()
}