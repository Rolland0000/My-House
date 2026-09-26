use bytes::Bytes;
use sqlx::PgPool;
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

/// Uploads one photo for `listing_id`, owned by `caller_id`. The listing row
/// is locked for the whole transaction so a second concurrent upload on the
/// same listing blocks until this one commits or rolls back, keeping the
/// 5-photo quota correct under concurrency rather than just sequentially.
pub async fn upload(
    pool: &PgPool,
    storage: &dyn StorageProvider,
    listing_id: Uuid,
    caller_id: Uuid,
    bytes: Bytes,
) -> Result<MediaRow, AppError> {
    let validated = validate_image(&bytes, MAX_IMAGE_SIZE_BYTES)?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|error| AppError::Database(error.to_string()))?;

    let owner_id = repository::lock_listing_owner(&mut *tx, listing_id)
        .await?
        .ok_or(AppError::ListingNotFound)?;
    verify_ownership(owner_id, caller_id)?;

    let existing_count = repository::count_media(&mut *tx, listing_id).await?;
    let decision = upload_decision(existing_count)?;

    let key = listing_media_key(ListingId::new(listing_id), validated.extension);
    let url = storage.upload(&key, bytes, validated.content_type).await?;

    let row = match repository::insert_media(
        &mut *tx,
        Uuid::new_v4(),
        listing_id,
        &key,
        &url,
        decision.is_cover,
        decision.position,
    )
    .await
    {
        Ok(row) => row,
        Err(error) => {
            if let Err(delete_error) = storage.delete(&key).await {
                tracing::warn!(
                    %listing_id, key, error = %delete_error,
                    "failed to delete orphaned listing photo; leaving as orphan"
                );
            }
            return Err(error);
        }
    };

    tx.commit()
        .await
        .map_err(|error| AppError::Database(error.to_string()))?;

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
