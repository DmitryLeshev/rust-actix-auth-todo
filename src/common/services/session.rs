use actix_identity::Identity;
use actix_session::Session;
use actix_web::FromRequest;
use std::{
    future::{ready, Ready},
    net::IpAddr,
};
use tracing::instrument;

use crate::{app::error::AppError, common::models::SessionAccount};

pub struct SessionService {
    pub session: Session,
    pub identity: Identity,
    pub ip: IpAddr,
}

impl SessionService {
    fn new(session: Session, identity: Identity, ip: IpAddr) -> Self {
        Self {
            session,
            identity,
            ip,
        }
    }

    pub async fn _check_session() {
        todo!()
    }

    pub async fn _create_session() {
        todo!()
    }

    pub async fn _delete_session() {
        todo!()
    }

    pub async fn get_session_account(&self) -> Result<SessionAccount, AppError> {
        if let Some(identity) = self.identity.identity() {
            if let Some(data) = self.session.get::<SessionAccount>(&identity)? {
                return Ok(data);
            }
        }
        Err(AppError::UNAUTHORIZED.default())
    }

    pub async fn get_session_account_id(&self) -> Result<i64, AppError> {
        if let Some(identity) = self.identity.identity() {
            if let Some(data) = self.session.get::<SessionAccount>(&identity)? {
                if let Some(account_id) = data.account_id {
                    return Ok(account_id);
                }
            }
        }
        Err(AppError::UNAUTHORIZED.default())
    }
}

impl FromRequest for SessionService {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    #[instrument(skip(req, payload))]
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let session = Session::from_request(req, payload).into_inner();
        let identity = Identity::from_request(req, payload).into_inner();
        let ip = req.peer_addr().unwrap().ip();
        if let (Ok(session), Ok(identity)) = (session, identity) {
            let session_service = SessionService::new(session, identity, ip);
            ready(Ok(session_service))
        } else {
            ready(Err(AppError::UNAUTHORIZED.default()))
        }
    }
}
