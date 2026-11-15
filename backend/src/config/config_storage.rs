//! File storage settings.

use super::config_error::ConfigError;
use super::env_reader::{require, require_parsed};

/// `STORAGE_PROVIDER` — selects the storage backend.
///
/// Only `local` is implemented in the MVP; `s3` is reserved for V2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageProvider {
    Local,
    S3,
}

impl StorageProvider {
    fn parse(raw: &str) -> Result<Self, String> {
        match raw {
            "local" => Ok(StorageProvider::Local),
            "s3" => Ok(StorageProvider::S3),
            other => Err(format!("expected \"local\" or \"s3\", got \"{other}\"")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// Which storage backend to use.
    pub provider: StorageProvider,
    /// Root path on the filesystem for `local` storage (e.g. `/app/storage`).
    pub local_path: String,
    /// Base URL used to build public media links (e.g. `http://localhost/media`).
    pub public_media_base_url: String,
}

impl StorageConfig {
    pub(super) fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            provider: require_parsed("STORAGE_PROVIDER", StorageProvider::parse)?,
            local_path: require("LOCAL_STORAGE_PATH")?,
            public_media_base_url: require("PUBLIC_MEDIA_BASE_URL")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use crate::config::test_support::{set_valid_env, ENV_LOCK};

    #[test]
    fn loads_every_storage_setting() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        let storage = StorageConfig::from_env().expect("should load without error");
        assert_eq!(storage.provider, StorageProvider::Local);
        assert_eq!(storage.local_path, "/app/storage");
        assert_eq!(storage.public_media_base_url, "http://localhost/media");
    }

    #[test]
    fn parses_both_known_providers() {
        assert_eq!(StorageProvider::parse("local"), Ok(StorageProvider::Local));
        assert_eq!(StorageProvider::parse("s3"), Ok(StorageProvider::S3));
    }

    #[test]
    fn rejects_unknown_storage_provider() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("STORAGE_PROVIDER", "gcs");
        let err = StorageConfig::from_env().expect_err("should fail on an unknown provider");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "STORAGE_PROVIDER"));
    }

    #[test]
    fn rejects_missing_local_storage_path() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::remove_var("LOCAL_STORAGE_PATH");
        let err = StorageConfig::from_env().expect_err("should fail on a missing path");
        assert!(matches!(err, ConfigError::Missing(key) if key == "LOCAL_STORAGE_PATH"));
    }
}
