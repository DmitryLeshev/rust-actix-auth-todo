pub mod account;
pub mod auth;

use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

async fn api_methods() -> HttpResponse {
    HttpResponse::Ok().body("api_methods")
}

pub fn v1_routing(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("", web::get().to(api_methods))
            .configure(account::routing)
            .configure(auth::routing),
    );
}
