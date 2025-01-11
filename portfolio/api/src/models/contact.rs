use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct ContactForm {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 10, max = 1000))]
    pub message: String,
}

#[derive(Deserialize)]
pub struct ContactRequest {
    pub form: ContactForm,
    pub form_time_ms: u64,
}
