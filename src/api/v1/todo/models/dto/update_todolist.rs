use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DTOUpdateTodoList {
    pub account_id: i64,
    pub todolist_id: i64,
    #[validate(length(min = 3))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DTOJsonUpdateTodoList {
    #[validate(length(min = 3))]
    pub name: Option<String>,
}
