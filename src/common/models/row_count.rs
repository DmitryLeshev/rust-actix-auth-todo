use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct RowCount {
    pub count: i64,
}
