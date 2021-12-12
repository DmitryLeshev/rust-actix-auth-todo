mod api;
mod app;
mod common;
mod config;
mod session;

use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc, RwLock,
};

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    error,
    web::{self},
    App, HttpResponse, HttpServer,
};

use crate::app::error::AppErrorResponse;
use crate::{
    app::state::AppState,
    common::{handlers::*, services::*},
    config::AppConfig,
    session::Sessions,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::init();
    let address = config.server.get_address();
    let domain = format!("http://{}", address);
    tracing::info!("The server is running on {}", domain);
    let pool = config.db.pg.db_pool().await;
    let thread_counter = Arc::new(AtomicU16::new(1));

    let hashing = CryptoService {
        key: Arc::new("TEST".to_string()),
    };

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
                sessions: RwLock::new(Sessions::default()),
                thread_id: thread_index,
                domain: domain.clone(),
                version: 1,
            }))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("test")
                    .secure(false),
            ))
            .wrap(Cors::permissive())
            .route("/", web::get().to(greet))
            .route("/test", web::get().to(test_response))
            .route("/test/{name}", web::get().to(test_response))
            .route("/check-session", web::get().to(check_session_user))
            .route("/create-session", web::get().to(create_session))
            .configure(api::app_routing)
    })
    .workers(config.server.workers)
    .bind(address)?
    .run()
    .await
}
