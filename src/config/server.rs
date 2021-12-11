use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
    pub workers: usize,
}

impl ServerConfig {
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
