mod cleanup;
mod local_fs;
mod provider;
#[cfg(test)]
pub(crate) mod test_support;

pub use cleanup::delete_storage_objects;
pub use local_fs::LocalFsStorage;
pub use provider::StorageProvider;

use std::sync::Arc;

use crate::config::{AppConfig, StorageProvider as StorageProviderKind};

/// Selects and constructs the `StorageProvider` implementation configured via
/// `STORAGE_PROVIDER`. Unknown or unimplemented values are rejected at startup
/// by `AppConfig::from_env`, so only implemented providers reach this match.
pub fn build_storage_provider(config: &AppConfig) -> Arc<dyn StorageProvider> {
    match config.storage.provider {
        StorageProviderKind::Local => Arc::new(LocalFsStorage::new(
            config.storage.local_path.clone(),
            config.storage.public_media_base_url.clone(),
        )),
    }
}
