use std::sync::Arc;
use headless_chrome::{Tab, Element};
use strum_macros::EnumString;

use crate::plan::Site;

pub struct AbsenceOf(pub bool);
pub struct PresenceOf(pub bool);

pub(crate) trait Rule {
    fn evaluate(&self, site: &Site, tab: &Arc<Tab>) -> bool;
}

#[derive(EnumString, Debug)]
pub enum RuleKind {
    #[strum(serialize="absence of element")]
    AbsenceOfElement,
    #[strum(serialize="absence of text")]
    AbsenceOfText,
    #[strum(serialize="presence of element")]
    PresenceOfElement,
    #[strum(serialize="presence of text")]
    PresenceOfText,
}

impl RuleKind {
    pub fn evaluate(&self, site: &Site, tab: &Arc<Tab>) -> bool {
        let rule: Box<dyn Rule> = match self {
            RuleKind::AbsenceOfElement => Box::new(AbsenceOf(false)),
            RuleKind::AbsenceOfText => Box::new(AbsenceOf(true)),
            RuleKind::PresenceOfElement => Box::new(PresenceOf(false)),
            RuleKind::PresenceOfText => Box::new(PresenceOf(true)),
        };

        rule.evaluate(site, tab)
    }
}

impl Rule for AbsenceOf {
    fn evaluate(&self, site: &Site, tab: &Arc<Tab>) -> bool {
        match tab.wait_for_element(&site.selector) {
            Ok(element) => {
                if self.0 && get_element_text(&element) != site.text {
                    return true;
                }
            },
            Err(_) => {
                return true;
            }
        }
        false
    }
}

impl Rule for PresenceOf {
    fn evaluate(&self, site: &Site, tab: &Arc<Tab>) -> bool {
        match tab.wait_for_element(&site.selector) {
            Ok(element) => {
                if self.0 {
                    if get_element_text(&element) != site.text {
                        return false;
                    }
                }
            },
            Err(_) => {
                return false;
            }
        }
        true
    }
}

fn get_element_text(element: &Element) -> String {
    if let Ok(node) = element.get_description() {
        if let Some(nodes) = node.children {
            if let Some(node) = nodes.first() {
                return node.node_value.trim().to_string();
            }
        }
    }
    "".to_string()
}