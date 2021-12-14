use sqlx::PgPool;

#[derive(Debug)]
pub struct AppState {
    pub pool: PgPool,
    pub thread_id: u16,
    pub domain: String,
    pub version: u8,
}
