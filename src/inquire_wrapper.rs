use inquire::{Text, validator::Validation};

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
