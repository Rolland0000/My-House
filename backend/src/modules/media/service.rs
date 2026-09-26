use bytes::Bytes;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::infra::storage::StorageProvider;
use crate::shared::errors::AppError;
use crate::shared::file_validation::{validate_image, MAX_IMAGE_SIZE_BYTES};
use crate::shared::storage_key::listing_media_key;
use crate::shared::types::ListingId;

use super::model::{MediaForDeletion, MediaRow};
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

/// Another owner's photo is reported like a missing one, so media ids never leak.
fn resolve_owned_media(
    media: Option<MediaForDeletion>,
    caller_id: Uuid,
) -> Result<MediaForDeletion, AppError> {
    media
        .filter(|media| media.owner_id == caller_id)
        .ok_or(AppError::MediaNotFound)
}

/// The cover can be deleted only when it is the listing's last photo.
fn delete_decision(is_cover: bool, listing_photo_count: i64) -> Result<(), AppError> {
    if is_cover && listing_photo_count > 1 {
        return Err(AppError::CoverPhotoRequired);
    }
    Ok(())
}

/// Deletes one photo owned by `caller_id`.
///
/// The row is deleted and committed before the stored object, so a storage
/// failure orphans a file and never leaves a row pointing at a missing object.
pub async fn delete(
    pool: &PgPool,
    storage: &dyn StorageProvider,
    media_id: Uuid,
    caller_id: Uuid,
) -> Result<(), AppError> {
    let mut tx = pool.begin().await.map_err(db_err)?;

    let media = repository::lock_media_for_deletion(&mut *tx, media_id).await?;
    let media = resolve_owned_media(media, caller_id)?;

    let listing_photo_count = repository::count_media(&mut *tx, media.listing_id).await?;
    delete_decision(media.is_cover, listing_photo_count)?;

    repository::delete_media(&mut *tx, media_id).await?;
    tx.commit().await.map_err(db_err)?;

    if let Err(delete_error) = storage.delete(&media.storage_key).await {
        tracing::warn!(
            %media_id, key = media.storage_key, error = %delete_error,
            "failed to delete listing photo from storage; leaving as orphan"
        );
    }
    Ok(())
}

enum CoverPromotion {
    AlreadyCover(MediaRow),
    Promote,
}

/// `target` is `None` when the photo doesn't exist or sits on another listing.
fn cover_promotion(target: Option<MediaRow>) -> Result<CoverPromotion, AppError> {
    match target {
        None => Err(AppError::MediaNotFound),
        Some(row) if row.is_cover => Ok(CoverPromotion::AlreadyCover(row)),
        Some(_) => Ok(CoverPromotion::Promote),
    }
}

/// Makes `media_id` the cover of `listing_id`, owned by `caller_id`.
///
/// The listing lock serializes this with uploads and deletes on the same
/// listing, and the demote/promote pair commits as one change.
pub async fn promote_cover(
    pool: &PgPool,
    listing_id: Uuid,
    media_id: Uuid,
    caller_id: Uuid,
) -> Result<MediaRow, AppError> {
    let mut tx = pool.begin().await.map_err(db_err)?;

    let owner_id = repository::lock_listing_owner(&mut *tx, listing_id)
        .await?
        .ok_or(AppError::ListingNotFound)?;
    verify_ownership(owner_id, caller_id)?;

    let target = repository::find_listing_media(&mut *tx, listing_id, media_id).await?;
    if let CoverPromotion::AlreadyCover(row) = cover_promotion(target)? {
        return Ok(row);
    }

    repository::demote_other_covers(&mut *tx, listing_id, media_id).await?;
    let row = repository::promote_cover(&mut *tx, media_id).await?;

    tx.commit().await.map_err(db_err)?;
    Ok(row)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn photo(is_cover: bool, position: i16) -> MediaRow {
        MediaRow {
            id: Uuid::new_v4(),
            url: "https://media.example/listings/photo.jpg".to_string(),
            is_cover,
            position,
        }
    }

    fn media_owned_by(owner_id: Uuid) -> MediaForDeletion {
        MediaForDeletion {
            listing_id: Uuid::new_v4(),
            owner_id,
            storage_key: "listings/some-listing/photo.jpg".to_string(),
            is_cover: false,
        }
    }

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

    #[test]
    fn cover_is_refused_while_other_photos_remain() {
        assert!(matches!(
            delete_decision(true, 3),
            Err(AppError::CoverPhotoRequired)
        ));
    }

    #[test]
    fn cover_can_be_deleted_as_the_last_photo() {
        assert!(delete_decision(true, 1).is_ok());
    }

    #[test]
    fn non_cover_photo_can_always_be_deleted() {
        assert!(delete_decision(false, 1).is_ok());
        assert!(delete_decision(false, MAX_PHOTOS_PER_LISTING).is_ok());
    }

    #[test]
    fn own_photo_is_resolved() {
        let owner = Uuid::new_v4();
        assert!(resolve_owned_media(Some(media_owned_by(owner)), owner).is_ok());
    }

    #[test]
    fn another_owners_photo_is_reported_as_not_found() {
        let media = media_owned_by(Uuid::new_v4());
        assert!(matches!(
            resolve_owned_media(Some(media), Uuid::new_v4()),
            Err(AppError::MediaNotFound)
        ));
    }

    #[test]
    fn unknown_media_id_is_reported_as_not_found() {
        assert!(matches!(
            resolve_owned_media(None, Uuid::new_v4()),
            Err(AppError::MediaNotFound)
        ));
    }

    #[test]
    fn non_cover_photo_of_a_three_photo_listing_is_promoted() {
        let third_photo = photo(false, 2);
        assert!(matches!(
            cover_promotion(Some(third_photo)),
            Ok(CoverPromotion::Promote)
        ));
    }

    #[test]
    fn promoting_the_current_cover_changes_nothing() {
        let cover = photo(true, 0);
        let cover_id = cover.id;
        match cover_promotion(Some(cover)) {
            Ok(CoverPromotion::AlreadyCover(row)) => assert_eq!(row.id, cover_id),
            _ => panic!("the current cover should be returned untouched"),
        }
    }

    #[test]
    fn media_from_another_listing_is_reported_as_not_found() {
        assert!(matches!(
            cover_promotion(None),
            Err(AppError::MediaNotFound)
        ));
    }
}
