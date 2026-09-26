use uuid::Uuid;

use crate::shared::errors::AppError;

use super::model::MediaRow;

fn db_err(error: sqlx::Error) -> AppError {
    AppError::Database(error.to_string())
}

/// Locks the listing's row for the duration of the caller's transaction, so a
/// concurrent upload on the same listing blocks here instead of racing the
/// quota check below. Returns `None` when the listing doesn't exist.
pub async fn lock_listing_owner<'e, E>(
    executor: E,
    listing_id: Uuid,
) -> Result<Option<Uuid>, AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    sqlx::query_scalar!(
        r#"SELECT owner_id FROM listings WHERE id = $1 FOR UPDATE"#,
        listing_id
    )
    .fetch_optional(executor)
    .await
    .map_err(db_err)
}

/// Current photo count for `listing_id`. Only safe to treat as authoritative
/// when run inside the same transaction as [`lock_listing_owner`].
pub async fn count_media<'e, E>(executor: E, listing_id: Uuid) -> Result<i64, AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    sqlx::query_scalar!(
        r#"SELECT COUNT(*) AS "count!" FROM listing_media WHERE listing_id = $1"#,
        listing_id
    )
    .fetch_one(executor)
    .await
    .map_err(db_err)
}

/// Inserts the new photo row. Callers are expected to have already uploaded
/// the object at `storage_key` — on failure here, the caller is responsible
/// for deleting that now-orphaned upload.
#[allow(clippy::too_many_arguments)]
pub async fn insert_media<'e, E>(
    executor: E,
    id: Uuid,
    listing_id: Uuid,
    storage_key: &str,
    url: &str,
    is_cover: bool,
    position: i16,
) -> Result<MediaRow, AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    sqlx::query_as!(
        MediaRow,
        r#"
        INSERT INTO listing_media (id, listing_id, storage_key, url, is_cover, position)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, url, is_cover, position
        "#,
        id,
        listing_id,
        storage_key,
        url,
        is_cover,
        position,
    )
    .fetch_one(executor)
    .await
    .map_err(db_err)
}
