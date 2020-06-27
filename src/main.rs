use std::{error};
use std::str::FromStr;

use headless_chrome::Browser;

mod plan;
mod rule;
mod util;

use crate::rule::RuleKind;
use crate::plan::{Plan};
use chrono::{Local};

fn main() -> Result<(), Box<dyn error::Error>> {
    let opts = util::get_opts();

    util::init_logging(&opts.log);

    let mut plan = Plan::new(&opts.plan)?;

    run(&mut plan)
}

fn run(plan: &mut Plan) -> Result<(), Box<dyn error::Error>> {
    for site in plan.sites.iter_mut() {
        log::info!("Checking {} for {}.", &site.description, &site.rule_kind);

        if site.status_changed.unwrap_or(false) {
            notify(true, site, &plan.mailgun);
            continue
        }

        let browser = Browser::default()?;

        let tab = browser.wait_for_initial_tab()?;

        tab.navigate_to(&site.url)?;

        if let Ok(site_rule) = RuleKind::from_str(&site.rule_kind) {
            let changed = site_rule.evaluate(&site, &tab);
            if changed {
                site.status_changed = Option::from(true);
                site.status_changed_date = Option::from(Local::now());
            }
            notify(changed, site, &plan.mailgun);
        }
    }
    plan.save()
}

fn notify(changed: bool, site: &plan::Site, mailgun: &plan::Mailgun) {
    if changed {
        log::info!("{}", site.happy_note);
        mailgun.send(&site.description, &site.happy_note);
    } else {
        log::info!("{}", site.disappointing_note);
    }
}