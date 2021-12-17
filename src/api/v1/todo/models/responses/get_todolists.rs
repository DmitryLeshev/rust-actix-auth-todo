use serde::Serialize;

use crate::{api::v1::todo::models::TodoLists, common::models::Pagination};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseGetTodoLists {
    pub items: TodoLists,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}
