use std::{error};
use std::str::FromStr;

use headless_chrome::Browser;

mod plan;
mod rule;
mod util;

use crate::rule::RuleKind;
use crate::plan::{Plan};

fn main() -> Result<(), Box<dyn error::Error>> {
    let opts = util::get_opts();

    util::init_logging(&opts.log);

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