use serde::{Deserialize, Serialize};
use validator_derive::Validate;

use crate::common::models::Role;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DTOUpdateAccount {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 3, max = 12))]
    pub first_name: Option<String>,
    #[validate(length(min = 3, max = 12))]
    pub last_name: Option<String>,
    pub role: Option<Role>,
}
