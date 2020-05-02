use std::{error, fs, path};
use std::str::FromStr;

use headless_chrome::Browser;

use plan::Plan;

use crate::rule::RuleKind;

mod multipart;
mod opts;
mod plan;
mod rule;

fn main() -> Result<(), Box<dyn error::Error>> {
    let opts = opts::get();

    set_logging(&opts.log);

    let plan = Plan::new(&opts.plan)?;

    run(&plan)
}

fn run(plan: &Plan) -> Result<(), Box<dyn error::Error>> {
    for site in plan.sites.iter() {
        log::info!("Checking {} for {}.", &site.description, &site.rule_kind);

        let browser = Browser::default()?;

        let tab = browser.wait_for_initial_tab()?;

        tab.navigate_to(&site.url)?;

        if let Ok(site_rule) = RuleKind::from_str(&site.rule_kind) {
            if site_rule.evaluate(&site, &tab) {
                log::info!("{}", &site.happy_note);
                plan.mailgun.send(&site.description, &site.happy_note);
            } else {
                log::info!("{}", &site.disappointing_note);
            }
        }
    }

    Ok(())
}

fn set_logging(log: &path::PathBuf) {
    let config = simplelog::ConfigBuilder::new()
        .add_filter_allow_str("status_change_monitor")
        .set_time_format_str("%F %T")
        .set_time_to_local(true)
        .build();

    let file = fs::File::create(log).expect("Cannot create log file");

    simplelog::WriteLogger::init(log::LevelFilter::Info, config, file).unwrap();
}