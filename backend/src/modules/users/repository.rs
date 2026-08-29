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

/// Overwrites the editable profile columns and returns the updated row, or
/// `None` if the user no longer exists.
pub async fn update_profile(
    pool: &PgPool,
    user_id: Uuid,
    first_name: &str,
    last_name: &str,
    phone: Option<&str>,
) -> Result<Option<UserRow>, AppError> {
    sqlx::query_as!(
        UserRow,
        r#"
        UPDATE users
        SET first_name = $2, last_name = $3, phone = $4
        WHERE id = $1
        RETURNING id, email, role AS "role: Role", first_name, last_name, phone, avatar_url
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
pub async fn upsert_admin(pool: &PgPool, email: &str) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO users (email, role, is_active)
        VALUES ($1, 'admin', true)
        ON CONFLICT (email) DO UPDATE SET role = 'admin', is_active = true
        "#,
        email
    )
    .execute(pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))?;
    Ok(())
}
