mod handlers;
mod models;
mod repository;
mod service;

use actix_web::web::{self, ServiceConfig};
use handlers::*;

pub const PATH: &str = "/accounts";

pub fn routing(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope(PATH)
            .route("", web::get().to(get_accounts))
            .route("", web::post().to(create_account))
            .route("/{account_id}", web::get().to(get_account_by_id))
            .route("/{account_id}", web::delete().to(delete_account))
            .route("/{account_id}", web::put().to(update_account))
            .route("/ban/{account_id}", web::post().to(ban_account)),
    );
}
