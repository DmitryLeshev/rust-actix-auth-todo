use serde::{Deserialize, Serialize};
use std::net::IpAddr;

use crate::{api::v1::account::models::Account, common::models::Role};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionData {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub authorities: Role,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionUser {
    pub session_id: String,
    pub account_id: Option<i64>,
    pub token: Option<String>,
    pub ip: Option<IpAddr>,
    pub data: SessionData,
}

impl Default for SessionUser {
    fn default() -> Self {
        Self {
            session_id: String::from("Default"),
            account_id: None,
            token: None,
            ip: None,
            data: SessionData::default(),
        }
    }
}

impl SessionUser {
    pub fn new(session_id: String, account: Account, role: Role, ip: IpAddr) -> Self {
        Self {
            session_id,
            account_id: Some(account.account_id),
            token: None,
            ip: Some(ip),
            data: SessionData {
                authorities: role,
                email: Some(account.email),
                first_name: Some(account.first_name),
                last_name: Some(account.last_name),
            },
        }
    }
}
