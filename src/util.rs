use std::{path,fs::File};

use clap::Clap;

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
        .add_filter_allow_str("status_change_monitor")
        .set_time_format_str("%F %T")
        .set_time_to_local(true)
        .build();

    let file = File::create(log).expect("Cannot create log file");

    simplelog::WriteLogger::init(log::LevelFilter::Info, config, file).unwrap();
}