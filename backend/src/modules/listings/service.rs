use sqlx::PgPool;
use uuid::Uuid;

use std::ops::RangeInclusive;

use crate::shared::errors::AppError;
use crate::shared::pagination::{PaginatedResponse, PaginationMeta};
use crate::shared::validation::FieldErrors;

use super::dto::{CreateListingRequest, ListListingsQuery, ListingDetailDto, ListingSummaryDto};
use super::model::{ListingType, NewListing};
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

/// Fetches the full detail (owner + media) for one listing.
pub async fn get_listing_detail(pool: &PgPool, id: Uuid) -> Result<ListingDetailDto, AppError> {
    let row = repository::find_listing_by_id(pool, id)
        .await?
        .ok_or(AppError::ListingNotFound)?;
    let media = repository::find_media_for_listing(pool, id).await?;

    Ok(ListingDetailDto::from_row_and_media(row, media))
}

/// Checks every field of a creation request, collecting all violations
/// before failing so the client can show them at once.
pub fn validate_new_listing(request: CreateListingRequest) -> Result<NewListing, AppError> {
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
        ) => Ok(NewListing {
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
    request: CreateListingRequest,
) -> Result<ListingDetailDto, AppError> {
    let new_listing = validate_new_listing(request)?;
    let id = repository::insert_listing(pool, owner_id, &new_listing).await?;
    get_listing_detail(pool, id).await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_request() -> CreateListingRequest {
        CreateListingRequest {
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

    fn violated_fields(request: CreateListingRequest) -> Vec<&'static str> {
        match validate_new_listing(request) {
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
        let listing = validate_new_listing(valid_request()).expect("valid request");
        assert_eq!(listing.city, "Dakar");
        assert_eq!(listing.neighborhood, "Plateau Nord");
        assert_eq!(listing.listing_type, ListingType::Studio);
        assert_eq!(listing.price, 150_000);
        assert_eq!(listing.surface_m2, Some(35));
        assert_eq!(listing.rooms, Some(1));
    }

    #[test]
    fn optional_fields_may_be_absent() {
        let listing = validate_new_listing(CreateListingRequest {
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
            let request = CreateListingRequest {
                title: Some("t".repeat(length)),
                ..valid_request()
            };
            assert_eq!(validate_new_listing(request).is_ok(), accepted, "{length}");
        }
    }

    #[test]
    fn description_bounds() {
        for (length, accepted) in [(19, false), (20, true), (2000, true), (2001, false)] {
            let request = CreateListingRequest {
                description: Some("d".repeat(length)),
                ..valid_request()
            };
            assert_eq!(validate_new_listing(request).is_ok(), accepted, "{length}");
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
            let request = CreateListingRequest {
                price: Some(price),
                ..valid_request()
            };
            assert_eq!(validate_new_listing(request).is_ok(), accepted, "{price}");
        }
    }

    #[test]
    fn surface_bounds() {
        for (surface, accepted) in [(0, false), (1, true), (100_000, true), (100_001, false)] {
            let request = CreateListingRequest {
                surface_m2: Some(surface),
                ..valid_request()
            };
            assert_eq!(validate_new_listing(request).is_ok(), accepted, "{surface}");
        }
    }

    #[test]
    fn rooms_bounds() {
        for (rooms, accepted) in [(-1, false), (0, true), (100, true), (101, false)] {
            let request = CreateListingRequest {
                rooms: Some(rooms),
                ..valid_request()
            };
            assert_eq!(validate_new_listing(request).is_ok(), accepted, "{rooms}");
        }
    }

    #[test]
    fn an_unknown_type_is_rejected_and_every_label_is_accepted() {
        let unknown = CreateListingRequest {
            listing_type: Some("castle".into()),
            ..valid_request()
        };
        assert_eq!(violated_fields(unknown), vec!["type"]);

        for label in ["apartment", "studio", "house", "room", "villa", "other"] {
            let request = CreateListingRequest {
                listing_type: Some(label.into()),
                ..valid_request()
            };
            assert!(validate_new_listing(request).is_ok(), "{label}");
        }
    }

    #[test]
    fn each_required_field_missing_is_reported() {
        let cases = [
            (
                "title",
                CreateListingRequest {
                    title: None,
                    ..valid_request()
                },
            ),
            (
                "description",
                CreateListingRequest {
                    description: None,
                    ..valid_request()
                },
            ),
            (
                "type",
                CreateListingRequest {
                    listing_type: None,
                    ..valid_request()
                },
            ),
            (
                "price",
                CreateListingRequest {
                    price: None,
                    ..valid_request()
                },
            ),
            (
                "city",
                CreateListingRequest {
                    city: None,
                    ..valid_request()
                },
            ),
            (
                "neighborhood",
                CreateListingRequest {
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
        let request = CreateListingRequest {
            city: Some(" \t\u{00A0} ".into()),
            neighborhood: Some("   ".into()),
            ..valid_request()
        };
        assert_eq!(violated_fields(request), vec!["city", "neighborhood"]);
    }

    #[test]
    fn three_invalid_fields_yield_exactly_three_entries() {
        let request = CreateListingRequest {
            title: Some("abc".into()),
            price: Some(0),
            rooms: Some(101),
            ..valid_request()
        };
        assert_eq!(violated_fields(request), vec!["title", "price", "rooms"]);
    }

    #[test]
    fn an_owner_id_in_the_body_is_ignored() {
        let request: CreateListingRequest = serde_json::from_value(serde_json::json!({
            "owner_id": "00000000-0000-0000-0000-000000000001",
            "title": "Studio meublé Plateau",
            "description": "d".repeat(20),
            "type": "studio",
            "price": 150000,
            "city": "Dakar",
            "neighborhood": "Plateau",
        }))
        .expect("body with an extra owner_id deserializes");
        assert!(validate_new_listing(request).is_ok());
    }
}
