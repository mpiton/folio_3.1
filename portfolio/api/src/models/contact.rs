use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Request {
    #[validate(length(
        min = 2,
        max = 100,
        message = "Le nom doit faire entre 2 et 100 caractères"
    ))]
    pub name: String,

    #[validate(email(message = "L'email n'est pas valide"))]
    pub email: String,

    #[validate(length(
        min = 10,
        max = 1000,
        message = "Le message doit faire entre 10 et 1000 caractères"
    ))]
    pub message: String,
}
