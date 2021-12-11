use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DTOUpdateAccount {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
