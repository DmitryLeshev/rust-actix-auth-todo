mod handlers;
pub mod models;
pub mod repository;
pub mod service;

use actix_web::web::{self, ServiceConfig};
use handlers::*;

pub const PATH: &str = "/todolists";

pub fn routing(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope(PATH)
            .route("", web::get().to(get_todolists))
            .route("", web::post().to(create_todolist))
            .route("/{todolist_id}", web::put().to(update_todolist))
            .route("/{todolist_id}", web::delete().to(delete_todolist))
            .route("/{todolist_id}/todoitems", web::get().to(get_todoitems))
            .route("/{todolist_id}/todoitems", web::post().to(create_todoitem))
            .route(
                "/{todolist_id}/todoitems/{todoitem_id}",
                web::put().to(update_todoitem),
            )
            .route(
                "/{todolist_id}/todoitems/{todoitem_id}",
                web::delete().to(delete_todoitem),
            ),
    );
}
