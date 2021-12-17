mod handlers;
pub mod models;
pub mod service;

use actix_web::web::{self, ServiceConfig};
use handlers::*;

use crate::common::handlers::create_session;

pub const PATH: &str = "/auth";

pub fn routing(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope(PATH)
            .route("/sign-in", web::post().to(sign_in))
            .route("/sign-out", web::get().to(sign_out))
            .route("/registration", web::post().to(registration))
            .route("/session-info", web::get().to(session_info))
            .route("/create-session", web::get().to(create_session)),
    );
}
