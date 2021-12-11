use actix_web::{HttpResponse, Responder};

use crate::session::SessionUser;

pub async fn check_session_user(user: SessionUser) -> impl Responder {
    HttpResponse::Ok().json(user)
}
