use bytes::Bytes;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::infra::storage::StorageProvider;
use crate::shared::errors::AppError;
use crate::shared::file_validation::{validate_image, MAX_IMAGE_SIZE_BYTES};
use crate::shared::storage_key::listing_media_key;
use crate::shared::types::ListingId;

use super::model::MediaRow;
use super::repository;

const MAX_PHOTOS_PER_LISTING: i64 = 5;

struct UploadDecision {
    is_cover: bool,
    position: i16,
}

/// Turns the listing's current photo count into where the new one lands.
/// Pure so the quota/cover/position rules can be tested without a database.
fn upload_decision(existing_count: i64) -> Result<UploadDecision, AppError> {
    if existing_count >= MAX_PHOTOS_PER_LISTING {
        return Err(AppError::MediaQuotaExceeded);
    }
    Ok(UploadDecision {
        is_cover: existing_count == 0,
        position: existing_count as i16,
    })
}

/// A listing owned by someone else is reported the same way as one that
/// doesn't exist, so this route never discloses another owner's listing ids.
fn verify_ownership(listing_owner_id: Uuid, caller_id: Uuid) -> Result<(), AppError> {
    if listing_owner_id == caller_id {
        Ok(())
    } else {
        Err(AppError::ListingNotFound)
    }
}

fn db_err(error: sqlx::Error) -> AppError {
    AppError::Database(error.to_string())
}

/// Uploads one photo for `listing_id`, owned by `caller_id`.
///
/// The storage write happens outside any row lock, so a slow backend (S3 in
/// V2) never holds a connection and the listing lock for its duration. The
/// quota is checked once before the write to fail fast, then again under
/// the lock before the insert, since a concurrent upload may have taken the
/// last slot in between.
pub async fn upload(
    pool: &PgPool,
    storage: &dyn StorageProvider,
    listing_id: Uuid,
    caller_id: Uuid,
    bytes: Bytes,
) -> Result<MediaRow, AppError> {
    let validated = validate_image(&bytes, MAX_IMAGE_SIZE_BYTES)?;

    let mut precheck_tx = pool.begin().await.map_err(db_err)?;
    check_upload_slot(&mut precheck_tx, listing_id, caller_id).await?;
    precheck_tx.rollback().await.map_err(db_err)?;

    let key = listing_media_key(ListingId::new(listing_id), validated.extension);
    let url = storage.upload(&key, bytes, validated.content_type).await?;

    match persist_media(pool, listing_id, caller_id, &key, &url).await {
        Ok(row) => Ok(row),
        Err(error) => {
            if let Err(delete_error) = storage.delete(&key).await {
                tracing::warn!(
                    %listing_id, key, error = %delete_error,
                    "failed to delete orphaned listing photo; leaving as orphan"
                );
            }
            Err(error)
        }
    }
}

/// Locks the listing row, then checks ownership and the photo quota. The lock
/// lasts until `tx` ends, which serializes concurrent uploads on one listing.
async fn check_upload_slot(
    tx: &mut Transaction<'_, Postgres>,
    listing_id: Uuid,
    caller_id: Uuid,
) -> Result<UploadDecision, AppError> {
    let owner_id = repository::lock_listing_owner(&mut **tx, listing_id)
        .await?
        .ok_or(AppError::ListingNotFound)?;
    verify_ownership(owner_id, caller_id)?;

    let existing_count = repository::count_media(&mut **tx, listing_id).await?;
    upload_decision(existing_count)
}

async fn persist_media(
    pool: &PgPool,
    listing_id: Uuid,
    caller_id: Uuid,
    key: &str,
    url: &str,
) -> Result<MediaRow, AppError> {
    let mut tx = pool.begin().await.map_err(db_err)?;
    let decision = check_upload_slot(&mut tx, listing_id, caller_id).await?;

    let row = repository::insert_media(
        &mut *tx,
        Uuid::new_v4(),
        listing_id,
        key,
        url,
        decision.is_cover,
        decision.position,
    )
    .await?;

    tx.commit().await.map_err(db_err)?;
    Ok(row)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_photo_becomes_the_cover_at_position_zero() {
        let decision = upload_decision(0).unwrap();
        assert!(decision.is_cover);
        assert_eq!(decision.position, 0);
    }

    #[test]
    fn second_photo_is_not_the_cover() {
        let decision = upload_decision(1).unwrap();
        assert!(!decision.is_cover);
        assert_eq!(decision.position, 1);
    }

    #[test]
    fn position_tracks_the_existing_count() {
        for count in 0..MAX_PHOTOS_PER_LISTING {
            assert_eq!(upload_decision(count).unwrap().position, count as i16);
        }
    }

    #[test]
    fn a_sixth_photo_is_refused() {
        assert!(matches!(
            upload_decision(MAX_PHOTOS_PER_LISTING),
            Err(AppError::MediaQuotaExceeded)
        ));
    }

    #[test]
    fn matching_owner_passes() {
        let owner = Uuid::new_v4();
        assert!(verify_ownership(owner, owner).is_ok());
    }

    #[test]
    fn mismatched_owner_is_reported_as_not_found() {
        let owner = Uuid::new_v4();
        let caller = Uuid::new_v4();
        assert!(matches!(
            verify_ownership(owner, caller),
            Err(AppError::ListingNotFound)
        ));
    }
}
