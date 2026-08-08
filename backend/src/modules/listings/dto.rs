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
// GET /listings — response item
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
        }
    }
}

/// Envelope for `GET /listings/:id` — single-object `{ "data": {...} }`,
/// distinct from the paginated list envelope used by `GET /listings`.
#[derive(Debug, Serialize, ToSchema)]
pub struct ListingDetailResponse {
    pub data: ListingDetailDto,
}
