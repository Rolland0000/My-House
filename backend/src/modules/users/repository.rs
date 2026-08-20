use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::errors::AppError;

/// `is_active` for `user_id`, or `None` if the user no longer exists.
/// Backs the `AuthUser` extractor's cache-miss fallback (MH-32).
pub async fn find_is_active(pool: &PgPool, user_id: Uuid) -> Result<Option<bool>, AppError> {
    sqlx::query_scalar!("SELECT is_active FROM users WHERE id = $1", user_id)
        .fetch_optional(pool)
        .await
        .map_err(|error| AppError::Database(error.to_string()))
}
