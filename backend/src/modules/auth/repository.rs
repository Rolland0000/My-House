use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::errors::AppError;
use crate::shared::rbac::Role;

use super::model::{AdminLoginRow, NewAccount, RefreshTokenLookup};

fn db_err(error: sqlx::Error) -> AppError {
    AppError::Database(error.to_string())
}

/// Hash lookup joined with the owning user's current role and state. Booleans are
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
            u.is_active,
            (rt.revoked_at IS NOT NULL) AS "is_revoked!",
            (rt.expires_at < NOW())     AS "is_expired!",
            (replacement.revoked_at IS NOT NULL) AS "replacement_revoked!"
        FROM refresh_tokens rt
        JOIN users u ON u.id = rt.user_id
        LEFT JOIN refresh_tokens replacement ON replacement.id = rt.replaced_by_id
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

/// Deletes `user_id`'s tokens expired for more than `retention_days`. Within
/// that delay a replayed expired token still revokes its family; after it,
/// the replay only gets a 401.
pub async fn purge_stale_tokens(
    pool: &PgPool,
    user_id: Uuid,
    retention_days: i32,
) -> Result<u64, AppError> {
    let deleted = sqlx::query!(
        r#"
        DELETE FROM refresh_tokens
        WHERE user_id = $1 AND expires_at < NOW() - make_interval(days => $2)
        "#,
        user_id,
        retention_days,
    )
    .execute(pool)
    .await
    .map_err(db_err)?;
    Ok(deleted.rows_affected())
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

/// Existing account's id, current role and `is_active`, for `/auth/otp/verify`'s
/// known-email branch. Writes nothing to `users`.
pub async fn find_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<(Uuid, Role, bool)>, AppError> {
    let row = sqlx::query!(
        r#"SELECT id, role AS "role: Role", is_active FROM users WHERE email = $1"#,
        email
    )
    .fetch_optional(pool)
    .await
    .map_err(db_err)?;

    Ok(row.map(|r| (r.id, r.role, r.is_active)))
}

/// Admin account matching `email`, with its password hash and lock state.
/// Non-admin accounts are invisible here, so they can never sign in with a password.
pub async fn find_admin_for_login(
    pool: &PgPool,
    email: &str,
) -> Result<Option<AdminLoginRow>, AppError> {
    sqlx::query_as!(
        AdminLoginRow,
        r#"
        SELECT
            id,
            password_hash,
            is_active,
            CASE WHEN locked_until > NOW()
                THEN CEIL(EXTRACT(EPOCH FROM (locked_until - NOW())))::BIGINT
            END AS "lock_remaining_seconds",
            (locked_until IS NOT NULL AND locked_until <= NOW()) AS "lock_expired!"
        FROM users
        WHERE email = $1 AND role = 'admin'
        "#,
        email
    )
    .fetch_optional(pool)
    .await
    .map_err(db_err)
}

/// Counts one wrong password and sets the lock once `max_attempts` is reached.
/// Increment and lock decision are a single statement, so concurrent
/// failures can't skip past the threshold.
pub async fn record_failed_login(
    pool: &PgPool,
    user_id: Uuid,
    max_attempts: i32,
    lock_minutes: i32,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE users
        SET failed_login_attempts = failed_login_attempts + 1,
            locked_until = CASE
                WHEN failed_login_attempts + 1 >= $2
                    THEN NOW() + make_interval(mins => $3)
                ELSE locked_until
            END
        WHERE id = $1
        "#,
        user_id,
        max_attempts,
        lock_minutes
    )
    .execute(pool)
    .await
    .map_err(db_err)?;
    Ok(())
}

/// Clears the failure counter and any lock; a no-op write is skipped.
pub async fn reset_login_failures(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE users
        SET failed_login_attempts = 0, locked_until = NULL
        WHERE id = $1 AND (failed_login_attempts > 0 OR locked_until IS NOT NULL)
        "#,
        user_id
    )
    .execute(pool)
    .await
    .map_err(db_err)?;
    Ok(())
}

/// Creates the account and its first refresh token in one transaction.
/// `role` and `is_active` rely on their DB defaults (`'seeker'` / `true`);
/// a `users.email` unique-violation surfaces as [`AppError::EmailAlreadyExists`].
pub async fn create_account(pool: &PgPool, account: NewAccount<'_>) -> Result<Uuid, AppError> {
    let mut tx = pool.begin().await.map_err(db_err)?;

    let user_id = match sqlx::query_scalar!(
        r#"
        INSERT INTO users (email, first_name, last_name, phone)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        account.email,
        account.first_name,
        account.last_name,
        account.phone,
    )
    .fetch_one(&mut *tx)
    .await
    {
        Ok(id) => id,
        // Dropping `tx` un-committed rolls it back.
        Err(sqlx::Error::Database(db_error)) if db_error.is_unique_violation() => {
            return Err(AppError::EmailAlreadyExists)
        }
        Err(error) => return Err(db_err(error)),
    };

    sqlx::query!(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, NOW() + make_interval(days => $3))
        "#,
        user_id,
        account.refresh_token_hash,
        account.refresh_ttl_days,
    )
    .execute(&mut *tx)
    .await
    .map_err(db_err)?;

    tx.commit().await.map_err(db_err)?;
    Ok(user_id)
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
