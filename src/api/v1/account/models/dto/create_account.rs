use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DTOCreateAccount {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub hash_password: String,
}