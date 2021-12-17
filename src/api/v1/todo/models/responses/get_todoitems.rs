use serde::Serialize;

use crate::{api::v1::todo::models::TodoItems, common::models::Pagination};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseGetTodoItems {
    pub items: TodoItems,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}
