use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::errors::AppError;
use crate::shared::rbac::Role;

use super::model::{AdminUserRow, UserRow};

fn db_err(error: sqlx::Error) -> AppError {
    AppError::Database(error.to_string())
}

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

/// Points the account at `avatar_url` and returns the updated row with the
/// URL it replaced, or `None` if the user no longer exists.
pub async fn swap_avatar_url(
    pool: &PgPool,
    user_id: Uuid,
    avatar_url: &str,
) -> Result<Option<(UserRow, Option<String>)>, AppError> {
    let mut tx = pool.begin().await.map_err(db_err)?;

    // Read under the row lock: of two concurrent replacements, the second
    // sees the first one's file as its predecessor and deletes it.
    let Some(previous_avatar_url) = sqlx::query_scalar!(
        "SELECT avatar_url FROM users WHERE id = $1 FOR UPDATE",
        user_id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(db_err)?
    else {
        return Ok(None);
    };

    let updated = sqlx::query_as!(
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
    .fetch_one(&mut *tx)
    .await
    .map_err(db_err)?;

    tx.commit().await.map_err(db_err)?;
    Ok(Some((updated, previous_avatar_url)))
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

/// The admin list item for `user_id`, or `None` if the user no longer exists.
pub async fn find_for_admin(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<AdminUserRow>, AppError> {
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
        WHERE u.id = $1
        "#,
        user_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(db_err)
}

/// Sets `is_active` and returns whether it changed. Reactivating deletes the refresh tokens so
/// old sessions stay closed; a revoked one, replayed later, would count as theft and end the new session.
pub async fn set_is_active(
    pool: &PgPool,
    user_id: Uuid,
    is_active: bool,
) -> Result<bool, AppError> {
    let mut tx = pool.begin().await.map_err(db_err)?;

    let changed = sqlx::query!(
        "UPDATE users SET is_active = $2 WHERE id = $1 AND is_active <> $2",
        user_id,
        is_active,
    )
    .execute(&mut *tx)
    .await
    .map_err(db_err)?
    .rows_affected()
        > 0;

    if changed && is_active {
        sqlx::query!("DELETE FROM refresh_tokens WHERE user_id = $1", user_id)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;
    }

    tx.commit().await.map_err(db_err)?;
    Ok(changed)
}

/// Turns an owner back into a seeker; `false` when the user is no longer an owner.
pub async fn demote_owner(pool: &PgPool, user_id: Uuid) -> Result<bool, AppError> {
    let result = sqlx::query!(
        "UPDATE users SET role = 'seeker' WHERE id = $1 AND role = 'owner'",
        user_id,
    )
    .execute(pool)
    .await
    .map_err(db_err)?;

    Ok(result.rows_affected() > 0)
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
/// Locks the account row and its listings for the caller's transaction and
/// returns the avatar URL (`None` when the account is gone). A concurrent
/// upload waits on these locks, then fails and deletes its own file.
pub async fn lock_account_for_deletion(
    connection: &mut sqlx::PgConnection,
    user_id: Uuid,
) -> Result<Option<Option<String>>, AppError> {
    let Some(avatar_url) = sqlx::query_scalar!(
        "SELECT avatar_url FROM users WHERE id = $1 FOR UPDATE",
        user_id
    )
    .fetch_optional(&mut *connection)
    .await
    .map_err(db_err)?
    else {
        return Ok(None);
    };

    sqlx::query!(
        "SELECT id FROM listings WHERE owner_id = $1 FOR UPDATE",
        user_id
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(db_err)?;

    Ok(Some(avatar_url))
}

pub async fn list_listing_media_keys_for_owner<'e, E>(
    executor: E,
    user_id: Uuid,
) -> Result<Vec<String>, AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    sqlx::query_scalar!(
        r#"
        SELECT lm.storage_key
        FROM listing_media lm
        JOIN listings l ON l.id = lm.listing_id
        WHERE l.owner_id = $1
        "#,
        user_id
    )
    .fetch_all(executor)
    .await
    .map_err(|error| AppError::Database(error.to_string()))
}

/// Storage keys unnested from `owner_requests.identity_documents`
/// (`[{ "storage_key", "original_filename" }]`) across all of `user_id`'s
/// requests. No dedicated `owner_requests` module exists yet, hence the
/// query living here.
pub async fn list_owner_request_document_keys<'e, E>(
    executor: E,
    user_id: Uuid,
) -> Result<Vec<String>, AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    let keys = sqlx::query_scalar!(
        r#"
        SELECT elem ->> 'storage_key' AS storage_key
        FROM owner_requests, jsonb_array_elements(identity_documents) AS elem
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(executor)
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
pub async fn delete_by_id<'e, E>(executor: E, user_id: Uuid) -> Result<bool, AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(executor)
        .await
        .map_err(|error| AppError::Database(error.to_string()))?;

    Ok(result.rows_affected() > 0)
}

/// Creates the admin account, promoting the row in place when `email` already
/// belongs to a user. The upsert also makes this safe against a concurrent
/// insert on the `users.email` unique constraint.
///
/// `true` = row inserted, `false` = existing account promoted (`xmax = 0`
/// distinguishes the two in an upsert). A promoted account loses its refresh tokens:
/// sessions opened before the promotion, a suspended account's included, don't come back.
/// `password_hash` replaces any previous value and the lockout counters restart.
pub async fn upsert_admin(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
) -> Result<bool, AppError> {
    let mut tx = pool.begin().await.map_err(db_err)?;

    let row = sqlx::query!(
        r#"
        INSERT INTO users (email, role, is_active, password_hash)
        VALUES ($1, 'admin', true, $2)
        ON CONFLICT (email) DO UPDATE SET
            role = 'admin',
            is_active = true,
            password_hash = EXCLUDED.password_hash,
            failed_login_attempts = 0,
            locked_until = NULL
        RETURNING id, (xmax = 0) AS "created!"
        "#,
        email,
        password_hash
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(db_err)?;

    if !row.created {
        sqlx::query!("DELETE FROM refresh_tokens WHERE user_id = $1", row.id)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;
    }

    tx.commit().await.map_err(db_err)?;
    Ok(row.created)
}

/// Whether an admin row exists that has no password yet (created before
/// password login existed).
pub async fn admin_without_password_exists(pool: &PgPool) -> Result<bool, AppError> {
    sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM users WHERE role = 'admin' AND password_hash IS NULL) AS "exists!""#
    )
    .fetch_one(pool)
    .await
    .map_err(db_err)
}

/// Sets the password on every admin that has none, returning the emails
/// updated. The `password_hash IS NULL` guard means a password that is already
/// set is never overwritten.
pub async fn set_missing_admin_passwords(
    pool: &PgPool,
    password_hash: &str,
) -> Result<Vec<String>, AppError> {
    sqlx::query_scalar!(
        r#"
        UPDATE users
        SET password_hash = $1
        WHERE role = 'admin' AND password_hash IS NULL
        RETURNING email
        "#,
        password_hash
    )
    .fetch_all(pool)
    .await
    .map_err(db_err)
}
