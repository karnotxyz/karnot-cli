use std::fmt::Display;
use std::str::FromStr;

use inquire::{Confirm, CustomType, InquireError, Select};

pub fn get_text_input(message: &str, default: Option<&str>) -> Result<String, InquireError> {
    let default = default.map(|s| s.to_string());
    get_custom_input::<String>(message, default, None)
}

pub fn get_custom_input<T: Clone + FromStr + Display>(
    message: &str,
    default: Option<T>,
    help_message: Option<&str>,
) -> Result<T, InquireError> {
    let mut prompt = CustomType::new(message);
    if let Some(default) = default {
        prompt = prompt.with_default(default);
    }
    if let Some(help_message) = help_message {
        prompt = prompt.with_help_message(help_message);
    }
    prompt.prompt()
}

pub fn get_option<T: Display>(message: &str, options: Vec<T>) -> Result<T, InquireError> {
    Select::new(message, options).prompt()
}

pub fn get_boolean_input(message: &str, default: Option<bool>) -> Result<bool, InquireError> {
    let mut prompt = Confirm::new(message);
    if let Some(default) = default {
        prompt = prompt.with_default(default);
    }
    prompt.prompt()
}
