use std::{error, fs, path};

use serde::{Deserialize, Serialize};

use formdata::{self, FormData};

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

        let form_data = FormData {
            fields: vec![
                ("from".to_string(), self.from.to_string()),
                ("to".to_string(), self.to.to_string()),
                ("subject".to_string(), subject.to_string()),
                ("html".to_string(), text.to_string()),
            ],
            files: vec![]
        };

        let boundary = formdata::generate_boundary();

        let mut data: Vec<u8> = vec![];

        formdata::write_formdata(&mut data, &boundary, &form_data).unwrap();

        let response = ureq::post(&url)
            .auth("api", &self.api_key)
            .set("Content-Type", &format!("multipart/form-data; boundary={}", String::from_utf8_lossy(&boundary)))
            .send_bytes(&data);

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