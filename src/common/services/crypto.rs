use argonautica::{Hasher, Verifier};
use std::sync::Arc;
use tracing::instrument;

use crate::app::error::AppError;

#[derive(Debug, Clone)]
pub struct CryptoService {
    pub key: Arc<String>,
}

impl CryptoService {
    pub fn new(key: String) -> Self {
        Self { key: Arc::new(key) }
    }

    #[instrument(skip(self, password), err)]
    pub async fn hash_password(&self, password: String) -> Result<String, AppError> {
        let hash = Hasher::default()
            .with_secret_key(&*self.key)
            .with_password(password)
            .hash()?;
        Ok(hash)
    }

    #[instrument(skip(self, password, password_hash))]
    pub async fn verify_password(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<bool, AppError> {
        let result = Verifier::default()
            .with_secret_key(&*self.key)
            .with_hash(password_hash)
            .with_password(password)
            .verify()?;

        Ok(result)
    }
}
