use std::{fs::File, path};

use clap::Clap;
use simplelog::{LevelFilter, WriteLogger};

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Jernej S.")]
pub struct Opts {
    pub plan: path::PathBuf,
    pub log: path::PathBuf,
}

pub fn get_opts() -> Opts {
    Opts::parse()
}

pub fn init_logging(log: &path::PathBuf) {
    let config = simplelog::ConfigBuilder::new()
        .set_time_format_str("%F %T")
        .set_time_to_local(true)
        .build();

    let file = File::create(log).expect("Cannot create log file");

    WriteLogger::init(LevelFilter::Info, config, file)
        .unwrap();
}