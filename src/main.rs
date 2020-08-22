use std::error;
use std::str::FromStr;
use std::sync::mpsc::channel;

use chrono::Local;
use headless_chrome::Browser;
use threadpool::ThreadPool;

use status_change_monitor::*;

fn main() -> Result<(), Box<dyn error::Error>> {
    let opts = util::get_opts();

    util::init_logging(&opts.log);

    let plan = Plan::new(&opts.plan)?;

    run(plan)
}

fn run(mut plan: Plan) -> Result<(), Box<dyn error::Error>> {
    let pool = ThreadPool::new(4);

    let n_sites = plan.sites.len();
    let (tx, rx) = channel();

    for site in plan.sites.into_iter() {
        let _tx = tx.clone();
        let mut _site = site.clone();

        pool.execute(move || {
            let status_already_changed = _site.status_changed.unwrap_or(false);

            if !status_already_changed {
                log::info!("Checking {} for {}.", _site.description, _site.rule_kind);

                let browser = Browser::default()
                    .expect("cannot get default browser");
                let tab = browser.wait_for_initial_tab()
                    .expect("cannot get browser tab");
                tab.navigate_to(&_site.url)
                    .expect("cannot navigate in browser");

                if let Ok(site_rule) = RuleKind::from_str(&_site.rule_kind) {
                    let changed = site_rule.evaluate(&site, &tab);

                    if changed {
                        _site.status_changed = Option::from(true);
                        _site.status_changed_date = Option::from(Local::now());
                    }
                }
            }

            _tx.send(_site).unwrap();
        });
    }

    plan.sites = rx.iter().take(n_sites).collect();

    let ids_in_multiples: Vec<String> = plan.multiples.iter()
        .map(|m| m.ids.clone())
        .flatten()
        .collect();

    for site in plan.sites.iter() {
        if ids_in_multiples.contains(&site.id) {
            continue;
        }

        let notify_data = NotifyData {
            description: site.description.to_string(),
            happy_note: site.happy_note.to_string(),
            disappointing_note: site.disappointing_note.to_string(),
        };

        notify(site.status_changed.unwrap_or(false), &notify_data, &plan.mailgun);
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