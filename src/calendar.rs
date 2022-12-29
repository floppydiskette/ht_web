use std::sync::Arc;
use ht_cal::datetime::{Month, MonthStatus};
use serde::Serialize;
use crate::WithTemplate;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CalendarTemplate {
    pub gl: String,
    pub months: String,
}

fn gen_select_code(selections: Vec<String>, selected: usize) -> String {
    let mut code = String::new();
    for (i, s) in selections.iter().enumerate() {
        code.push_str(&format!(r#"<option value="{}"{}>{}</option>"#, i, if i == selected { " selected" } else { "" }, s));
    }
    code
}

pub fn route() -> WithTemplate<CalendarTemplate> {
    let month = crate::synch::HDATE.lock().unwrap().month.clone();
    let gl = gen_select_code(vec!["greater".to_string(), "lesser".to_string()], if month.0 == MonthStatus::Greater { 0 } else { 1 });
    let months = gen_select_code(
        vec![
            "zero".to_string(),
            "niktvirin".to_string(),
            "apress".to_string(),
            "smosh".to_string(),
            "funny".to_string(),
        ],
        match month.1 {
            Month::Zero => 0,
            Month::Niktvirin => 1,
            Month::Apress => 2,
            Month::Smosh => 3,
            Month::Funny => 4,
        },
    );
    WithTemplate {
        name: "calendar",
        value: CalendarTemplate {
            gl,
            months,
        },
    }
}