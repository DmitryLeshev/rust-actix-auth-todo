use serde::Deserialize;
use validator_derive::Validate;

// use validator::{Validate, ValidationError};

#[derive(Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DTOSignIn {
    #[validate(email)]
    pub email: String,
    #[validate(required)]
    pub password: Option<String>,
}
