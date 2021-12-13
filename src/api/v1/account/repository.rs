use actix_web::{web::Data, FromRequest};
use sqlx::{Pool, Postgres};
use std::{
    collections::HashMap,
    future::{ready, Ready},
    ops::Deref,
    sync::Arc,
};
use tracing::instrument;

use crate::{
    app::{error::AppError, state::AppState},
    common::models::{Role, RowCount},
};

use super::models::{Account, AccountId, DTOCreateAccount, DTOUpdateAccount};

pub struct AccountRepository {
    pool: Arc<Pool<Postgres>>,
}

impl AccountRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }

    pub async fn get(
        &self,
        limit: i64,
        offset: i64,
        email: String,
    ) -> Result<(Vec<Account>, i64), AppError> {
        sqlx::query("drop table if exists tmp_account cascade")
            .execute(&*self.pool)
            .await?;

        let sql = format!(
            "select * into tmp_account from account where email like '%{}%'",
            email
        );
        sqlx::query(&sql).execute(&*self.pool).await?;

        let sql = format!(
            "select * from tmp_account offset {} limit {}",
            offset, limit
        );
        let accounts = sqlx::query_as::<_, Account>(&sql)
            .fetch_all(&*self.pool)
            .await?;

        let row_count_in_result = sqlx::query_as::<_, RowCount>("select count(*) from tmp_account")
            .fetch_one(&*self.pool)
            .await?;

        sqlx::query("drop table if exists tmp_account cascade")
            .execute(&*self.pool)
            .await?;

        Ok((accounts, row_count_in_result.count))
    }

    pub async fn _get_all(&self) -> Result<Vec<Account>, AppError> {
        let accounts = sqlx::query_as::<_, Account>("select * from account")
            .fetch_all(&*self.pool)
            .await?;
        Ok(accounts)
    }

    pub async fn get_by_id(&self, account_id: AccountId) -> Result<Account, AppError> {
        let sql = format!("select * from account where account_id = {}", account_id);
        let account = sqlx::query_as::<_, Account>(&sql)
            .fetch_one(&*self.pool)
            .await;
        match account {
            Ok(account) => Ok(account),
            Err(e) => Err(AppError::NOT_FOUND
                .message(format!("Аккаунта с id = {} не найден!", account_id))
                .with_cause(e.to_string())),
        }
    }

    pub async fn get_by_email(&self, email: String) -> Result<Account, AppError> {
        let sql = format!("select * from account where email = '{}'", email);
        let account = sqlx::query_as::<_, Account>(&sql)
            .fetch_one(&*self.pool)
            .await?;
        Ok(account)
    }

    pub async fn create(&self, dto: DTOCreateAccount) -> Result<Account, AppError> {
        let sql = format!("insert into account (email, first_name, last_name, hash_password) values ('{}', '{}', '{}', '{}') returning *", dto.email, dto.first_name, dto.last_name, dto.hash_password);
        let account = sqlx::query_as::<_, Account>(&sql)
            .fetch_one(&*self.pool)
            .await?;
        if let Some(role) = dto.role {
            self.add_role(account.account_id, role).await?;
        };
        // let sql = format!(
        //     "insert into account_role (account_id, role_id) values ({}, {})",
        //     account.account_id, role_id
        // );
        // sqlx::query(&sql).execute(&*self.pool).await?;
        Ok(account)
    }

    pub async fn delete(&self, account_id: AccountId) -> Result<(), AppError> {
        let sql = format!("delete from account where account_id = {}", account_id);
        sqlx::query(&sql).execute(&*self.pool).await?;
        Ok(())
    }

    pub async fn update(
        &self,
        account_id: AccountId,
        dto: DTOUpdateAccount,
    ) -> Result<Account, AppError> {
        let sql = {
            let mut hash_map = HashMap::new();
            if let Some(email) = dto.email {
                hash_map.insert("email", email);
            };
            if let Some(first_name) = dto.first_name {
                hash_map.insert("first_name", first_name);
            };
            if let Some(last_name) = dto.last_name {
                hash_map.insert("last_name", last_name);
            };
            hash_map
                .iter()
                .map(|(key, value)| format!("{} = '{}'", key, value))
                .collect::<Vec<String>>()
                .join(", ")
        };
        let sql = format!(
            "update account set {} where account_id = {} returning *",
            sql, account_id
        );
        let account = sqlx::query_as::<_, Account>(&sql)
            .fetch_one(&*self.pool)
            .await?;
        if let Some(role) = dto.role {
            self.update_role(account_id, role).await?
        };
        Ok(account)
    }

    pub async fn add_role(&self, account_id: AccountId, role: Role) -> Result<(), AppError> {
        let sql = format!(
            "insert into account_role (account_id, role_id) values ({}, {})",
            account_id,
            role.get_role_id()
        );
        sqlx::query(&sql).execute(&*self.pool).await?;
        Ok(())
    }

    pub async fn update_role(&self, account_id: AccountId, role: Role) -> Result<(), AppError> {
        let sql = format!(
            "update account_role set role_id = {} where account_id = {}",
            role.get_role_id(),
            account_id,
        );
        sqlx::query(&sql).execute(&*self.pool).await?;
        Ok(())
    }

    pub async fn _ban_account(&self) -> Result<Vec<Account>, AppError> {
        let accounts = sqlx::query_as::<_, Account>("select * from account")
            .fetch_all(&*self.pool)
            .await?;
        Ok(accounts)
    }
}

impl FromRequest for AccountRepository {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    #[instrument(skip(req, payload))]
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let app_state_result = Data::<AppState>::from_request(req, payload).into_inner();

        match app_state_result {
            Ok(app_state) => {
                let pool = app_state.deref().pool.clone();
                ready(Ok(AccountRepository::new(Arc::new(pool))))
            }
            _ => ready(Err(AppError::UNAUTHORIZED.default())),
        }
    }
}
