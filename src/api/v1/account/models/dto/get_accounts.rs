use serde::Deserialize;

use crate::api::v1::account::models::QuerySearchAccounts;

#[derive(Deserialize, Debug, Clone)]
pub struct DTOGetAccounts {
    pub query: QuerySearchAccounts,
    pub base_path: String,
    pub domain: String,
    pub version: u8,
}
