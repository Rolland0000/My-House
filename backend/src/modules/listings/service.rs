use sqlx::PgPool;
use uuid::Uuid;

use std::ops::RangeInclusive;

use crate::infra::storage::{delete_storage_objects, StorageProvider};
use crate::shared::errors::AppError;
use crate::shared::pagination::{PaginatedResponse, PaginationMeta};
use crate::shared::validation::FieldErrors;

use super::dto::{
    ListListingsQuery, ListingDetailDto, ListingRequest, ListingStatusDto, ListingStatusRequest,
    ListingSummaryDto, OwnerListingsQuery,
};
use super::model::{ListingFields, ListingStatus, ListingType};
use super::repository::{self, ListingFilters};

const TITLE_LENGTH: RangeInclusive<usize> = 5..=120;
const DESCRIPTION_LENGTH: RangeInclusive<usize> = 20..=2000;
const PLACE_NAME_MAX_LENGTH: usize = 100;
/// Whole XAF; the upper bound is the largest integer `NUMERIC(12,2)` holds.
const PRICE_RANGE: RangeInclusive<i64> = 1..=9_999_999_999;
const SURFACE_RANGE: RangeInclusive<i64> = 1..=100_000;
const ROOMS_RANGE: RangeInclusive<i64> = 0..=100;

/// Fetches one page of the public listings feed matching `query`'s filters.
pub async fn list_listings(
    pool: &PgPool,
    query: ListListingsQuery,
) -> Result<PaginatedResponse<ListingSummaryDto>, AppError> {
    let filters = ListingFilters {
        owner_id: query.owner_id,
        city: query.city,
        listing_type: query.listing_type,
    };

    let total = repository::count_listings(pool, &filters).await?;
    let meta = PaginationMeta::new(query.page, query.per_page, total as u64);

    let rows = repository::list_listings(pool, &filters, meta.per_page, meta.offset()).await?;
    let data = rows.into_iter().map(ListingSummaryDto::from).collect();

    Ok(PaginatedResponse::new(data, meta))
}

/// Fetches one page of `owner_id`'s listings, drafts and photo-less ones included.
pub async fn list_owner_listings(
    pool: &PgPool,
    owner_id: Uuid,
    query: OwnerListingsQuery,
) -> Result<PaginatedResponse<ListingSummaryDto>, AppError> {
    let total = repository::count_owner_listings(pool, owner_id).await?;
    let meta = PaginationMeta::new(query.page, query.per_page, total as u64);

    let rows =
        repository::list_owner_listings(pool, owner_id, meta.per_page, meta.offset()).await?;
    let data = rows.into_iter().map(ListingSummaryDto::from).collect();

    Ok(PaginatedResponse::new(data, meta))
}

/// True when `caller` may see a listing in this state: the owner always can;
/// anyone else only once it's published and has at least one photo.
pub fn is_visible_to(
    caller: Option<Uuid>,
    owner_id: Uuid,
    is_published: bool,
    has_photo: bool,
) -> bool {
    caller == Some(owner_id) || (is_published && has_photo)
}

/// Fetches the full detail (owner + media) for one listing, as seen by
/// `caller` (`None` for an anonymous request). A listing that exists but
/// isn't visible to `caller` is reported as not found, identical to a
/// genuinely unknown id.
pub async fn get_listing_detail(
    pool: &PgPool,
    id: Uuid,
    caller: Option<Uuid>,
) -> Result<ListingDetailDto, AppError> {
    let row = repository::find_listing_by_id(pool, id)
        .await?
        .ok_or(AppError::ListingNotFound)?;

    if !is_visible_to(
        caller,
        row.owner_id,
        row.published_at.is_some(),
        row.has_photo,
    ) {
        return Err(AppError::ListingNotFound);
    }

    let media = repository::find_media_for_listing(pool, id).await?;

    Ok(ListingDetailDto::from_row_and_media(row, media))
}

/// Checks every field of a creation or edit request, collecting all
/// violations before failing so the client can show them at once.
pub fn validate_listing(request: ListingRequest) -> Result<ListingFields, AppError> {
    let mut errors = FieldErrors::default();

    let title = errors.required_text("title", request.title, TITLE_LENGTH);
    let description = errors.required_text("description", request.description, DESCRIPTION_LENGTH);
    let listing_type = match request.listing_type.as_deref() {
        None => {
            errors.push("type", "type is required.");
            None
        }
        Some(label) => {
            let parsed = ListingType::from_label(label);
            if parsed.is_none() {
                errors.push(
                    "type",
                    "type must be one of apartment, studio, house, room, villa, other.",
                );
            }
            parsed
        }
    };
    let price = errors.required_int("price", request.price, PRICE_RANGE);
    let city = errors.required_place_name("city", request.city, PLACE_NAME_MAX_LENGTH);
    let neighborhood =
        errors.required_place_name("neighborhood", request.neighborhood, PLACE_NAME_MAX_LENGTH);
    let surface_m2 = errors.optional_int("surface_m2", request.surface_m2, SURFACE_RANGE);
    let rooms = errors.optional_int("rooms", request.rooms, ROOMS_RANGE);

    errors.finish()?;

    // `finish` succeeded, so every required value is present and the optional
    // ones were range-checked to fit an i32.
    match (title, description, listing_type, price, city, neighborhood) {
        (
            Some(title),
            Some(description),
            Some(listing_type),
            Some(price),
            Some(city),
            Some(neighborhood),
        ) => Ok(ListingFields {
            title,
            description,
            listing_type,
            price,
            city,
            neighborhood,
            surface_m2: surface_m2.and_then(|value| i32::try_from(value).ok()),
            rooms: rooms.and_then(|value| i32::try_from(value).ok()),
        }),
        _ => Err(AppError::Internal),
    }
}

/// Validates and creates a listing owned by `owner_id`, then returns it in
/// the same shape as `GET /listings/:id`.
pub async fn create_listing(
    pool: &PgPool,
    owner_id: Uuid,
    request: ListingRequest,
) -> Result<ListingDetailDto, AppError> {
    let fields = validate_listing(request)?;
    let id = repository::insert_listing(pool, owner_id, &fields).await?;
    get_listing_detail(pool, id, Some(owner_id)).await
}

/// Validates and replaces the editable fields of `owner_id`'s listing `id`.
/// An unknown id and another owner's listing both give `ListingNotFound`.
pub async fn update_listing(
    pool: &PgPool,
    id: Uuid,
    owner_id: Uuid,
    request: ListingRequest,
) -> Result<ListingDetailDto, AppError> {
    let fields = validate_listing(request)?;
    repository::update_listing(pool, id, owner_id, &fields)
        .await?
        .ok_or(AppError::ListingNotFound)?;
    // Owner-aware read: an edited draft still answers 200.
    get_listing_detail(pool, id, Some(owner_id)).await
}

/// Parses the requested status; a missing or unknown label is a 422 on `status`.
pub fn parse_status(request: ListingStatusRequest) -> Result<ListingStatus, AppError> {
    let mut errors = FieldErrors::default();

    let status = match request.status.as_deref() {
        None => {
            errors.push("status", "status is required.");
            None
        }
        Some(label) => {
            let parsed = ListingStatus::from_label(label);
            if parsed.is_none() {
                errors.push("status", "status must be one of available, unavailable.");
            }
            parsed
        }
    };

    errors.finish()?;
    status.ok_or(AppError::Internal)
}

/// Sets the status of `owner_id`'s listing `id`. Drafts are accepted and
/// `published_at` is left alone. An unknown id and another owner's listing
/// both give `ListingNotFound`.
pub async fn set_listing_status(
    pool: &PgPool,
    id: Uuid,
    owner_id: Uuid,
    request: ListingStatusRequest,
) -> Result<ListingStatusDto, AppError> {
    let status = parse_status(request)?;
    let (id, status) = repository::update_listing_status(pool, id, owner_id, status)
        .await?
        .ok_or(AppError::ListingNotFound)?;
    Ok(ListingStatusDto { id, status })
}

/// Deletes `owner_id`'s listing `id` and its photos, under the same row lock as uploads.
/// Files are deleted after the commit, so a storage failure can only leave an orphaned file.
pub async fn delete_listing(
    pool: &PgPool,
    storage: &dyn StorageProvider,
    id: Uuid,
    owner_id: Uuid,
) -> Result<(), AppError> {
    let db_err = |error: sqlx::Error| AppError::Database(error.to_string());
    let mut tx = pool.begin().await.map_err(db_err)?;

    if !repository::lock_owned_listing(&mut *tx, id, owner_id).await? {
        return Err(AppError::ListingNotFound);
    }
    let keys = repository::list_media_keys(&mut *tx, id).await?;
    repository::delete_listing(&mut *tx, id).await?;
    tx.commit().await.map_err(db_err)?;

    delete_storage_objects(storage, &keys).await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn status_request(status: Option<&str>) -> ListingStatusRequest {
        ListingStatusRequest {
            status: status.map(String::from),
        }
    }

    #[test]
    fn both_status_labels_are_accepted() {
        for (label, expected) in [
            ("available", ListingStatus::Available),
            ("unavailable", ListingStatus::Unavailable),
        ] {
            assert_eq!(
                parse_status(status_request(Some(label))).expect("known label"),
                expected
            );
        }
    }

    #[test]
    fn an_unknown_or_missing_status_is_a_status_field_error() {
        for status in [Some("rented"), Some("Available"), None] {
            match parse_status(status_request(status)) {
                Err(AppError::Validation(entries)) => assert_eq!(
                    entries
                        .into_iter()
                        .map(|entry| entry.field)
                        .collect::<Vec<_>>(),
                    vec!["status"],
                    "{status:?}"
                ),
                other => panic!("expected a validation error for {status:?}, got {other:?}"),
            }
        }
    }

    #[test]
    fn is_visible_to_the_owner_regardless_of_state_or_photos() {
        let owner_id = Uuid::from_u128(1);
        for is_published in [false, true] {
            for has_photo in [false, true] {
                assert!(is_visible_to(
                    Some(owner_id),
                    owner_id,
                    is_published,
                    has_photo
                ));
            }
        }
    }

    #[test]
    fn is_visible_to_a_non_owner_only_when_published_with_a_photo() {
        let owner_id = Uuid::from_u128(1);
        let other_id = Uuid::from_u128(2);

        for caller in [None, Some(other_id)] {
            for (is_published, has_photo, expected) in [
                (false, false, false),
                (false, true, false),
                (true, false, false),
                (true, true, true),
            ] {
                assert_eq!(
                    is_visible_to(caller, owner_id, is_published, has_photo),
                    expected,
                    "caller={caller:?} is_published={is_published} has_photo={has_photo}"
                );
            }
        }
    }

    fn valid_request() -> ListingRequest {
        ListingRequest {
            title: Some("Studio meublé Plateau".into()),
            description: Some("d".repeat(20)),
            listing_type: Some("studio".into()),
            price: Some(150_000),
            city: Some("  Dakar ".into()),
            neighborhood: Some("Plateau   Nord".into()),
            surface_m2: Some(35),
            rooms: Some(1),
        }
    }

    fn violated_fields(request: ListingRequest) -> Vec<&'static str> {
        match validate_listing(request) {
            Err(AppError::Validation(entries)) => {
                entries.into_iter().map(|entry| entry.field).collect()
            }
            other => panic!("expected a validation error, got {other:?}"),
        }
    }

    #[test]
    fn optional_int_ranges_fit_an_i32_column() {
        let max = i64::from(i32::MAX);
        assert!(*SURFACE_RANGE.end() <= max && *ROOMS_RANGE.end() <= max);
    }

    #[test]
    fn a_valid_request_is_normalized() {
        let listing = validate_listing(valid_request()).expect("valid request");
        assert_eq!(listing.city, "Dakar");
        assert_eq!(listing.neighborhood, "Plateau Nord");
        assert_eq!(listing.listing_type, ListingType::Studio);
        assert_eq!(listing.price, 150_000);
        assert_eq!(listing.surface_m2, Some(35));
        assert_eq!(listing.rooms, Some(1));
    }

    #[test]
    fn optional_fields_may_be_absent() {
        let listing = validate_listing(ListingRequest {
            surface_m2: None,
            rooms: None,
            ..valid_request()
        })
        .expect("valid request");
        assert_eq!(listing.surface_m2, None);
        assert_eq!(listing.rooms, None);
    }

    #[test]
    fn title_bounds() {
        for (length, accepted) in [(4, false), (5, true), (120, true), (121, false)] {
            let request = ListingRequest {
                title: Some("t".repeat(length)),
                ..valid_request()
            };
            assert_eq!(validate_listing(request).is_ok(), accepted, "{length}");
        }
    }

    #[test]
    fn description_bounds() {
        for (length, accepted) in [(19, false), (20, true), (2000, true), (2001, false)] {
            let request = ListingRequest {
                description: Some("d".repeat(length)),
                ..valid_request()
            };
            assert_eq!(validate_listing(request).is_ok(), accepted, "{length}");
        }
    }

    #[test]
    fn price_bounds() {
        for (price, accepted) in [
            (0, false),
            (1, true),
            (9_999_999_999, true),
            (10_000_000_000, false),
        ] {
            let request = ListingRequest {
                price: Some(price),
                ..valid_request()
            };
            assert_eq!(validate_listing(request).is_ok(), accepted, "{price}");
        }
    }

    #[test]
    fn surface_bounds() {
        for (surface, accepted) in [(0, false), (1, true), (100_000, true), (100_001, false)] {
            let request = ListingRequest {
                surface_m2: Some(surface),
                ..valid_request()
            };
            assert_eq!(validate_listing(request).is_ok(), accepted, "{surface}");
        }
    }

    #[test]
    fn rooms_bounds() {
        for (rooms, accepted) in [(-1, false), (0, true), (100, true), (101, false)] {
            let request = ListingRequest {
                rooms: Some(rooms),
                ..valid_request()
            };
            assert_eq!(validate_listing(request).is_ok(), accepted, "{rooms}");
        }
    }

    #[test]
    fn an_unknown_type_is_rejected_and_every_label_is_accepted() {
        let unknown = ListingRequest {
            listing_type: Some("castle".into()),
            ..valid_request()
        };
        assert_eq!(violated_fields(unknown), vec!["type"]);

        for label in ["apartment", "studio", "house", "room", "villa", "other"] {
            let request = ListingRequest {
                listing_type: Some(label.into()),
                ..valid_request()
            };
            assert!(validate_listing(request).is_ok(), "{label}");
        }
    }

    #[test]
    fn each_required_field_missing_is_reported() {
        let cases = [
            (
                "title",
                ListingRequest {
                    title: None,
                    ..valid_request()
                },
            ),
            (
                "description",
                ListingRequest {
                    description: None,
                    ..valid_request()
                },
            ),
            (
                "type",
                ListingRequest {
                    listing_type: None,
                    ..valid_request()
                },
            ),
            (
                "price",
                ListingRequest {
                    price: None,
                    ..valid_request()
                },
            ),
            (
                "city",
                ListingRequest {
                    city: None,
                    ..valid_request()
                },
            ),
            (
                "neighborhood",
                ListingRequest {
                    neighborhood: None,
                    ..valid_request()
                },
            ),
        ];
        for (field, request) in cases {
            assert_eq!(violated_fields(request), vec![field]);
        }
    }

    #[test]
    fn whitespace_only_place_names_are_rejected() {
        let request = ListingRequest {
            city: Some(" \t\u{00A0} ".into()),
            neighborhood: Some("   ".into()),
            ..valid_request()
        };
        assert_eq!(violated_fields(request), vec!["city", "neighborhood"]);
    }

    #[test]
    fn three_invalid_fields_yield_exactly_three_entries() {
        let request = ListingRequest {
            title: Some("abc".into()),
            price: Some(0),
            rooms: Some(101),
            ..valid_request()
        };
        assert_eq!(violated_fields(request), vec!["title", "price", "rooms"]);
    }

    #[test]
    fn server_owned_fields_in_the_body_are_ignored() {
        let clean = serde_json::json!({
            "title": "Studio meublé Plateau",
            "description": "d".repeat(20),
            "type": "studio",
            "price": 150000,
            "city": "Dakar",
            "neighborhood": "Plateau",
        });
        let mut with_foreign_fields = clean.clone();
        with_foreign_fields
            .as_object_mut()
            .expect("body is an object")
            .extend([
                (
                    "owner_id".into(),
                    "00000000-0000-0000-0000-000000000001".into(),
                ),
                ("status".into(), "unavailable".into()),
                ("published_at".into(), "2026-10-01T08:30:00Z".into()),
            ]);

        let validate = |body| {
            let request: ListingRequest = serde_json::from_value(body).expect("body deserializes");
            validate_listing(request).expect("valid request")
        };
        assert_eq!(validate(with_foreign_fields), validate(clean));
    }
}
