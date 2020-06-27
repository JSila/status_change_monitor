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

    let ids_in_multiples: Vec<String> = plan.multiples.iter()
        .map(|m| m.ids.clone())
        .flatten()
        .collect();

    for site in plan.sites.iter_mut() {
        log::info!("Checking {} for {}.", &site.description, &site.rule_kind);

        let in_multiples = ids_in_multiples.contains(&site.id);

        let notify_data = NotifyData {
            description: site.description.to_string(),
            happy_note: site.happy_note.to_string(),
            disappointing_note: site.disappointing_note.to_string(),
        };

        // do not check again if the system found out in previous execution the site had changed status
        if site.status_changed.unwrap_or(false) {
            if !in_multiples {
                notify(true, &notify_data, &plan.mailgun);
            }
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
            if !in_multiples {
                notify(changed, &notify_data, &plan.mailgun);
            }
        }
    }

    for multiple in plan.multiples.iter_mut() {
        let changed = plan.sites.iter()
            .filter(|s| multiple.ids.contains(&s.id))
            .all(|s| s.status_changed.unwrap_or(false));

        if changed {
            multiple.status_changed = Option::from(true);
            multiple.status_changed_date = Option::from(Local::now());
        }

        let description = plan.sites.iter()
            .filter(|s| multiple.ids.contains(&s.id))
            .map(|s| s.description.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let happy_note = plan.sites.iter()
            .filter(|s| multiple.ids.contains(&s.id))
            .map(|s| s.happy_note.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let disappointing_note = plan.sites.iter()
            .filter(|s| multiple.ids.contains(&s.id))
            .map(|s| {
                if s.status_changed.unwrap_or(false) {
                    s.happy_note.to_string()
                } else {
                    s.disappointing_note.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        let notify_data = NotifyData {
            description,
            happy_note,
            disappointing_note,
        };
        notify(changed, &notify_data, &plan.mailgun);
    }

    plan.save()
}

fn notify(changed: bool, data: &NotifyData, mailgun: &plan::Mailgun) {
    if changed {
        log::info!("{}", data.happy_note);
        mailgun.send(&data.description, &data.happy_note);
    } else {
        log::info!("{}", data.disappointing_note);
    }
}

pub struct NotifyData {
    pub description: String,
    pub happy_note: String,
    pub disappointing_note: String,
}