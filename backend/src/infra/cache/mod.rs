mod moka;
mod provider;

pub use provider::AppCacheProvider;

use std::sync::Arc;
use std::time::Duration;

use uuid::Uuid;

use moka::MokaStore;

/// Only staleness bound on an admin suspension taking effect — no proactive
/// invalidation (MH-32, ARCHITECTURE.md §8.1).
const IS_ACTIVE_STATUS_TTL: Duration = Duration::from_secs(8);

/// Selects and constructs the cache engine backing `AppCache`. Single choice
/// point, made at startup by the caller (`main.rs`) — mirrors
/// `storage::build_storage_provider`.
pub fn build_cache_provider() -> Arc<dyn AppCacheProvider<Uuid, bool>> {
    Arc::new(MokaStore::new(IS_ACTIVE_STATUS_TTL))
}

/// Single point of entry for every in-memory cache the app keeps. Fields are
/// typed by `AppCacheProvider`, not the concrete engine.
pub struct AppCache {
    is_active_status: Arc<dyn AppCacheProvider<Uuid, bool>>,
}

impl AppCache {
    pub fn new(cache_provider: Arc<dyn AppCacheProvider<Uuid, bool>>) -> Self {
        Self {
            is_active_status: cache_provider,
        }
    }

    pub fn is_active_status(&self) -> &dyn AppCacheProvider<Uuid, bool> {
        &*self.is_active_status
    }
}
