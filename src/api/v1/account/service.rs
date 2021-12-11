use actix_web::{web::Data, FromRequest};
use std::{
    future::{ready, Ready},
    ops::Deref,
    sync::Arc,
};
use tracing::instrument;

use crate::{
    app::{error::AppError, state::AppState},
    common::models::Pagination,
};

use super::{
    models::{Account, DTOGetAccounts, ResponseGetAccounts},
    repository::AccountRepository,
};

pub struct AccountService {
    repository: Arc<AccountRepository>,
}

impl AccountService {
    fn new(repository: Arc<AccountRepository>) -> Self {
        Self { repository }
    }
    pub async fn _get_all(&self) -> Result<Vec<Account>, AppError> {
        Ok(self.repository._get_all().await?)
    }

    pub async fn get_accounts(&self, dto: DTOGetAccounts) -> Result<ResponseGetAccounts, AppError> {
        let DTOGetAccounts {
            query,
            base_path,
            domain,
            version,
        } = dto;

        let def_limit = 10;
        let def_page = 1;
        let def_email = String::from("");

        let (limit, offset, email, page) = {
            let page = match query.page {
                Some(page) => page,
                None => def_page,
            };
            let (limit, offset) = match query.limit {
                Some(limit) => (limit, limit * page - limit),
                None => (def_limit, def_limit * page - def_limit),
            };
            let email = match query.email.clone() {
                Some(email) => email,
                None => def_email,
            };
            (limit, offset, email, page)
        };
        let (accounts, number_of_accounts) = self.repository.get(limit, offset, email).await?;
        let pagination = if accounts.len() > 0 {
            let base_link = format!("{}/api/v{}{}?", domain, version, base_path);
            Some(Pagination::new(number_of_accounts, limit, page).create_links(base_link))
        } else {
            None
        };
        let data = ResponseGetAccounts {
            pagination,
            items: accounts,
        };
        Ok(data)
    }

    pub async fn get_account_by_id(&self, account_id: i64) -> Result<Account, AppError> {
        Ok(self.repository.get_by_id(account_id).await?)
    }
}

impl FromRequest for AccountService {
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
                let repo = AccountRepository::new(Arc::new(pool));
                ready(Ok(AccountService::new(Arc::new(repo))))
            }
            Err(e) => ready(Err(AppError::DB_ERROR.default().with_cause(format!(
                "[AccountService] Initialization error: {}",
                e.to_string()
            )))),
        }
    }
}
