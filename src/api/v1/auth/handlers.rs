use actix_web::web;
use tracing::warn;

use crate::{
    app::{
        error::AppError,
        response::{AppResponse, ClientResponse},
    },
    common::{models::SessionUser, services::SessionService},
};

use super::{
    models::{DTORegistration, DTOSignIn},
    service::AuthService,
};

pub async fn sign_in(
    dto: web::Json<DTOSignIn>,
    service: AuthService,
    session_service: SessionService,
) -> AppResponse {
    if let Some(identity) = session_service.identity.identity() {
        if let Some(_) = session_service.session.get::<SessionUser>(&identity)? {
            let msg = String::from("Ты чё? Ты уже в системе");
            return Err(AppError::BAD_REQUEST.message(msg));
        }
    }
    let (account, role) = service.sign_in(dto.clone()).await?;
    let session_user = SessionUser::new(account.email.clone(), account, role, session_service.ip);
    let session_id = session_user.session_id.clone();

    session_service.identity.remember(session_id.clone());
    session_service
        .session
        .insert(session_id, session_user.clone())?;

    Ok(ClientResponse::<SessionUser>::build()
        .with_data(session_user)
        .send())
}

pub async fn sign_out(session_service: SessionService) -> AppResponse {
    if let Some(session_id) = session_service.identity.identity() {
        session_service.identity.forget();
        if let Some(user) = session_service.session.remove(&session_id) {
            warn!("logout user: {:?}", user);
            return Ok(ClientResponse::<SessionUser>::build()
                .with_message(format!("logout user: {:?}", user))
                .send());
        }
    }
    Err(AppError::UNAUTHORIZED.default())
}

pub async fn registration(
    dto: web::Json<DTORegistration>,
    service: AuthService,
    session_service: SessionService,
) -> AppResponse {
    if let Some(identity) = session_service.identity.identity() {
        if let Some(_) = session_service.session.get::<SessionUser>(&identity)? {
            let msg = String::from("Ты чё? Ты уже в системе");
            return Err(AppError::BAD_REQUEST.message(msg));
        }
    }
    let (account, role) = service.registration(dto.clone()).await?;
    let session_user = SessionUser::new(account.email.clone(), account, role, session_service.ip);
    let session_id = session_user.session_id.clone();

    session_service.identity.remember(session_id.clone());
    session_service
        .session
        .insert(session_id, session_user.clone())?;

    Ok(ClientResponse::<SessionUser>::build()
        .with_data(session_user)
        .send())
}

pub async fn session_info(session_service: SessionService) -> AppResponse {
    let identity = session_service.identity.identity();
    warn!("Identity: {:?}", identity);
    let map = session_service.session.entries();
    warn!("Map session: {:?}", map);
    for item in map.keys() {
        warn!("Key session: {:?}", item);
    }
    if let Some(identity) = session_service.identity.identity() {
        if let Some(data) = session_service.session.get::<SessionUser>(&identity)? {
            return Ok(ClientResponse::<SessionUser>::build()
                .with_data(data)
                .send());
        }
    }
    Err(AppError::UNAUTHORIZED.default())
}
