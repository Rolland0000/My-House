use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use super::model::{
    ListingDetailRow, ListingMediaRow, ListingStatus, ListingSummaryRow, ListingType,
};

// ─────────────────────────────────────────────────────────────────────────────
// GET /listings — query params
// ─────────────────────────────────────────────────────────────────────────────

/// Query parameters accepted by `GET /listings` (API contract §4.3).
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListListingsQuery {
    pub owner_id: Option<Uuid>,
    pub city: Option<String>,
    #[serde(rename = "type")]
    pub listing_type: Option<ListingType>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

// ─────────────────────────────────────────────────────────────────────────────
// GET /users/me/listings — query params
// ─────────────────────────────────────────────────────────────────────────────

/// Pagination for `GET /users/me/listings`. The owner is always the caller.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct OwnerListingsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

// ─────────────────────────────────────────────────────────────────────────────
// POST /listings — request body
// ─────────────────────────────────────────────────────────────────────────────

/// Body of `POST /listings`. Every field is optional at the serde level so a
/// missing one is reported as a `422` field error rather than a `400`; the
/// schema still marks the required ones. An `owner_id` in the body is ignored.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateListingRequest {
    /// 5 to 120 characters after trimming.
    #[schema(required = true)]
    pub title: Option<String>,
    /// 20 to 2,000 characters after trimming.
    #[schema(required = true)]
    pub description: Option<String>,
    #[serde(rename = "type")]
    #[schema(required = true, value_type = ListingType)]
    pub listing_type: Option<String>,
    /// Whole XAF, 1 to 9,999,999,999.
    #[schema(required = true)]
    pub price: Option<i64>,
    #[schema(required = true)]
    pub city: Option<String>,
    #[schema(required = true)]
    pub neighborhood: Option<String>,
    /// 1 to 100,000.
    pub surface_m2: Option<i64>,
    /// 0 to 100.
    pub rooms: Option<i64>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Shared nested shapes
// ─────────────────────────────────────────────────────────────────────────────

/// Owner info as embedded in a listing summary (no avatar, no phone).
#[derive(Debug, Serialize, ToSchema)]
pub struct OwnerSummaryDto {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

/// Owner info as embedded in a listing detail (adds avatar, still no phone —
/// phone reveal is gated behind `GET /listings/:id/contact`, out of scope here).
#[derive(Debug, Serialize, ToSchema)]
pub struct OwnerDetailDto {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
}

/// One media attachment in a listing detail response.
#[derive(Debug, Serialize, ToSchema)]
pub struct ListingMediaDto {
    pub id: Uuid,
    pub url: String,
    pub is_cover: bool,
    pub position: i16,
}

// ─────────────────────────────────────────────────────────────────────────────
// GET /listings, GET /users/me/listings — response item
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct ListingSummaryDto {
    pub id: Uuid,
    pub title: String,
    #[serde(rename = "type")]
    pub listing_type: ListingType,
    pub status: ListingStatus,
    pub city: String,
    pub neighborhood: Option<String>,
    pub price: f64,
    pub cover_photo_url: Option<String>,
    /// ISO 8601 UTC, or `null` for a draft. Never `null` in the public feed.
    pub published_at: Option<String>,
    pub owner: OwnerSummaryDto,
}

impl From<ListingSummaryRow> for ListingSummaryDto {
    fn from(row: ListingSummaryRow) -> Self {
        Self {
            id: row.id,
            title: row.title,
            listing_type: row.listing_type,
            status: row.status,
            city: row.city,
            neighborhood: row.neighborhood,
            price: row.price,
            cover_photo_url: row.cover_photo_url,
            published_at: row.published_at,
            owner: OwnerSummaryDto {
                id: row.owner_id,
                first_name: row.owner_first_name,
                last_name: row.owner_last_name,
            },
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GET /listings/:id — response
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct ListingDetailDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub listing_type: ListingType,
    pub status: ListingStatus,
    pub city: String,
    pub neighborhood: Option<String>,
    pub price: f64,
    pub surface_m2: Option<i32>,
    pub rooms: Option<i32>,
    pub media: Vec<ListingMediaDto>,
    pub owner: OwnerDetailDto,
    pub created_at: String,
    /// ISO 8601 UTC, or `null` for a draft.
    pub published_at: Option<String>,
}

impl From<ListingMediaRow> for ListingMediaDto {
    fn from(row: ListingMediaRow) -> Self {
        Self {
            id: row.id,
            url: row.url,
            is_cover: row.is_cover,
            position: row.position,
        }
    }
}

impl ListingDetailDto {
    pub fn from_row_and_media(row: ListingDetailRow, media: Vec<ListingMediaRow>) -> Self {
        Self {
            id: row.id,
            title: row.title,
            description: row.description,
            listing_type: row.listing_type,
            status: row.status,
            city: row.city,
            neighborhood: row.neighborhood,
            price: row.price,
            surface_m2: row.surface_m2,
            rooms: row.rooms,
            media: media.into_iter().map(ListingMediaDto::from).collect(),
            owner: OwnerDetailDto {
                id: row.owner_id,
                first_name: row.owner_first_name,
                last_name: row.owner_last_name,
                avatar_url: row.owner_avatar_url,
            },
            created_at: row.created_at,
            published_at: row.published_at,
        }
    }
}

/// Envelope for `GET /listings/:id` and `POST /listings` — single-object `{ "data": {...} }`,
/// distinct from the paginated list envelope used by `GET /listings`.
#[derive(Debug, Serialize, ToSchema)]
pub struct ListingDetailResponse {
    pub data: ListingDetailDto,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn summary_row(published_at: Option<String>) -> ListingSummaryRow {
        ListingSummaryRow {
            id: Uuid::from_u128(1),
            title: "Studio meublé Plateau".into(),
            listing_type: ListingType::Studio,
            status: ListingStatus::Available,
            city: "Dakar".into(),
            neighborhood: Some("Plateau".into()),
            price: 150_000.0,
            cover_photo_url: None,
            published_at,
            owner_id: Uuid::from_u128(2),
            owner_first_name: None,
            owner_last_name: None,
        }
    }

    #[test]
    fn a_draft_summary_serializes_published_at_as_null() {
        let json = serde_json::to_value(ListingSummaryDto::from(summary_row(None)))
            .expect("summary serializes");
        assert_eq!(json.get("published_at"), Some(&serde_json::Value::Null));
    }

    #[test]
    fn a_published_summary_keeps_its_published_at() {
        let published_at = "2026-10-01T08:30:00Z".to_string();
        let dto = ListingSummaryDto::from(summary_row(Some(published_at.clone())));
        assert_eq!(dto.published_at, Some(published_at));
    }
}
