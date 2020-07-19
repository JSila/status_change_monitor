use std::sync::Arc;

use headless_chrome::{Element, Tab};
use strum_macros::EnumString;

use crate::plan::Site;

pub struct PresenceOf(
    // presence=true
    // absence=false
    pub bool,
    // element=false
    // text=true
    pub bool
);

pub struct NumberChange(
    // can be one of >, <, >=, <=, ==
    pub String
);

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
    #[strum(serialize=">")]
    MoreThan,
    #[strum(serialize=">=")]
    MoreOrEqualThan,
    #[strum(serialize="<")]
    LessThan,
    #[strum(serialize="<=")]
    LessOrEqualThan,
    #[strum(serialize="==")]
    EqualThan,
}

impl RuleKind {
    pub fn evaluate(&self, site: &Site, tab: &Arc<Tab>) -> bool {
        let rule: Box<dyn Rule> = match self {
            RuleKind::AbsenceOfElement  => Box::new(PresenceOf(false, false)),
            RuleKind::AbsenceOfText     => Box::new(PresenceOf(false, true)),
            RuleKind::PresenceOfElement => Box::new(PresenceOf(true, false)),
            RuleKind::PresenceOfText    => Box::new(PresenceOf(true, true)),
            RuleKind::MoreThan          => Box::new(NumberChange(">".to_string())),
            RuleKind::MoreOrEqualThan   => Box::new(NumberChange(">=".to_string())),
            RuleKind::LessThan          => Box::new(NumberChange("<".to_string())),
            RuleKind::LessOrEqualThan   => Box::new(NumberChange("<=".to_string())),
            RuleKind::EqualThan         => Box::new(NumberChange("==".to_string())),
        };

        rule.evaluate(site, tab)
    }
}

impl Rule for PresenceOf {
    fn evaluate(&self, site: &Site, tab: &Arc<Tab>) -> bool {

        if site.text.is_none() {
            return false;
        }

        let site_text = site.text
            .as_ref()
            .unwrap();

        match tab.wait_for_element(&site.selector) {
            Ok(element) => {
                if self.1 && get_element_text(&element) != *site_text {
                    return !self.0;
                }
            },
            Err(_) => {
                return !self.0;
            }
        }
        self.0
    }
}

impl Rule for NumberChange {
    fn evaluate(&self, site: &Site, tab: &Arc<Tab>) -> bool {

        if site.value.is_none() {
            return false;
        }

        let site_value = site.value
            .as_ref()
            .unwrap();

        match tab.wait_for_element(&site.selector) {
            Ok(element) => {
                let actual_value = get_element_value(&element);

                return match self.0.as_str() {
                    ">"  => actual_value > *site_value,
                    ">=" => actual_value >= *site_value,
                    "<"  => actual_value < *site_value,
                    "<=" => actual_value <= *site_value,
                    "==" => actual_value == *site_value,
                    _    => actual_value == *site_value
                }
            },
            Err(_) => false
        }
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

fn get_element_value(element: &Element) -> f32 {
    get_element_text(element)
        .parse::<f32>()
        .expect("Problem converting text to f32 number")
}