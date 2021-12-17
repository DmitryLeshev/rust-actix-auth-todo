use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DTOCreateTodoList {
    pub account_id: Option<i64>,
    #[validate(length(min = 3))]
    pub name: String,
}
