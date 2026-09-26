use uuid::Uuid;

use crate::shared::errors::AppError;

use super::model::{MediaForDeletion, MediaRow};

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

/// Locks the photo and its listing's row until the caller's transaction ends.
/// Taking the listing lock serializes deletes with uploads on the same listing.
pub async fn lock_media_for_deletion<'e, E>(
    executor: E,
    media_id: Uuid,
) -> Result<Option<MediaForDeletion>, AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    sqlx::query_as!(
        MediaForDeletion,
        r#"
        SELECT m.listing_id, l.owner_id, m.storage_key, m.is_cover
        FROM listing_media m
        JOIN listings l ON l.id = m.listing_id
        WHERE m.id = $1
        FOR UPDATE OF l, m
        "#,
        media_id
    )
    .fetch_optional(executor)
    .await
    .map_err(db_err)
}

pub async fn delete_media<'e, E>(executor: E, media_id: Uuid) -> Result<(), AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    sqlx::query!(r#"DELETE FROM listing_media WHERE id = $1"#, media_id)
        .execute(executor)
        .await
        .map_err(db_err)?;
    Ok(())
}

/// Returns the photo only if it belongs to `listing_id`.
pub async fn find_listing_media<'e, E>(
    executor: E,
    listing_id: Uuid,
    media_id: Uuid,
) -> Result<Option<MediaRow>, AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    sqlx::query_as!(
        MediaRow,
        r#"
        SELECT id, url, is_cover, position
        FROM listing_media
        WHERE id = $1 AND listing_id = $2
        "#,
        media_id,
        listing_id
    )
    .fetch_optional(executor)
    .await
    .map_err(db_err)
}

/// Must run before [`promote_cover`] in the same transaction: the one-cover
/// index is checked row by row, so the old cover has to be cleared first.
pub async fn demote_other_covers<'e, E>(
    executor: E,
    listing_id: Uuid,
    media_id: Uuid,
) -> Result<(), AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    sqlx::query!(
        r#"
        UPDATE listing_media SET is_cover = FALSE
        WHERE listing_id = $1 AND is_cover AND id <> $2
        "#,
        listing_id,
        media_id
    )
    .execute(executor)
    .await
    .map_err(db_err)?;
    Ok(())
}

pub async fn promote_cover<'e, E>(executor: E, media_id: Uuid) -> Result<MediaRow, AppError>
where
    E: sqlx::PgExecutor<'e>,
{
    sqlx::query_as!(
        MediaRow,
        r#"
        UPDATE listing_media SET is_cover = TRUE
        WHERE id = $1
        RETURNING id, url, is_cover, position
        "#,
        media_id
    )
    .fetch_one(executor)
    .await
    .map_err(db_err)
}
