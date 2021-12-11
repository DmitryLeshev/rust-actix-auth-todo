use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(skip_serializing)]
    pub hash_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
