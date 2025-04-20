use std::error::Error;

use inquire::{Text, validator::Validation};

use crate::{file::is_file_exist, ssh::is_private_key_file};

pub fn text_input<'a>(message: &str) -> Text<'a> {
    let prompt = format!("{}: ", message);
    let prompt = Box::leak(prompt.into_boxed_str());

    Text::new(prompt).with_validator(|input: &str| {
        if input.trim().is_empty() {
            Ok(Validation::Invalid("Value cannot be empty".into()))
        } else {
            Ok(Validation::Valid)
        }
    })
}

pub fn text_input_with_default<'a>(message: &str, default: &'a str) -> Text<'a> {
    text_input(message).with_default(default)
}

pub fn ssh_key_path_validator(key_path: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
    if key_path.trim().is_empty() {
        Ok(Validation::Invalid("Value cannot be empty".into()))
    } else if !is_private_key_file(key_path) {
        Ok(Validation::Invalid(
            "File is public key (don't use the .pub file)".into(),
        ))
    } else if !is_file_exist(key_path) {
        Ok(Validation::Invalid("File does not exist".into()))
    } else {
        Ok(Validation::Valid)
    }
}
