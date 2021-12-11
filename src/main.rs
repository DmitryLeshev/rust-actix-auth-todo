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
    web::{self},
    App, HttpServer,
};

use crate::{app::state::AppState, common::handlers::*, config::AppConfig, session::Sessions};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::init();
    let address = config.server.get_address();
    let domain = format!("http://{}", address);
    tracing::info!("The server is running on {}", domain);
    let pool = config.db.pg.db_pool().await;
    let thread_counter = Arc::new(AtomicU16::new(1));

    HttpServer::new(move || {
        let thread_index = thread_counter.fetch_add(1, Ordering::SeqCst);
        tracing::debug!("Starting thread {}", thread_index);

        App::new()
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
