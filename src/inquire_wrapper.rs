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
