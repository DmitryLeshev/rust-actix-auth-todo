use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TodoItem {
    todolist_id: i64,
    todoitem_id: i64,
    name: String,
    description: Option<String>,
    active: bool,
    completed: bool,
    deleted: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
