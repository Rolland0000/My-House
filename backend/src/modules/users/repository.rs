use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::errors::AppError;
use crate::shared::rbac::Role;

use super::model::UserRow;

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

/// Whether any user row currently has `role = 'admin'`.
pub async fn admin_exists(pool: &PgPool) -> Result<bool, AppError> {
    sqlx::query_scalar!(r#"SELECT EXISTS(SELECT 1 FROM users WHERE role = 'admin') AS "exists!""#)
        .fetch_one(pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))
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
