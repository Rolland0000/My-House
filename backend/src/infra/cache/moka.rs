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
    /// `max_capacity` is mandatory: moka is unbounded by default, so a
    /// flooded key space would otherwise grow the cache without limit.
    pub fn new(ttl: Duration, max_capacity: u64) -> Self {
        Self {
            inner: Cache::builder()
                .max_capacity(max_capacity)
                .time_to_live(ttl)
                .build(),
        }
    }
}
