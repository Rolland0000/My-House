use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::infra::cache::{AppCache, AppCacheProvider};
use crate::infra::mailer::Mailer;
use crate::infra::storage::StorageProvider;
use crate::shared::token_decoder::{TokenDecoder, UnimplementedTokenDecoder};

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
    /// Storage backend, chosen by the caller at startup (see `main.rs`).
    pub storage: Arc<dyn StorageProvider>,
    /// In-memory caches (e.g. `AuthUser`'s `is_active` recheck).
    pub cache: AppCache,
    /// JWT claims decoder — `UnimplementedTokenDecoder` until MH-35.
    pub token_decoder: Arc<dyn TokenDecoder>,
}

impl AppState {
    /// `storage` and `cache_provider` are engine choices selectable via
    /// config (`STORAGE_PROVIDER`, and eventually a cache equivalent) — the
    /// composition root (`main.rs`) builds them and passes the result in.
    /// `token_decoder` has no such choice to make yet (MH-35 ships the one
    /// real implementation) so it stays built internally.
    pub fn new(
        config: AppConfig,
        db: PgPool,
        mailer: Arc<Mailer>,
        storage: Arc<dyn StorageProvider>,
        cache_provider: Arc<dyn AppCacheProvider<Uuid, bool>>,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                config,
                db,
                mailer,
                storage,
                cache: AppCache::new(cache_provider),
                token_decoder: Arc::new(UnimplementedTokenDecoder),
            }),
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

    /// Exposes the configured storage backend.
    pub fn storage(&self) -> &Arc<dyn StorageProvider> {
        &self.inner.storage
    }

    /// Exposes the application's in-memory caches.
    pub fn cache(&self) -> &AppCache {
        &self.inner.cache
    }

    /// Exposes the JWT claims decoder.
    pub fn token_decoder(&self) -> &Arc<dyn TokenDecoder> {
        &self.inner.token_decoder
    }
}
