use actix_web::{web::Data, FromRequest};
use std::{
    future::{ready, Ready},
    ops::Deref,
    sync::Arc,
};
use tracing::instrument;
use validator::Validate;

use crate::{
    api::v1::account::{models::Account, repository::AccountRepository, service::AccountService},
    app::{error::AppError, state::AppState},
    common::{models::Role, services::CryptoService},
};

use super::{
    models::{DTORegistration, DTOSignIn},
    repository::AuthRepository,
};

pub struct AuthService {
    repository: Arc<AuthRepository>,
    account_service: Arc<AccountService>,
    crypto_service: Arc<CryptoService>,
}

impl AuthService {
    pub fn new(
        repository: Arc<AuthRepository>,
        account_service: Arc<AccountService>,
        crypto_service: Arc<CryptoService>,
    ) -> Self {
        Self {
            repository,
            account_service,
            crypto_service,
        }
    }

    pub async fn sign_in(&self, dto: DTOSignIn) -> Result<(Account, Role), AppError> {
        match dto.validate() {
            Ok(_) => Ok(()),
            Err(errors) => {
                let error_map = errors.field_errors();

                let message = if error_map.contains_key("email") {
                    format!("Invalid email address \"{}\"", dto.email)
                } else {
                    "Invalid input.".to_string()
                };

                Err(AppError::BAD_REQUEST.message(message))
            }
        }?;
        let account = self.account_service.get_account_by_email(dto.email).await?;
        let role = self.account_service.get_role(account.account_id).await?;
        let result = self
            .crypto_service
            .verify_password(&dto.password.unwrap(), &account.hash_password.clone())
            .await?;
        if result {
            Ok((account, role))
        } else {
            Err(AppError::BAD_REQUEST.message("Не верный пароль и емайл".to_string()))
        }
    }

    pub async fn _sign_out(&self) -> Result<(), AppError> {
        Ok(())
    }

    pub async fn registration(&self, dto: DTORegistration) -> Result<(Account, Role), AppError> {
        let account = self.account_service.create_account(dto).await?;
        let role = self.account_service.get_role(account.account_id).await?;
        Ok((account, role))
    }

    pub async fn _session_info(&self) -> Result<(), AppError> {
        Ok(())
    }
}

impl FromRequest for AuthService {
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
                    let account_repo = Arc::new(AccountRepository::new(Arc::new(pool.clone())));
                    let auth_repo = Arc::new(AuthRepository::new(Arc::new(pool)));

                    let crypto_service = crypto_service.deref().clone();
                    let account_service =
                        Arc::new(AccountService::new(account_repo, crypto_service.clone()));
                    let auth_service = AuthService::new(auth_repo, account_service, crypto_service);

                    ready(Ok(auth_service))
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
