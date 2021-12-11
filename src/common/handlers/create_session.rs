use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};

use crate::{
    app::state::AppState,
    session::{Role, SessionUser},
};

pub async fn create_session(app_state: web::Data<AppState>, identity: Identity) -> impl Responder {
    let session_user = SessionUser {
        id: "session_id".to_string(),
        email: None,
        first_name: None,
        last_name: None,
        authorities: Role::default(),
    };
    identity.remember(session_user.id.clone());
    app_state
        .sessions
        .write()
        .unwrap()
        .map
        .insert(session_user.id.clone(), session_user.clone());
    HttpResponse::Ok().json(session_user)
}
