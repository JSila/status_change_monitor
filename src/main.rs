use std::{error, fs, env, path};

use headless_chrome::Browser;

use plan::Plan;

mod plan;
mod opts;
mod multipart;

fn main() -> Result<(), Box<dyn error::Error>> {
    let opts = opts::get();

    set_logging(&opts.log);

    let plan = Plan::new(&opts.plan)?;

    run(&plan)
}

fn run(plan: &Plan) -> Result<(), Box<dyn error::Error>> {
    for site in plan.sites.iter() {
        log::info!("Checking {}", &site.description);

        let browser = Browser::default()?;

        let tab = browser.wait_for_initial_tab()?;

        tab.navigate_to(&site.url)?;

        match tab.wait_for_element(&site.selector) {
            Ok(_) => {
                log::info!("{}", &site.disappointing_note);
            },
            Err(_) => {
                log::info!("{}", &site.happy_note);
                plan.mailgun.send(&site.description, &site.happy_note);
                log::info!("Recipient notified via email");
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