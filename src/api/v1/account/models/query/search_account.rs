use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct QuerySearchAccounts {
    pub email: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}
