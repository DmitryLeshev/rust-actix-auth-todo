mod api;
mod app;
mod common;
mod config;

use crate::app::error::AppErrorResponse;
use crate::{app::state::AppState, common::services::*, config::AppConfig};
use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::cookie::SameSite;
use actix_web::http::header;
use actix_web::{
    error,
    web::{self},
    App, HttpResponse, HttpServer,
};

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};

const COOKIE_SESSION_NAME: &str = "session";
const IDENTITY_SERVICE_NAME: &str = "identity";
const ALLOWED_ORIGIN: &str = "http://localhost:3000";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::init();
    let address = config.server.get_address();
    let domain = format!("https://{}", address);
    tracing::info!("The server is running on {}", domain);
    let pool = config.db.pg.db_pool().await;
    let thread_counter = Arc::new(AtomicU16::new(1));

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

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

        let cors = Cors::permissive()
            .allowed_origin(ALLOWED_ORIGIN)
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600)
            .supports_credentials(); // Allow the cookie auth.

        App::new()
            .app_data(json_cfg)
            .app_data(web::Data::new(hashing.clone()))
            .app_data(web::Data::new(AppState {
                pool: pool.clone(),
                thread_id: thread_index,
                domain: domain.clone(),
                version: 1,
            }))
            .wrap(cors)
            .wrap(
                CookieSession::signed(&[0; 32])
                    .name(COOKIE_SESSION_NAME)
                    .path("/")
                    .max_age(600000)
                    .same_site(SameSite::None)
                    .http_only(true)
                    .secure(true),
            )
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name(IDENTITY_SERVICE_NAME)
                    .path("/")
                    .max_age_secs(60000)
                    .same_site(SameSite::None)
                    .http_only(true)
                    .secure(true),
            ))
            .configure(api::app_routing)
    })
    .workers(config.server.workers)
    // .bind(address)?
    .bind_openssl(address, builder)?
    .run()
    .await
}
