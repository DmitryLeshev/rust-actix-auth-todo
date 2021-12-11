use actix_web::web::{self, ServiceConfig};

pub mod v1;

pub fn app_routing(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api").configure(v1::v1_routing));
}
