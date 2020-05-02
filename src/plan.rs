use std::{error, fs, path};
use serde::{Deserialize, Serialize};

use crate::multipart::MultiPart;

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub description: String,
    pub url: String,
    pub selector: String,
    pub rule_kind: String,
    #[serde(default)]
    pub text: String,
    pub happy_note: String,
    pub disappointing_note: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mailgun {
    pub from: String,
    pub to: String,
    pub domain: String,
    pub api_key: String,
}

impl Mailgun {
    pub fn send(&self, subject: &str, text: &str) {
        let url = format!("https://api.mailgun.net/v3/{}/messages", self.domain);

        let mut data = MultiPart::new();
        data.add_string("from", &self.from);
        data.add_string("to", &self.to);
        data.add_string("subject", subject);
        data.add_string("html", text);

        let response = ureq::post(&url)
            .auth("api", &self.api_key)
            .set("Content-Type", &format!("multipart/form-data; boundary={}", data.boundary))
            .send_bytes(&data.to_bytes());

        if response.status() != 200 {
            log::error!("Cannot send to {}: {:?}", self.to, response);
        } else {
            log::info!("Sent notification to {}", self.to);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plan {
    pub sites: Vec<Site>,
    pub mailgun: Mailgun,

    #[serde(skip)]
    filename: path::PathBuf,
}

impl Plan {
    pub fn new(filename: &path::PathBuf) -> Result<Plan, Box<dyn error::Error>> {
        let mut plan: Plan = serde_json::from_reader(fs::File::open(filename)?)?;
        plan.filename = filename.clone();
        Ok(plan)
    }
}