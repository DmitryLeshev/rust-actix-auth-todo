use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct DataBaseConfig {
    pub pg: PostgresConfig,
}

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    pub url: String,
}

impl PostgresConfig {
    pub async fn db_pool(&self) -> PgPool {
        info!("Creating postgres connection pool");
        PgPool::connect(&self.url).await.unwrap()
    }
}
