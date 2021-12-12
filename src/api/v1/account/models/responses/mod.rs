mod create_account;
mod get_account_by_id;
mod get_accounts;
mod update_account;

pub use create_account::*;
pub use get_account_by_id::*;
pub use get_accounts::*;
pub use update_account::*;

use serde::Serialize;
#[derive(Debug, Serialize, Clone)]
pub struct ResponseAccount;
