use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DTODeleteTodoList {
    pub account_id: i64,
    pub todolist_id: i64,
}
