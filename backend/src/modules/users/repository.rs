use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::errors::AppError;
use crate::shared::rbac::Role;

use super::model::{AdminUserRow, UserRow};

/// `is_active` for `user_id`, or `None` if the user no longer exists.
/// Backs the `AuthUser` extractor's cache-miss fallback (MH-32).
pub async fn find_is_active(pool: &PgPool, user_id: Uuid) -> Result<Option<bool>, AppError> {
    sqlx::query_scalar!("SELECT is_active FROM users WHERE id = $1", user_id)
        .fetch_optional(pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))
}

/// Full profile row for `user_id`, or `None` if the user no longer exists.
pub async fn find_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<UserRow>, AppError> {
    sqlx::query_as!(
        UserRow,
        r#"
        SELECT id, email, role AS "role: Role", first_name, last_name, phone, avatar_url,
               is_active,
               to_char(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS "created_at!"
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}

/// Overwrites the editable profile columns and returns the updated row, or
/// `None` if the user no longer exists.
pub async fn update_profile(
    pool: &PgPool,
    user_id: Uuid,
    first_name: Option<&str>,
    last_name: &str,
    phone: &str,
) -> Result<Option<UserRow>, AppError> {
    sqlx::query_as!(
        UserRow,
        r#"
        UPDATE users
        SET first_name = $2, last_name = $3, phone = $4
        WHERE id = $1
        RETURNING id, email, role AS "role: Role", first_name, last_name, phone, avatar_url,
                  is_active,
                  to_char(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS "created_at!"
        "#,
        user_id,
        first_name,
        last_name,
        phone
    )
    .fetch_optional(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}

/// Points the account at `avatar_url` and returns the updated row, or `None`
/// if the user no longer exists.
pub async fn update_avatar_url(
    pool: &PgPool,
    user_id: Uuid,
    avatar_url: &str,
) -> Result<Option<UserRow>, AppError> {
    sqlx::query_as!(
        UserRow,
        r#"
        UPDATE users
        SET avatar_url = $2
        WHERE id = $1
        RETURNING id, email, role AS "role: Role", first_name, last_name, phone, avatar_url,
                  is_active,
                  to_char(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS "created_at!"
        "#,
        user_id,
        avatar_url
    )
    .fetch_optional(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}

/// Total for the admin user list. `search` goes through `strpos` because
/// `unaccent` turns fullwidth `％` and `＿` into `LIKE` wildcards.
pub async fn count_for_admin(
    pool: &PgPool,
    role: Option<Role>,
    is_active: Option<bool>,
    search: Option<&str>,
) -> Result<i64, AppError> {
    sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) AS "count!"
        FROM users u
        WHERE ($1::user_role IS NULL OR u.role = $1)
          AND ($2::bool IS NULL OR u.is_active = $2)
          AND ($3::text IS NULL
               OR strpos(lower(immutable_unaccent(u.email)), lower(immutable_unaccent($3))) > 0
               OR strpos(lower(immutable_unaccent(u.first_name)), lower(immutable_unaccent($3))) > 0
               OR strpos(lower(immutable_unaccent(u.last_name)), lower(immutable_unaccent($3))) > 0)
        "#,
        role as Option<Role>,
        is_active,
        search,
    )
    .fetch_one(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}

/// One page of the admin user list, newest first, with the same filters as
/// `count_for_admin`.
pub async fn list_for_admin(
    pool: &PgPool,
    role: Option<Role>,
    is_active: Option<bool>,
    search: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<AdminUserRow>, AppError> {
    sqlx::query_as!(
        AdminUserRow,
        r#"
        SELECT
            u.id,
            u.email,
            u.first_name,
            u.last_name,
            u.phone,
            u.role AS "role: Role",
            u.is_active,
            to_char(u.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS "created_at!",
            (SELECT COUNT(*) FROM listings l WHERE l.owner_id = u.id) AS "listing_count!"
        FROM users u
        WHERE ($1::user_role IS NULL OR u.role = $1)
          AND ($2::bool IS NULL OR u.is_active = $2)
          AND ($3::text IS NULL
               OR strpos(lower(immutable_unaccent(u.email)), lower(immutable_unaccent($3))) > 0
               OR strpos(lower(immutable_unaccent(u.first_name)), lower(immutable_unaccent($3))) > 0
               OR strpos(lower(immutable_unaccent(u.last_name)), lower(immutable_unaccent($3))) > 0)
        ORDER BY u.created_at DESC, u.id DESC
        LIMIT $4 OFFSET $5
        "#,
        role as Option<Role>,
        is_active,
        search,
        limit,
        offset,
    )
    .fetch_all(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}

/// Whether any user row currently has `role = 'admin'`.
pub async fn admin_exists(pool: &PgPool) -> Result<bool, AppError> {
    sqlx::query_scalar!(r#"SELECT EXISTS(SELECT 1 FROM users WHERE role = 'admin') AS "exists!""#)
        .fetch_one(pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))
}

/// Storage keys of every listing photo owned by `user_id`, read from
/// `listing_media.storage_key` rather than reconstructed from `url`.
pub async fn list_listing_media_keys_for_owner(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<String>, AppError> {
    sqlx::query_scalar!(
        r#"
        SELECT lm.storage_key
        FROM listing_media lm
        JOIN listings l ON l.id = lm.listing_id
        WHERE l.owner_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}

/// Storage keys unnested from `owner_requests.identity_documents`
/// (`[{ "storage_key", "original_filename" }]`) across all of `user_id`'s
/// requests. No dedicated `owner_requests` module exists yet, hence the
/// query living here.
pub async fn list_owner_request_document_keys(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<String>, AppError> {
    let keys = sqlx::query_scalar!(
        r#"
        SELECT elem ->> 'storage_key' AS storage_key
        FROM owner_requests, jsonb_array_elements(identity_documents) AS elem
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))?;

    Ok(keys
        .into_iter()
        .filter_map(|key| {
            if key.is_none() {
                tracing::warn!(%user_id, "identity document missing storage_key; skipping delete");
            }
            key
        })
        .collect())
}

/// Deletes the account row; `ON DELETE CASCADE` removes its dependent rows.
/// `false` when `user_id` no longer exists.
pub async fn delete_by_id(pool: &PgPool, user_id: Uuid) -> Result<bool, AppError> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))?;

    Ok(result.rows_affected() > 0)
}

/// Creates the admin account, promoting the row in place when `email` already
/// belongs to a user. The upsert also makes this safe against a concurrent
/// insert on the `users.email` unique constraint.
///
/// `true` = row inserted, `false` = existing account promoted (`xmax = 0`
/// distinguishes the two in an upsert).
pub async fn upsert_admin(pool: &PgPool, email: &str) -> Result<bool, AppError> {
    sqlx::query_scalar!(
        r#"
        INSERT INTO users (email, role, is_active)
        VALUES ($1, 'admin', true)
        ON CONFLICT (email) DO UPDATE SET role = 'admin', is_active = true
        RETURNING (xmax = 0) AS "created!"
        "#,
        email
    )
    .fetch_one(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}
