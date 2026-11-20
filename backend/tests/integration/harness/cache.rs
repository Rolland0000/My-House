#![cfg(feature = "integration")]

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use backend_my_house::infra::cache::AppCacheProvider;

/// In-memory cache whose expiry the test controls: `None` keeps entries forever.
pub struct TestCache<K, V> {
    ttl: Option<Duration>,
    entries: Mutex<HashMap<K, (V, Instant)>>,
}

impl<K, V> TestCache<K, V> {
    pub fn new(ttl: Option<Duration>) -> Arc<Self> {
        Arc::new(Self {
            ttl,
            entries: Mutex::new(HashMap::new()),
        })
    }

    fn is_live(&self, inserted_at: Instant) -> bool {
        self.ttl.is_none_or(|ttl| inserted_at.elapsed() < ttl)
    }
}

#[async_trait]
impl<K, V> AppCacheProvider<K, V> for TestCache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    async fn get(&self, key: &K) -> Option<V> {
        let entries = self.entries.lock().unwrap_or_else(PoisonError::into_inner);
        entries
            .get(key)
            .filter(|(_, inserted_at)| self.is_live(*inserted_at))
            .map(|(value, _)| value.clone())
    }

    async fn insert(&self, key: K, value: V) {
        let mut entries = self.entries.lock().unwrap_or_else(PoisonError::into_inner);
        entries.insert(key, (value, Instant::now()));
    }

    async fn invalidate(&self, key: &K) {
        let mut entries = self.entries.lock().unwrap_or_else(PoisonError::into_inner);
        entries.remove(key);
    }

    async fn get_or_insert(&self, key: K, value: V) -> V {
        let mut entries = self.entries.lock().unwrap_or_else(PoisonError::into_inner);
        match entries.get(&key) {
            Some((existing, inserted_at)) if self.is_live(*inserted_at) => existing.clone(),
            _ => {
                entries.insert(key, (value.clone(), Instant::now()));
                value
            }
        }
    }
}
