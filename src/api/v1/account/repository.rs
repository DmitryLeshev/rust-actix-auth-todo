use actix_web::{web::Data, FromRequest};
use sqlx::{Pool, Postgres};
use std::{
    future::{ready, Ready},
    ops::Deref,
    sync::Arc,
};
use tracing::instrument;

use crate::{
    app::{error::AppError, state::AppState},
    common::models::RowCount,
};

use super::models::Account;

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

    pub async fn get_by_id(&self, account_id: i64) -> Result<Account, AppError> {
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
