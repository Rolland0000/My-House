use async_trait::async_trait;
use std::hash::Hash;

use super::moka::MokaStore;

/// Key/value cache capability — decouples callers from the backing engine
/// (moka, Redis, ...). Swapping engines only touches this file's impl block
/// and the composition root that builds it (`main.rs`).
#[async_trait]
pub trait AppCacheProvider<K, V>: Send + Sync
where
    K: Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    async fn get(&self, key: &K) -> Option<V>;
    async fn insert(&self, key: K, value: V);
    async fn invalidate(&self, key: &K);
}

#[async_trait]
impl<K, V> AppCacheProvider<K, V> for MokaStore<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    async fn get(&self, key: &K) -> Option<V> {
        self.inner.get(key).await
    }

    async fn insert(&self, key: K, value: V) {
        self.inner.insert(key, value).await;
    }

    async fn invalidate(&self, key: &K) {
        self.inner.invalidate(key).await;
    }
}
