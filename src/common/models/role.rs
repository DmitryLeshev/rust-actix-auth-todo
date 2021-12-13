use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Guest,
    User,
    Admin,
}

impl Default for Role {
    fn default() -> Self {
        Role::Guest
    }
}

impl Role {
    pub fn get_role_id(&self) -> i64 {
        match self {
            Role::Admin => 1,
            Role::User => 2,
            Role::Guest => 3,
        }
    }
}
