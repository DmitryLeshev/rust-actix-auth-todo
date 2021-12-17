use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DTOGetTodoItems {
    pub account_id: i64,
    pub todolist_id: i64,
    pub limit: i64,
    pub offset: i64,
    #[validate(length(min = 3))]
    pub name: Option<String>,
}
