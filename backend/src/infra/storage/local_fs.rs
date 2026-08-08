//! `LocalFsStorage` — MVP `StorageProvider` backend writing under `LOCAL_STORAGE_PATH`
//! (TECHNICAL_SPEC_MVP.md §3.1, ARCHITECTURE.md §7.3).
//!
//! Public-prefixed keys (`listings/*`, `avatars/*`) are served directly by nginx from
//! the same volume; `upload()` returns the computed public URL for those. Private keys
//! (`owner-requests/*`) are never exposed statically — `upload()` returns only the
//! storage key, and reading them back requires the authenticated admin document
//! endpoint (`StorageProvider::read`), never the filesystem directly.

use std::path::{Path, PathBuf};

use async_trait::async_trait;
use bytes::Bytes;
use tokio::fs;

use crate::shared::errors::AppError;

use super::provider::StorageProvider;

const OWNER_REQUESTS_PREFIX: &str = "owner-requests/";

pub struct LocalFsStorage {
    root: PathBuf,
    public_media_base_url: String,
}

impl LocalFsStorage {
    pub fn new(
        local_storage_path: impl Into<PathBuf>,
        public_media_base_url: impl Into<String>,
    ) -> Self {
        Self {
            root: local_storage_path.into(),
            public_media_base_url: public_media_base_url.into(),
        }
    }

    /// Resolves `key` to an absolute path under the storage root.
    fn resolve(&self, key: &str) -> PathBuf {
        self.root.join(key)
    }

    /// Builds the public URL for a public-prefixed key.
    fn public_url(&self, key: &str) -> String {
        format!(
            "{}/{}",
            self.public_media_base_url.trim_end_matches('/'),
            key.trim_start_matches('/')
        )
    }
}

#[async_trait]
impl StorageProvider for LocalFsStorage {
    async fn upload(
        &self,
        key: &str,
        data: Bytes,
        _content_type: &str,
    ) -> Result<String, AppError> {
        let path = self.resolve(key);
        if let Some(parent) = Path::new(&path).parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(&path, &data).await?;

        if key.starts_with(OWNER_REQUESTS_PREFIX) {
            Ok(key.to_string())
        } else {
            Ok(self.public_url(key))
        }
    }

    async fn read(&self, key: &str) -> Result<Bytes, AppError> {
        let path = self.resolve(key);
        let data = fs::read(&path).await?;
        Ok(Bytes::from(data))
    }

    async fn delete(&self, key: &str) -> Result<(), AppError> {
        let path = self.resolve(key);
        fs::remove_file(&path).await.map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                AppError::Storage(format!("cannot delete: key not found: {key}"))
            } else {
                AppError::from(error)
            }
        })
    }

    async fn presigned_url(
        &self,
        _key: &str,
        _expires_in: std::time::Duration,
    ) -> Result<String, AppError> {
        Err(AppError::Storage(
            "presigned_url is not implemented for LocalFsStorage (MVP)".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn storage(root: &Path) -> LocalFsStorage {
        LocalFsStorage::new(root.to_path_buf(), "http://localhost/media")
    }

    #[tokio::test]
    async fn upload_on_public_prefix_returns_computed_url() {
        let dir = tempdir();
        let store = storage(dir.path());
        let key = "listings/11111111-1111-1111-1111-111111111111/photo.jpg";

        let result = store
            .upload(key, Bytes::from_static(b"data"), "image/jpeg")
            .await
            .expect("upload should succeed");

        assert_eq!(result, format!("http://localhost/media/{key}"));
    }

    #[tokio::test]
    async fn upload_on_owner_requests_prefix_returns_key_only() {
        let dir = tempdir();
        let store = storage(dir.path());
        let key = "owner-requests/22222222-2222-2222-2222-222222222222/id-card.png";

        let result = store
            .upload(key, Bytes::from_static(b"data"), "image/png")
            .await
            .expect("upload should succeed");

        assert_eq!(result, key);
    }

    #[tokio::test]
    async fn read_returns_previously_uploaded_bytes() {
        let dir = tempdir();
        let store = storage(dir.path());
        let key = "avatars/33333333-3333-3333-3333-333333333333/avatar.png";
        store
            .upload(key, Bytes::from_static(b"avatar-bytes"), "image/png")
            .await
            .expect("upload should succeed");

        let data = store.read(key).await.expect("read should succeed");

        assert_eq!(data, Bytes::from_static(b"avatar-bytes"));
    }

    #[tokio::test]
    async fn delete_removes_existing_file() {
        let dir = tempdir();
        let store = storage(dir.path());
        let key = "listings/44444444-4444-4444-4444-444444444444/photo.jpg";
        store
            .upload(key, Bytes::from_static(b"data"), "image/jpeg")
            .await
            .expect("upload should succeed");

        store.delete(key).await.expect("delete should succeed");

        assert!(store.read(key).await.is_err());
    }

    #[tokio::test]
    async fn delete_on_missing_key_returns_typed_error_not_panic() {
        let dir = tempdir();
        let store = storage(dir.path());

        let result = store.delete("listings/does-not-exist/photo.jpg").await;

        assert!(matches!(result, Err(AppError::Storage(_))));
    }

    #[tokio::test]
    async fn presigned_url_returns_not_implemented_error() {
        let dir = tempdir();
        let store = storage(dir.path());

        let result = store
            .presigned_url("listings/x/photo.jpg", std::time::Duration::from_secs(60))
            .await;

        assert!(matches!(result, Err(AppError::Storage(_))));
    }

    /// Minimal temp-dir helper (avoids pulling in a `tempfile` dependency for
    /// four lines of test-only cleanup logic).
    struct TempDir(PathBuf);

    impl TempDir {
        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn tempdir() -> TempDir {
        let dir =
            std::env::temp_dir().join(format!("myhouse-storage-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).expect("failed to create temp dir for test");
        TempDir(dir)
    }
}
