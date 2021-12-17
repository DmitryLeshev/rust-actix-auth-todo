use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DTOCreateTodoItem {
    pub todolist_id: Option<i64>,
    #[validate(length(min = 3))]
    pub name: String,
    pub description: Option<String>,
}
