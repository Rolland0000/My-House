use std::sync::Mutex;
use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;

use super::StorageProvider;
use crate::shared::errors::AppError;

/// Records every key passed to `delete`; `fails` makes each deletion return an
/// error, standing in for a file already gone from disk.
pub(crate) struct RecordingStorage {
    deleted: Mutex<Vec<String>>,
    fails: bool,
}

impl RecordingStorage {
    pub(crate) fn new(fails: bool) -> Self {
        Self {
            deleted: Mutex::new(Vec::new()),
            fails,
        }
    }

    pub(crate) fn deleted_keys(&self) -> Vec<String> {
        self.deleted.lock().expect("lock poisoned").clone()
    }
}

#[async_trait]
impl StorageProvider for RecordingStorage {
    async fn upload(
        &self,
        key: &str,
        _data: Bytes,
        _content_type: &str,
    ) -> Result<String, AppError> {
        Ok(format!("http://localhost/media/{key}"))
    }

    async fn read(&self, _key: &str) -> Result<Bytes, AppError> {
        unimplemented!("not exercised by these tests")
    }

    async fn delete(&self, key: &str) -> Result<(), AppError> {
        self.deleted
            .lock()
            .expect("lock poisoned")
            .push(key.to_string());
        if self.fails {
            return Err(AppError::Storage("backend unavailable".to_string()));
        }
        Ok(())
    }

    async fn presigned_url(&self, _key: &str, _expires_in: Duration) -> Result<String, AppError> {
        unimplemented!("not exercised by these tests")
    }
}
