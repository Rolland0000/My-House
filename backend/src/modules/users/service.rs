use bytes::Bytes;
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::infra::storage::StorageProvider;
use crate::shared::errors::AppError;
use crate::shared::file_validation::{validate_image, MAX_IMAGE_SIZE_BYTES};
use crate::shared::storage_key::{avatar_key, avatar_key_from_url};
use crate::shared::types::UserId;
use crate::shared::validation::{optional_name, required_name, required_phone};

use super::dto::UpdateMeDto;
use super::model::UserRow;
use super::repository;

/// Startup-only entry point — called once from `main` before the HTTP server
/// starts accepting requests. There is no HTTP route that reaches this: the
/// admin role is never creatable through the API surface. No-op when
/// disabled or when an admin row already exists — never reconciles, merges,
/// or deletes.
pub async fn bootstrap_admin(pool: &PgPool, config: &AppConfig) -> Result<(), AppError> {
    if !config.admin_bootstrap_on_startup {
        return Ok(());
    }
    // Guaranteed `Some` when the flag is on — `AppConfig::from_env` rejects the
    // combination at startup.
    let Some(email) = config.admin_bootstrap_email.as_deref() else {
        return Ok(());
    };
    if repository::admin_exists(pool).await? {
        return Ok(());
    }
    if repository::upsert_admin(pool, email).await? {
        tracing::info!(email, "admin account bootstrapped");
    } else {
        tracing::warn!(email, "existing account promoted to admin by bootstrap");
    }
    Ok(())
}

pub async fn get_me(pool: &PgPool, user_id: Uuid) -> Result<UserRow, AppError> {
    repository::find_by_id(pool, user_id)
        .await?
        .ok_or(AppError::UserNotFound)
}

pub async fn update_me(
    pool: &PgPool,
    user_id: Uuid,
    payload: UpdateMeDto,
) -> Result<UserRow, AppError> {
    let first_name = optional_name(payload.first_name.as_deref(), "first_name")?;
    let last_name = required_name(&payload.last_name, "last_name")?;
    let phone = required_phone(&payload.phone)?;

    repository::update_profile(pool, user_id, first_name, last_name, phone)
        .await?
        .ok_or(AppError::UserNotFound)
}

/// Stores `bytes` as the caller's avatar and drops the file it replaces.
///
/// Ordered write-then-point-then-delete: a failure before the row is updated
/// leaves the account on its previous avatar, which is still on disk — never
/// on a dead URL.
pub async fn replace_avatar(
    pool: &PgPool,
    storage: &dyn StorageProvider,
    user_id: Uuid,
    bytes: Bytes,
) -> Result<UserRow, AppError> {
    let validated = validate_image(&bytes, MAX_IMAGE_SIZE_BYTES)?;

    let previous_avatar_url = repository::find_by_id(pool, user_id)
        .await?
        .ok_or(AppError::UserNotFound)?
        .avatar_url;

    let key = avatar_key(UserId::new(user_id), validated.extension);
    let avatar_url = storage.upload(&key, bytes, validated.content_type).await?;

    let updated = repository::update_avatar_url(pool, user_id, &avatar_url)
        .await?
        .ok_or(AppError::UserNotFound)?;

    delete_previous_avatar(storage, previous_avatar_url.as_deref(), user_id).await;

    Ok(updated)
}

/// Best-effort cleanup: the replacement is already stored and referenced, so a
/// failure here costs an orphaned file, never a broken profile.
async fn delete_previous_avatar(
    storage: &dyn StorageProvider,
    previous_url: Option<&str>,
    user_id: Uuid,
) {
    let Some(previous_url) = previous_url else {
        return;
    };
    let Some(key) = avatar_key_from_url(previous_url, UserId::new(user_id)) else {
        tracing::warn!(%user_id, "previous avatar URL yields no storage key; skipping delete");
        return;
    };
    if let Err(error) = storage.delete(&key).await {
        tracing::warn!(%user_id, key, error = %error, "failed to delete previous avatar");
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::time::Duration;

    use async_trait::async_trait;

    use super::*;

    /// Records the keys `delete_previous_avatar` asks for; `fails` makes every
    /// deletion return an error, standing in for a file already gone from disk.
    struct RecordingStorage {
        deleted: Mutex<Vec<String>>,
        fails: bool,
    }

    impl RecordingStorage {
        fn new(fails: bool) -> Self {
            Self {
                deleted: Mutex::new(Vec::new()),
                fails,
            }
        }

        fn deleted_keys(&self) -> Vec<String> {
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

        async fn presigned_url(
            &self,
            _key: &str,
            _expires_in: Duration,
        ) -> Result<String, AppError> {
            unimplemented!("not exercised by these tests")
        }
    }

    #[tokio::test]
    async fn deletes_the_key_behind_the_previous_avatar_url() {
        let storage = RecordingStorage::new(false);
        let user_id = Uuid::new_v4();
        let key = avatar_key(UserId::new(user_id), "png");
        let url = format!("http://localhost/media/{key}");

        delete_previous_avatar(&storage, Some(&url), user_id).await;

        assert_eq!(storage.deleted_keys(), vec![key]);
    }

    #[tokio::test]
    async fn swallows_a_failing_delete() {
        let storage = RecordingStorage::new(true);
        let user_id = Uuid::new_v4();
        let url = format!(
            "http://localhost/media/{}",
            avatar_key(UserId::new(user_id), "jpg")
        );

        delete_previous_avatar(&storage, Some(&url), user_id).await;

        assert_eq!(storage.deleted_keys().len(), 1);
    }

    #[tokio::test]
    async fn skips_deletion_when_there_is_no_previous_avatar() {
        let storage = RecordingStorage::new(false);

        delete_previous_avatar(&storage, None, Uuid::new_v4()).await;

        assert!(storage.deleted_keys().is_empty());
    }

    /// An URL pointing outside the caller's own prefix must never reach
    /// `StorageProvider::delete`.
    #[tokio::test]
    async fn skips_deletion_when_the_url_belongs_to_another_user() {
        let storage = RecordingStorage::new(false);
        let url = format!(
            "http://localhost/media/{}",
            avatar_key(UserId::generate(), "png")
        );

        delete_previous_avatar(&storage, Some(&url), Uuid::new_v4()).await;

        assert!(storage.deleted_keys().is_empty());
    }
}
