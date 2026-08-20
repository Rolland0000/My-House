use moka::future::Cache;
use std::hash::Hash;
use std::time::Duration;

/// `moka::future::Cache` behind `AppCacheProvider` — the MVP cache engine.
pub struct MokaStore<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub(in crate::infra::cache) inner: Cache<K, V>,
}

impl<K, V> MokaStore<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new(ttl: Duration) -> Self {
        Self {
            inner: Cache::builder().time_to_live(ttl).build(),
        }
    }
}
