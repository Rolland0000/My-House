use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::errors::AppError;
use crate::shared::rbac::Role;

use super::model::RefreshTokenLookup;

fn db_err(error: sqlx::Error) -> AppError {
    AppError::Database(error.to_string())
}

/// Hash lookup joined with the owning user's current role. Booleans are
/// computed in SQL — this crate's sqlx build has no chrono/time feature.
pub async fn find_by_hash(
    pool: &PgPool,
    token_hash: &str,
) -> Result<Option<RefreshTokenLookup>, AppError> {
    sqlx::query_as!(
        RefreshTokenLookup,
        r#"
        SELECT
            rt.id,
            rt.user_id,
            u.role AS "role: Role",
            (rt.revoked_at IS NOT NULL) AS "is_revoked!",
            (rt.expires_at < NOW())     AS "is_expired!"
        FROM refresh_tokens rt
        JOIN users u ON u.id = rt.user_id
        WHERE rt.token_hash = $1
        LIMIT 1
        "#,
        token_hash
    )
    .fetch_optional(pool)
    .await
    .map_err(db_err)
}

/// Atomically revokes `old_id` (chaining `replaced_by_id`) and inserts the
/// rotated row in one transaction. `ttl_days` sets the new sliding
/// `expires_at`. Returns `None` if `old_id` was already revoked by a
/// concurrent request — the `AND revoked_at IS NULL` guard on the UPDATE is
/// what makes that detectable instead of silently double-rotating.
pub async fn rotate(
    pool: &PgPool,
    old_id: Uuid,
    user_id: Uuid,
    new_token_hash: &str,
    ttl_days: i32,
) -> Result<Option<Uuid>, AppError> {
    let mut tx = pool.begin().await.map_err(db_err)?;

    let new_id = sqlx::query_scalar!(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, NOW() + make_interval(days => $3))
        RETURNING id
        "#,
        user_id,
        new_token_hash,
        ttl_days,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(db_err)?;

    let revoked = sqlx::query!(
        r#"UPDATE refresh_tokens SET revoked_at = NOW(), replaced_by_id = $2 WHERE id = $1 AND revoked_at IS NULL"#,
        old_id,
        new_id,
    )
    .execute(&mut *tx)
    .await
    .map_err(db_err)?;

    if revoked.rows_affected() == 0 {
        tx.rollback().await.map_err(db_err)?;
        return Ok(None);
    }

    tx.commit().await.map_err(db_err)?;
    Ok(Some(new_id))
}

/// Revokes every still-active row for `user_id` — the theft response: the
/// whole family, not just the reused row.
pub async fn revoke_all_for_user(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
    sqlx::query!(
        r#"UPDATE refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL"#,
        user_id
    )
    .execute(pool)
    .await
    .map_err(db_err)?;
    Ok(())
}

/// Revokes the single row matching `token_hash`. The `AND revoked_at IS
/// NULL` guard makes this a no-op (0 rows affected, still `Ok`) for an
/// already-revoked or unknown hash — logout's idempotent contract.
pub async fn revoke(pool: &PgPool, token_hash: &str) -> Result<(), AppError> {
    sqlx::query!(
        r#"UPDATE refresh_tokens SET revoked_at = NOW() WHERE token_hash = $1 AND revoked_at IS NULL"#,
        token_hash
    )
    .execute(pool)
    .await
    .map_err(db_err)?;
    Ok(())
}

/// Whether `email` already belongs to a registered user — resolves the OTP
/// request's `is_new` flag without exposing anything else about the row.
pub async fn email_exists(pool: &PgPool, email: &str) -> Result<bool, AppError> {
    sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) AS "exists!""#,
        email
    )
    .fetch_one(pool)
    .await
    .map_err(db_err)
}

/// Existing account's id + current role, for `/auth/otp/verify`'s
/// known-email branch — no `users` mutation, straight to session issuance.
pub async fn find_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<(Uuid, Role)>, AppError> {
    let row = sqlx::query!(
        r#"SELECT id, role AS "role: Role" FROM users WHERE email = $1"#,
        email
    )
    .fetch_optional(pool)
    .await
    .map_err(db_err)?;

    Ok(row.map(|r| (r.id, r.role)))
}

/// Creates a brand-new seeker account for `email`. `role` and `is_active`
/// rely on their DB defaults (`'seeker'` / `true`).
///
/// Returns `None` on a `users.email` unique-violation — a concurrent verify
/// call for the same email won the race — so the caller can fall back
/// instead of surfacing a raw 500.
pub async fn create_seeker(pool: &PgPool, email: &str) -> Result<Option<Uuid>, AppError> {
    match sqlx::query_scalar!(
        r#"INSERT INTO users (email) VALUES ($1) RETURNING id"#,
        email
    )
    .fetch_one(pool)
    .await
    {
        Ok(id) => Ok(Some(id)),
        Err(sqlx::Error::Database(db_error)) if db_error.is_unique_violation() => Ok(None),
        Err(error) => Err(db_err(error)),
    }
}

/// Inserts a brand-new `refresh_tokens` row — backs `/auth/otp/verify`'s
/// initial session issuance, where there is no prior row to revoke (unlike
/// [`rotate`]).
pub async fn insert_refresh_token(
    pool: &PgPool,
    user_id: Uuid,
    token_hash: &str,
    ttl_days: i32,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, NOW() + make_interval(days => $3))
        "#,
        user_id,
        token_hash,
        ttl_days,
    )
    .execute(pool)
    .await
    .map_err(db_err)?;

    Ok(())
}
