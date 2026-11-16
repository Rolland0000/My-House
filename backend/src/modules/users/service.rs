use bytes::Bytes;
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::infra::cache::AppCacheProvider;
use crate::infra::storage::{delete_storage_objects, StorageProvider};
use crate::shared::crypto;
use crate::shared::errors::AppError;
use crate::shared::file_validation::{validate_image, MAX_IMAGE_SIZE_BYTES};
use crate::shared::image_processing::reencode_public_image;
use crate::shared::pagination::{PaginatedResponse, PaginationMeta};
use crate::shared::pii::masked_email;
use crate::shared::rbac::Role;
use crate::shared::storage_key::{avatar_key, avatar_key_from_url};
use crate::shared::types::UserId;
use crate::shared::validation::{
    keyword_filter, optional_name, required_name, required_phone, FieldErrors,
};

use super::dto::{
    AccountState, AdminUserDto, AdminUserListQuery, UpdateAccountStateDto, UpdateMeDto,
};
use super::model::{AdminUserRow, UserRow};
use super::repository;

/// Startup-only entry point — called once from `main` before the HTTP server
/// starts accepting requests. There is no HTTP route that reaches this: the
/// admin role is never creatable through the API surface. No-op when
/// disabled. Creates the admin when none exists; otherwise only gives a
/// password to an admin that has none — a password already set is never
/// reconciled, merged, or overwritten.
pub async fn bootstrap_admin(pool: &PgPool, config: &AppConfig) -> Result<(), AppError> {
    if !config.admin.bootstrap_on_startup {
        return Ok(());
    }
    // Guaranteed `Some` when the flag is on — `AppConfig::from_env` rejects the
    // combination at startup.
    let (Some(email), Some(password)) = (
        config.admin.bootstrap_email.as_deref(),
        config.admin.bootstrap_password.as_ref(),
    ) else {
        return Ok(());
    };

    let admin_exists = repository::admin_exists(pool).await?;
    if admin_exists && !repository::admin_without_password_exists(pool).await? {
        return Ok(());
    }

    let password_hash = hash_password_blocking(password.expose().to_owned()).await?;

    if !admin_exists {
        if repository::upsert_admin(pool, email, &password_hash).await? {
            tracing::info!(email = %masked_email(email), "admin account bootstrapped");
        } else {
            tracing::warn!(
                email = %masked_email(email),
                "existing account promoted to admin by bootstrap"
            );
        }
        return Ok(());
    }

    for updated_email in repository::set_missing_admin_passwords(pool, &password_hash).await? {
        tracing::info!(
            email = %masked_email(&updated_email),
            "password set on existing admin account"
        );
    }
    Ok(())
}

/// Argon2 is CPU-bound; running it on a runtime worker would stall other tasks.
async fn hash_password_blocking(password: String) -> Result<String, AppError> {
    tokio::task::spawn_blocking(move || crypto::hash_password(&password))
        .await
        .map_err(|error| {
            tracing::error!(%error, "password hashing task failed");
            AppError::Internal
        })?
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
    let image = reencode_public_image(bytes, &validated).await?;

    let key = avatar_key(UserId::new(user_id), image.extension);
    let avatar_url = storage
        .upload(&key, image.bytes, image.content_type)
        .await?;

    // The new file is referenced by no row until the swap commits: if the
    // account is gone or the write fails, it is removed rather than orphaned.
    let swapped = repository::swap_avatar_url(pool, user_id, &avatar_url).await;
    let (updated, previous_avatar_url) = match swapped {
        Ok(Some(swapped)) => swapped,
        Ok(None) => {
            delete_storage_objects(storage, std::slice::from_ref(&key)).await;
            return Err(AppError::UserNotFound);
        }
        Err(error) => {
            delete_storage_objects(storage, std::slice::from_ref(&key)).await;
            return Err(error);
        }
    };

    delete_previous_avatar(storage, previous_avatar_url.as_deref(), user_id).await;

    Ok(updated)
}

/// Deletes the caller's account. Dropping the cached `is_active` makes the
/// next request with the same token a 401 instead of a write against a
/// deleted user.
pub async fn delete_account(
    pool: &PgPool,
    storage: &dyn StorageProvider,
    is_active_cache: &dyn AppCacheProvider<Uuid, bool>,
    user_id: Uuid,
) -> Result<(), AppError> {
    delete_user_and_files(pool, storage, user_id).await?;
    is_active_cache.invalidate(&user_id).await;
    Ok(())
}

/// Backs `GET /admin/users`.
pub async fn list_for_admin(
    pool: &PgPool,
    query: AdminUserListQuery,
) -> Result<PaginatedResponse<AdminUserDto>, AppError> {
    let is_active = query.state.map(|state| state == AccountState::Active);
    let search = keyword_filter(query.q)?;

    let total = repository::count_for_admin(pool, query.role, is_active, search.as_deref()).await?;
    let meta = PaginationMeta::new(query.page, query.per_page, total as u64);

    let rows = repository::list_for_admin(
        pool,
        query.role,
        is_active,
        search.as_deref(),
        meta.per_page as i64,
        meta.offset() as i64,
    )
    .await?;
    let data = rows.into_iter().map(AdminUserDto::from).collect();
    Ok(PaginatedResponse::new(data, meta))
}

/// Backs `DELETE /admin/users/:id`: `delete_account`'s cleanup, refused for an
/// admin account. Dropping the cached `is_active` ends the user's open session.
pub async fn delete_for_admin(
    pool: &PgPool,
    storage: &dyn StorageProvider,
    is_active_cache: &dyn AppCacheProvider<Uuid, bool>,
    user_id: Uuid,
) -> Result<(), AppError> {
    let user = repository::find_by_id(pool, user_id)
        .await?
        .ok_or(AppError::UserNotFound)?;
    ensure_not_admin(user.role)?;

    delete_user_and_files(pool, storage, user_id).await?;
    is_active_cache.invalidate(&user_id).await;
    Ok(())
}

/// Backs `PATCH /admin/users/:id`. Sending the current value changes nothing.
/// Dropping the cached `is_active` applies the new state to open sessions at once.
pub async fn set_active_for_admin(
    pool: &PgPool,
    is_active_cache: &dyn AppCacheProvider<Uuid, bool>,
    admin_id: Uuid,
    user_id: Uuid,
    request: UpdateAccountStateDto,
) -> Result<AdminUserDto, AppError> {
    let is_active = parse_is_active(request)?;
    let user = repository::find_for_admin(pool, user_id)
        .await?
        .ok_or(AppError::UserNotFound)?;
    ensure_not_admin(user.role)?;

    if repository::set_is_active(pool, user_id, is_active).await? {
        is_active_cache.invalidate(&user_id).await;
        tracing::info!(%admin_id, %user_id, is_active, "account state changed by admin");
    }

    Ok(AdminUserRow { is_active, ..user }.into())
}

/// Backs `POST /admin/users/:id/revoke-owner`. Listings are kept; the visibility
/// rule hides them until the user is approved as owner again.
pub async fn revoke_owner_for_admin(
    pool: &PgPool,
    admin_id: Uuid,
    user_id: Uuid,
) -> Result<AdminUserDto, AppError> {
    let user = repository::find_for_admin(pool, user_id)
        .await?
        .ok_or(AppError::UserNotFound)?;
    ensure_revocable(user.role)?;

    // No row changed: a concurrent revocation or deletion came first.
    if !repository::demote_owner(pool, user_id).await? {
        return Err(AppError::NotAnOwner);
    }
    tracing::info!(%admin_id, %user_id, "owner role revoked by admin");

    Ok(AdminUserRow {
        role: Role::Seeker,
        ..user
    }
    .into())
}

/// A missing or non-boolean value is a 422 on `is_active`.
fn parse_is_active(request: UpdateAccountStateDto) -> Result<bool, AppError> {
    let is_active = request.is_active.and_then(|value| value.as_bool());

    let mut errors = FieldErrors::default();
    if is_active.is_none() {
        errors.push("is_active", "is_active must be true or false.");
    }
    errors.finish()?;

    is_active.ok_or(AppError::Internal)
}

/// Deletes the files, then the rows, in one transaction that first locks the
/// account and its listings so no upload slips in between. The connection is
/// held during the storage deletes; a retry re-enumerates whatever remains.
async fn delete_user_and_files(
    pool: &PgPool,
    storage: &dyn StorageProvider,
    user_id: Uuid,
) -> Result<(), AppError> {
    let mut tx = pool.begin().await.map_err(db_err)?;

    let avatar_url = repository::lock_account_for_deletion(&mut tx, user_id)
        .await?
        .ok_or(AppError::UserNotFound)?;

    let mut keys = Vec::new();
    if let Some(url) = avatar_url.as_deref() {
        match avatar_key_from_url(url, UserId::new(user_id)) {
            Some(key) => keys.push(key),
            None => tracing::warn!(%user_id, "avatar URL yields no storage key; skipping delete"),
        }
    }
    keys.extend(repository::list_listing_media_keys_for_owner(&mut *tx, user_id).await?);
    keys.extend(repository::list_owner_request_document_keys(&mut *tx, user_id).await?);

    delete_storage_objects(storage, &keys).await;

    if !repository::delete_by_id(&mut *tx, user_id).await? {
        return Err(AppError::UserNotFound);
    }

    tx.commit().await.map_err(db_err)?;
    Ok(())
}

fn db_err(error: sqlx::Error) -> AppError {
    AppError::Database(error.to_string())
}

/// The admin routes never act on an admin account, the caller's own included.
fn ensure_not_admin(role: Role) -> Result<(), AppError> {
    match role {
        Role::Admin => Err(AppError::AdminAccountProtected),
        Role::Seeker | Role::Owner => Ok(()),
    }
}

/// Only an owner can lose the owner role, and admin accounts stay protected.
fn ensure_revocable(role: Role) -> Result<(), AppError> {
    ensure_not_admin(role)?;
    if role == Role::Owner {
        Ok(())
    } else {
        Err(AppError::NotAnOwner)
    }
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
    use super::*;
    use crate::infra::storage::test_support::RecordingStorage;

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

    #[test]
    fn only_admin_accounts_are_protected() {
        assert!(ensure_not_admin(Role::Seeker).is_ok());
        assert!(ensure_not_admin(Role::Owner).is_ok());
        assert!(matches!(
            ensure_not_admin(Role::Admin),
            Err(AppError::AdminAccountProtected)
        ));
    }

    #[test]
    fn only_an_owner_role_can_be_revoked() {
        assert!(ensure_revocable(Role::Owner).is_ok());
        assert!(matches!(
            ensure_revocable(Role::Seeker),
            Err(AppError::NotAnOwner)
        ));
        assert!(matches!(
            ensure_revocable(Role::Admin),
            Err(AppError::AdminAccountProtected)
        ));
    }

    #[test]
    fn parse_is_active_accepts_only_a_boolean() {
        let parse = |value| parse_is_active(UpdateAccountStateDto { is_active: value });

        assert!(matches!(parse(Some(serde_json::json!(true))), Ok(true)));
        assert!(matches!(parse(Some(serde_json::json!(false))), Ok(false)));
        for invalid in [
            None,
            Some(serde_json::json!("true")),
            Some(serde_json::json!(1)),
        ] {
            assert!(matches!(parse(invalid), Err(AppError::Validation(_))));
        }
    }
}
