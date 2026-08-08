//! `StorageProvider` — the abstraction every module writing files (media, avatars,
//! owner-request documents) depends on instead of filesystem specifics.
//!
//! Key naming convention used by callers:
//!   - `listings/{listing_id}/{uuid}.{ext}`       — public, served statically by nginx
//!   - `avatars/{user_id}/{uuid}.{ext}`           — public, served statically by nginx
//!   - `owner-requests/{request_id}/{uuid}.{ext}` — private, never exposed statically,
//!     read only via [`StorageProvider::read`] (admin-only)

use async_trait::async_trait;
use bytes::Bytes;
use std::time::Duration;

use crate::shared::errors::AppError;

/// Storage backend abstraction (TECHNICAL_SPEC_MVP.md §3.1). `LocalFsStorage` (MVP) and
/// `AwsS3Storage` (V2) are the two intended implementations, selected at startup by
/// [`super::build_storage_provider`].
#[async_trait]
pub trait StorageProvider: Send + Sync {
    /// Uploads `data` under `key` and returns the resulting URL (public prefixes) or
    /// the stored key itself (private prefixes such as `owner-requests/*`).
    async fn upload(&self, key: &str, data: Bytes, content_type: &str) -> Result<String, AppError>;

    /// Reads the raw bytes stored under `key`. Required for private documents
    /// (`owner-requests/*`) that are never served statically.
    async fn read(&self, key: &str) -> Result<Bytes, AppError>;

    /// Deletes the object stored under `key`.
    async fn delete(&self, key: &str) -> Result<(), AppError>;

    /// Returns a time-limited URL for `key`, valid for `expires_in`.
    ///
    /// Kept on the trait for interface compatibility with the V2 S3 backend even
    /// though no MVP implementation uses it — do not drop it "to simplify."
    async fn presigned_url(&self, key: &str, expires_in: Duration) -> Result<String, AppError>;
}

/// Placeholder implementation used by [`super::build_storage_provider`] until a real
/// backend (`LocalFsStorage` — MH-26, `AwsS3Storage` — V2) is implemented. Keeps the
/// crate compiling against the finalized trait boundary independently of those tickets.
pub(super) struct UnimplementedStorage;

#[async_trait]
impl StorageProvider for UnimplementedStorage {
    async fn upload(
        &self,
        _key: &str,
        _data: Bytes,
        _content_type: &str,
    ) -> Result<String, AppError> {
        todo!("StorageProvider implementation lands in MH-26 (LocalFsStorage) / V2 (AwsS3Storage)")
    }

    async fn read(&self, _key: &str) -> Result<Bytes, AppError> {
        todo!("StorageProvider implementation lands in MH-26 (LocalFsStorage) / V2 (AwsS3Storage)")
    }

    async fn delete(&self, _key: &str) -> Result<(), AppError> {
        todo!("StorageProvider implementation lands in MH-26 (LocalFsStorage) / V2 (AwsS3Storage)")
    }

    async fn presigned_url(&self, _key: &str, _expires_in: Duration) -> Result<String, AppError> {
        todo!("StorageProvider implementation lands in MH-26 (LocalFsStorage) / V2 (AwsS3Storage)")
    }
}
