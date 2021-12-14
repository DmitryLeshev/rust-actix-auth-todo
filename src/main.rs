mod api;
mod app;
mod common;
mod config;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};

use actix_cors::Cors;
use actix_web::{
    error,
    web::{self},
    App, HttpResponse, HttpServer,
};

use crate::app::error::AppErrorResponse;
use crate::{app::state::AppState, common::services::*, config::AppConfig};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::init();
    let address = config.server.get_address();
    let domain = format!("http://{}", address);
    tracing::info!("The server is running on {}", domain);
    let pool = config.db.pg.db_pool().await;
    let thread_counter = Arc::new(AtomicU16::new(1));

    let hashing = CryptoService::new("QWERTY".to_string());
    HttpServer::new(move || {
        let thread_index = thread_counter.fetch_add(1, Ordering::SeqCst);
        tracing::debug!("Starting thread {}", thread_index);

        let json_cfg = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                let error = err.to_string();
                error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest()
                        .json(AppErrorResponse {
                            code: 400,
                            error,
                            status: app::response::ResponseStatus::Fail,
                        })
                        .into(),
                )
                .into()
            });

        App::new()
            .app_data(json_cfg)
            .app_data(web::Data::new(hashing.clone()))
            .app_data(web::Data::new(AppState {
                pool: pool.clone(),
                thread_id: thread_index,
                domain: domain.clone(),
                version: 1,
            }))
            .wrap(
                CookieSession::signed(&[0; 32])
                    .name("CookieSession")
                    .secure(false),
            )
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("IdentityServiceCookieIdentityPolicy")
                    .secure(false),
            ))
            .wrap(Cors::permissive())
            .configure(api::app_routing)
    })
    .workers(config.server.workers)
    .bind(address)?
    .run()
    .await
}
