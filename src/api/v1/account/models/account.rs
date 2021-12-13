use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};

use super::DTOCreateAccount;

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

impl Default for Account {
    fn default() -> Self {
        Self {
            account_id: 123456,
            email: "email".to_string(),
            first_name: "first_name".to_string(),
            last_name: "last_name".to_string(),
            hash_password: "hash_password".to_string(),
            created_at: NaiveDateTime::new(
                NaiveDate::from_ymd(2021, 3, 14),
                NaiveTime::from_hms(23, 56, 10),
            ),
            updated_at: NaiveDateTime::new(
                NaiveDate::from_ymd(2021, 3, 14),
                NaiveTime::from_hms(23, 56, 10),
            ),
        }
    }
}

impl Account {
    pub fn _new(dto: DTOCreateAccount) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            account_id: 12345,
            email: dto.email,
            first_name: dto.first_name,
            last_name: dto.last_name,
            hash_password: dto.hash_password,
            created_at: now,
            updated_at: now,
        }
    }
}
