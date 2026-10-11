use std::collections::{HashMap, HashSet};

use bytes::Bytes;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::infra::storage::StorageProvider;
use crate::shared::errors::AppError;
use crate::shared::file_validation::{validate_image, MAX_IMAGE_SIZE_BYTES};
use crate::shared::media_order::display_order_key;
use crate::shared::storage_key::listing_media_key;
use crate::shared::types::ListingId;
use crate::shared::validation::FieldErrors;

use super::model::{MediaForDeletion, MediaRow, OrderedMediaRow};
use super::repository;

const MAX_PHOTOS_PER_LISTING: i64 = 5;

struct UploadDecision {
    is_cover: bool,
    position: i16,
}

/// Turns the listing's photo count and highest position into where the new
/// photo lands: after the last one, even when deletions left gaps.
/// Pure so the quota/cover/position rules can be tested without a database.
fn upload_decision(
    existing_count: i64,
    last_position: Option<i16>,
) -> Result<UploadDecision, AppError> {
    if existing_count >= MAX_PHOTOS_PER_LISTING {
        return Err(AppError::MediaQuotaExceeded);
    }
    let position = match last_position {
        None => 0,
        Some(last) => last.checked_add(1).ok_or(AppError::Internal)?,
    };
    Ok(UploadDecision {
        is_cover: existing_count == 0,
        position,
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

    let (existing_count, last_position) = repository::upload_slot(&mut **tx, listing_id).await?;
    upload_decision(existing_count, last_position)
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

/// A missing, empty or duplicated list is a 422 on `media_ids`.
fn validate_media_ids(media_ids: Option<Vec<Uuid>>) -> Result<Vec<Uuid>, AppError> {
    let mut errors = FieldErrors::default();
    match &media_ids {
        None => errors.push("media_ids", "media_ids is required."),
        Some(ids) if ids.is_empty() => errors.push("media_ids", "media_ids must not be empty."),
        Some(ids) if ids.iter().collect::<HashSet<_>>().len() != ids.len() => {
            errors.push("media_ids", "media_ids must not contain duplicates.")
        }
        Some(_) => {}
    }
    errors.finish()?;
    media_ids.ok_or(AppError::Internal)
}

/// Gives each current photo its index in `sent` as position, then returns
/// them in display order. `sent` must be exactly the current set of photos.
fn apply_order(
    mut current: Vec<OrderedMediaRow>,
    sent: &[Uuid],
) -> Result<Vec<OrderedMediaRow>, AppError> {
    let index_by_id: HashMap<&Uuid, usize> = sent
        .iter()
        .enumerate()
        .map(|(index, id)| (id, index))
        .collect();
    if index_by_id.len() != sent.len() || sent.len() != current.len() {
        return Err(AppError::MediaOrderConflict);
    }

    for row in &mut current {
        let index = *index_by_id
            .get(&row.id)
            .ok_or(AppError::MediaOrderConflict)?;
        row.position = i16::try_from(index).map_err(|_| AppError::Internal)?;
    }
    current.sort_by_key(|row| {
        display_order_key(row.is_cover, row.position, row.created_at_micros, row.id)
    });
    Ok(current)
}

/// Stores `media_ids` as the photo order and returns the photos in display order.
/// The listing lock serializes this with uploads, deletes and cover changes;
/// any error before the commit leaves every position unchanged.
pub async fn reorder_media(
    pool: &PgPool,
    listing_id: Uuid,
    caller_id: Uuid,
    media_ids: Option<Vec<Uuid>>,
) -> Result<Vec<OrderedMediaRow>, AppError> {
    let media_ids = validate_media_ids(media_ids)?;

    let mut tx = pool.begin().await.map_err(db_err)?;

    let owner_id = repository::lock_listing_owner(&mut *tx, listing_id)
        .await?
        .ok_or(AppError::ListingNotFound)?;
    verify_ownership(owner_id, caller_id)?;

    let current = repository::list_listing_media(&mut *tx, listing_id).await?;
    let ordered = apply_order(current, &media_ids)?;
    repository::set_media_positions(&mut *tx, listing_id, &media_ids).await?;

    tx.commit().await.map_err(db_err)?;
    Ok(ordered)
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

    fn ordered(is_cover: bool, position: i16) -> OrderedMediaRow {
        OrderedMediaRow {
            id: Uuid::new_v4(),
            url: "https://media.example/listings/photo.jpg".to_string(),
            is_cover,
            position,
            created_at_micros: 0,
        }
    }

    fn validation_fields(result: Result<Vec<Uuid>, AppError>) -> Vec<&'static str> {
        match result {
            Err(AppError::Validation(errors)) => errors.iter().map(|error| error.field).collect(),
            other => panic!("expected a validation error, got {other:?}"),
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
        let decision = upload_decision(0, None).unwrap();
        assert!(decision.is_cover);
        assert_eq!(decision.position, 0);
    }

    #[test]
    fn second_photo_is_not_the_cover() {
        let decision = upload_decision(1, Some(0)).unwrap();
        assert!(!decision.is_cover);
        assert_eq!(decision.position, 1);
    }

    #[test]
    fn position_follows_the_last_position() {
        // Two photos left at positions 0 and 3 after deletions.
        assert_eq!(upload_decision(2, Some(3)).unwrap().position, 4);
    }

    #[test]
    fn position_overflow_is_refused_instead_of_duplicated() {
        assert!(matches!(
            upload_decision(1, Some(i16::MAX)),
            Err(AppError::Internal)
        ));
    }

    #[test]
    fn a_sixth_photo_is_refused() {
        assert!(matches!(
            upload_decision(MAX_PHOTOS_PER_LISTING, Some(4)),
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

    #[test]
    fn missing_media_ids_is_a_validation_error() {
        assert_eq!(validation_fields(validate_media_ids(None)), ["media_ids"]);
    }

    #[test]
    fn empty_media_ids_is_a_validation_error() {
        assert_eq!(
            validation_fields(validate_media_ids(Some(vec![]))),
            ["media_ids"]
        );
    }

    #[test]
    fn duplicated_media_id_is_a_validation_error() {
        let id = Uuid::new_v4();
        assert_eq!(
            validation_fields(validate_media_ids(Some(vec![id, id]))),
            ["media_ids"]
        );
    }

    #[test]
    fn distinct_media_ids_are_accepted() {
        let ids = vec![Uuid::new_v4(), Uuid::new_v4()];
        assert_eq!(validate_media_ids(Some(ids.clone())).unwrap(), ids);
    }

    #[test]
    fn exact_match_stores_the_sent_order_with_the_cover_first() {
        let cover = ordered(true, 0);
        let second = ordered(false, 1);
        let third = ordered(false, 2);
        let sent = vec![third.id, second.id, cover.id];

        let result = apply_order(vec![cover, second, third], &sent).unwrap();

        let ids: Vec<Uuid> = result.iter().map(|row| row.id).collect();
        assert_eq!(ids, [sent[2], sent[0], sent[1]]);
        let positions: Vec<i16> = result.iter().map(|row| row.position).collect();
        assert_eq!(positions, [2, 0, 1]);
    }

    #[test]
    fn a_missing_id_is_a_conflict() {
        let first = ordered(true, 0);
        let sent = vec![first.id];
        assert!(matches!(
            apply_order(vec![first, ordered(false, 1)], &sent),
            Err(AppError::MediaOrderConflict)
        ));
    }

    #[test]
    fn an_extra_id_is_a_conflict() {
        let first = ordered(true, 0);
        let sent = vec![first.id, Uuid::new_v4()];
        assert!(matches!(
            apply_order(vec![first], &sent),
            Err(AppError::MediaOrderConflict)
        ));
    }

    #[test]
    fn an_unknown_id_of_the_same_length_is_a_conflict() {
        let first = ordered(true, 0);
        let sent = vec![first.id, Uuid::new_v4()];
        assert!(matches!(
            apply_order(vec![first, ordered(false, 1)], &sent),
            Err(AppError::MediaOrderConflict)
        ));
    }

    #[test]
    fn a_duplicated_id_is_a_conflict() {
        let first = ordered(true, 0);
        let sent = vec![first.id, first.id];
        assert!(matches!(
            apply_order(vec![first, ordered(false, 1)], &sent),
            Err(AppError::MediaOrderConflict)
        ));
    }
}
