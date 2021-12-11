use serde::Serialize;

use crate::{api::v1::account::models::Account, common::models::Pagination};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseGetAccounts {
    pub items: Vec<Account>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}
