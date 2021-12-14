use actix_web::{web::Data, FromRequest};
use std::{
    future::{ready, Ready},
    ops::Deref,
    sync::Arc,
};
use tracing::instrument;
use validator::Validate;

use crate::{
    app::{error::AppError, state::AppState},
    common::{
        models::{Pagination, Role},
        services::CryptoService,
    },
};

use super::{
    models::{
        Account, AccountId, DTOCreateAccount, DTOGetAccounts, DTOUpdateAccount, ResponseGetAccounts,
    },
    repository::AccountRepository,
};

pub struct AccountService {
    repository: Arc<AccountRepository>,
    crypto_service: Arc<CryptoService>,
}

impl AccountService {
    pub fn new(repository: Arc<AccountRepository>, crypto_service: Arc<CryptoService>) -> Self {
        Self {
            repository,
            crypto_service,
        }
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

    pub async fn get_account_by_id(&self, account_id: AccountId) -> Result<Account, AppError> {
        Ok(self.repository.get_by_id(account_id).await?)
    }

    pub async fn get_account_by_email(&self, email: String) -> Result<Account, AppError> {
        Ok(self.repository.get_by_email(email).await?)
    }

    pub async fn get_role(&self, account_id: AccountId) -> Result<Role, AppError> {
        Ok(self.repository.get_role(account_id).await?)
    }

    pub async fn delete_account(&self, account_id: AccountId) -> Result<(), AppError> {
        self.repository.get_by_id(account_id).await?;
        self.repository.delete(account_id).await?;
        Ok(())
    }
    pub async fn create_account(&self, mut dto: DTOCreateAccount) -> Result<Account, AppError> {
        match dto.validate() {
            Ok(_) => Ok(()),
            Err(errors) => {
                let error_map = errors.field_errors();

                let message = if error_map.contains_key("first_name") {
                    format!("Invalid first name. \"{}\" is too short.", dto.first_name)
                } else if error_map.contains_key("last_name") {
                    format!("Invalid last name. \"{}\" is too short.", dto.last_name)
                } else if error_map.contains_key("hash_password") {
                    format!("Invalid password. \"{}\"", dto.hash_password)
                } else if error_map.contains_key("email") {
                    format!("Invalid email address \"{}\"", dto.email)
                } else {
                    "Invalid input.".to_string()
                };

                Err(AppError::BAD_REQUEST.message(message))
            }
        }?;
        let account = self.repository.get_by_email(dto.email.clone()).await;
        if let Ok(account) = account {
            return Err(
                AppError::BAD_REQUEST.message(format!("Такой email({}), уже занят", account.email))
            );
        };
        dto.hash_password = self.crypto_service.hash_password(dto.hash_password).await?;
        Ok(self.repository.create(dto).await?)
    }
    pub async fn update_account(
        &self,
        account_id: AccountId,
        dto: DTOUpdateAccount,
    ) -> Result<Account, AppError> {
        if let (None, None, None, None) = (
            dto.last_name.clone(),
            dto.first_name.clone(),
            dto.email.clone(),
            dto.role.clone(),
        ) {
            return Err(AppError::BAD_REQUEST
                .message("Нужно указать хотя-бы одно поле для изменения".to_string()));
        };
        match dto.validate() {
            Ok(_) => Ok(()),
            Err(errors) => {
                let error_map = errors.field_errors();

                let message = if error_map.contains_key("first_name") {
                    format!(
                        "Invalid first name. \"{}\" is too short.",
                        dto.first_name.as_ref().unwrap()
                    )
                } else if error_map.contains_key("last_name") {
                    format!(
                        "Invalid last name. \"{}\" is too short.",
                        dto.last_name.as_ref().unwrap()
                    )
                } else if error_map.contains_key("email") {
                    format!("Invalid email address \"{}\"", dto.email.as_ref().unwrap())
                } else {
                    "Invalid input.".to_string()
                };

                Err(AppError::BAD_REQUEST.message(message))
            }
        }?;
        if let Some(email) = dto.email.clone() {
            let account = self.repository.get_by_email(email).await;
            if let Ok(account) = account {
                return Err(AppError::BAD_REQUEST
                    .message(format!("Такой email({}), уже занят", account.email)));
            };
        };
        let account = self.repository.update(account_id, dto).await?;
        Ok(account)
    }
    pub async fn ban_account(&self, _account_id: AccountId) -> Result<Account, AppError> {
        Ok(Account::default())
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
        let crypto_service_result = Data::<CryptoService>::from_request(req, payload).into_inner();
        match app_state_result {
            Ok(app_state) => match crypto_service_result {
                Ok(crypto_service) => {
                    let pool = app_state.deref().pool.clone();
                    let repo = AccountRepository::new(Arc::new(pool));
                    let crypto_service = crypto_service.deref().clone();
                    ready(Ok(AccountService::new(Arc::new(repo), crypto_service)))
                }
                Err(e) => ready(Err(AppError::DB_ERROR.default().with_cause(format!(
                    "[CryproService] Initialization error: {}",
                    e.to_string()
                )))),
            },
            Err(e) => ready(Err(AppError::DB_ERROR.default().with_cause(format!(
                "[AccountService] Initialization error: {}",
                e.to_string()
            )))),
        }
    }
}
