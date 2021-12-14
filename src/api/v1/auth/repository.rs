use actix_web::{web::Data, FromRequest};
use sqlx::{Pool, Postgres};
use std::{
    future::{ready, Ready},
    ops::Deref,
    sync::Arc,
};
use tracing::instrument;

use crate::app::{error::AppError, state::AppState};

pub struct AuthRepository {
    pool: Arc<Pool<Postgres>>,
}

impl AuthRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
}

impl FromRequest for AuthRepository {
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
                ready(Ok(AuthRepository::new(Arc::new(pool))))
            }
            _ => ready(Err(AppError::UNAUTHORIZED.default())),
        }
    }
}
