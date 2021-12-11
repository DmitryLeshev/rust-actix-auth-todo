use std::sync::RwLock;

use sqlx::PgPool;

use crate::session::Sessions;

#[derive(Debug)]
pub struct AppState {
    pub pool: PgPool,
    pub sessions: RwLock<Sessions>,
    pub thread_id: u16,
    pub domain: String,
    pub version: u8,
}
