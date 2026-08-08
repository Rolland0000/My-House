pub mod provider;

pub use provider::StorageProvider;

use std::sync::Arc;

use provider::UnimplementedStorage;

use crate::config::{AppConfig, StorageProvider as StorageProviderKind};

/// Selects and constructs the `StorageProvider` implementation configured via
/// `STORAGE_PROVIDER`. Invalid values are already rejected at startup by
/// `AppConfig::from_env` — this function only has to handle the values the
/// config layer accepts (`local`, `s3`).
pub fn build_storage_provider(config: &AppConfig) -> Arc<dyn StorageProvider> {
    match config.storage_provider {
        StorageProviderKind::Local => Arc::new(UnimplementedStorage),
        StorageProviderKind::S3 => Arc::new(UnimplementedStorage),
    }
}
