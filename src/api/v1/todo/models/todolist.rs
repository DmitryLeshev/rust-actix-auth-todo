use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TodoList {
    todolist_id: i64,
    account_id: i64,
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
