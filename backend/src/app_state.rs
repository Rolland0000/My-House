use std::sync::Arc;

use crate::config::AppConfig;

/// Shared application state injected into every Axum handler via `axum::extract::State`.
///
/// Wrap mutable / expensive resources (DB pool, config, storage client) in
/// `Arc` so that `.clone()` on `AppState` is cheap and `Send + Sync`.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    /// Application configuration, loaded once at startup.
    pub config: AppConfig,
    // TODO EP-02: pub db: sqlx::PgPool,
    // TODO EP-02: pub storage: Arc<dyn StorageProvider>,
    // TODO EP-02: pub cache: moka::future::Cache<String, CachedOtp>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(Inner { config }),
        }
    }

    /// Exposes the application configuration.
    pub fn config(&self) -> &AppConfig {
        &self.inner.config
    }
}
