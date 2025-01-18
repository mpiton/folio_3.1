use regex::Regex;
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Request {
    #[validate(length(
        min = 2,
        max = 100,
        message = "Le nom doit faire entre 2 et 100 caractères"
    ))]
    #[validate(custom(
        function = "validate_text",
        message = "Le nom contient des caractères non autorisés"
    ))]
    pub name: String,

    #[validate(email(message = "L'email n'est pas valide"))]
    pub email: String,

    #[validate(length(
        min = 2,
        max = 100,
        message = "Le sujet doit faire entre 2 et 100 caractères"
    ))]
    #[validate(custom(
        function = "validate_text",
        message = "Le sujet contient des caractères non autorisés"
    ))]
    pub subject: String,

    #[validate(length(
        min = 10,
        max = 1000,
        message = "Le message doit faire entre 10 et 1000 caractères"
    ))]
    #[validate(custom(
        function = "validate_text",
        message = "Le message contient des caractères non autorisés"
    ))]
    pub message: String,
}

lazy_static::lazy_static! {
    static ref SAFE_TEXT_REGEX: Regex = Regex::new(r"^[\p{L}\p{N}\s.,!?@()'\[\]\-_&+=%°:;]+$").unwrap();
}

fn validate_text(text: &str) -> Result<(), validator::ValidationError> {
    if SAFE_TEXT_REGEX.is_match(text) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("caractères_non_autorisés"))
    }
}
