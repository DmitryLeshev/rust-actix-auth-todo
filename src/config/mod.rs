mod db;
mod server;
mod tracing;

use config::ConfigError;
use serde::Deserialize;

use self::{db::DataBaseConfig, server::ServerConfig, tracing::TracingConfig};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub tracing: TracingConfig,
    pub db: DataBaseConfig,
}

impl AppConfig {
    pub fn init() -> Self {
        dotenv::dotenv().ok();
        let config = Self::from_env().unwrap();
        config.tracing.init();
        config
    }
    fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
