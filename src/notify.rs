use crate::Mailgun;

pub fn notify(changed: bool, data: &NotifyData, mailgun: &Mailgun) {
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