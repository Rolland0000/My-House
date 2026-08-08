use std::sync::Arc;

use sqlx::PgPool;

use crate::config::AppConfig;
use crate::infra::mailer::Mailer;

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
    /// PostgreSQL connection pool.
    pub db: PgPool,
    /// Outbound SMTP mailer, shared across every module that sends email.
    pub mailer: Arc<Mailer>,
    // TODO EP-02: pub storage: Arc<dyn StorageProvider>,
    // TODO EP-02: pub cache: moka::future::Cache<String, CachedOtp>,
}

impl AppState {
    pub fn new(config: AppConfig, db: PgPool, mailer: Arc<Mailer>) -> Self {
        Self {
            inner: Arc::new(Inner { config, db, mailer }),
        }
    }

    /// Exposes the application configuration.
    pub fn config(&self) -> &AppConfig {
        &self.inner.config
    }

    /// Exposes the database connection pool.
    pub fn db(&self) -> &PgPool {
        &self.inner.db
    }

    /// Exposes the outbound SMTP mailer.
    pub fn mailer(&self) -> &Arc<Mailer> {
        &self.inner.mailer
    }
}
