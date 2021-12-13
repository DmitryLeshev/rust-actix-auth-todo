use serde::{Deserialize, Serialize};
use validator_derive::Validate;

use crate::common::models::Role;
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DTOCreateAccount {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub first_name: String,
    #[validate(length(min = 3))]
    pub last_name: String,
    #[validate(length(min = 3))]
    pub hash_password: String,
    pub role: Option<Role>,
}

// fn validate_password(password: &str) -> Result<(), ValidationError> {
//     // // let regex = Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$")
//     // // .unwrap();

//     let regex = Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$").unwrap();
//     if regex.is_match(password) {
//         return Ok(());
//     };
//     Err(ValidationError::new("terrible_hashpassword"))
// }
