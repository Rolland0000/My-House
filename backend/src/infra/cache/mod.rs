mod moka;
mod provider;

pub use provider::AppCacheProvider;

use std::sync::Arc;
use std::time::Duration;

use uuid::Uuid;

use moka::MokaStore;

use crate::shared::types::RefreshTokenId;

/// Staleness bound on an admin suspension taking effect (MH-32).
const IS_ACTIVE_STATUS_TTL: Duration = Duration::from_secs(8);

/// Grace window for concurrent double-refresh (MH-36) — a cache hit means
/// the reuse is within the window, by construction of the TTL.
const REFRESH_REUSE_GRACE_WINDOW: Duration = Duration::from_secs(5);

/// Engine choice, made once at startup by the caller (`main.rs`).
pub fn build_cache_provider() -> Arc<dyn AppCacheProvider<Uuid, bool>> {
    Arc::new(MokaStore::new(IS_ACTIVE_STATUS_TTL))
}

/// Keyed by the just-revoked row's id, valued with the raw new
/// refresh-token string — only recoverable here, since only its hash is
/// persisted to `refresh_tokens`.
pub fn build_refresh_replay_cache_provider() -> Arc<dyn AppCacheProvider<RefreshTokenId, String>> {
    Arc::new(MokaStore::new(REFRESH_REUSE_GRACE_WINDOW))
}

/// Single point of entry for every in-memory cache the app keeps.
pub struct AppCache {
    is_active_status: Arc<dyn AppCacheProvider<Uuid, bool>>,
    refresh_replay: Arc<dyn AppCacheProvider<RefreshTokenId, String>>,
}

impl AppCache {
    pub fn new(
        is_active_status: Arc<dyn AppCacheProvider<Uuid, bool>>,
        refresh_replay: Arc<dyn AppCacheProvider<RefreshTokenId, String>>,
    ) -> Self {
        Self {
            is_active_status,
            refresh_replay,
        }
    }

    pub fn is_active_status(&self) -> Arc<dyn AppCacheProvider<Uuid, bool>> {
        Arc::clone(&self.is_active_status)
    }

    pub fn refresh_replay(&self) -> Arc<dyn AppCacheProvider<RefreshTokenId, String>> {
        Arc::clone(&self.refresh_replay)
    }
}
