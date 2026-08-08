use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

// ─────────────────────────────────────────────────────────────────────────────
// Domain enums — mirror the Postgres `listing_type` / `listing_status` enums.
//
// `sqlx::Type` binds each variant to the matching Postgres enum label so rows
// decode directly into these types (no manual string parsing in the
// repository); `Serialize`/`Deserialize` reuse the same casing for the JSON
// wire format and for `GET /listings` query-param filters.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "listing_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ListingType {
    Apartment,
    Studio,
    House,
    Room,
    Villa,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "listing_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ListingStatus {
    Available,
    Unavailable,
}

// ─────────────────────────────────────────────────────────────────────────────
// Repository rows
//
// Raw shapes returned by SQL queries, kept separate from the DTOs in `dto.rs`
// so the wire format (owner nested, price as f64, …) can evolve independently
// of the flat columns a `SELECT` naturally produces.
// ─────────────────────────────────────────────────────────────────────────────

/// One row of the paginated `GET /listings` feed — joins the cover photo
/// (if any) and the owner's public name.
#[derive(Debug, FromRow)]
pub struct ListingSummaryRow {
    pub id: Uuid,
    pub title: String,
    pub listing_type: ListingType,
    pub status: ListingStatus,
    pub city: String,
    pub neighborhood: Option<String>,
    /// Cast from `NUMERIC(12,2)` to `float8` in SQL — the `sqlx` build here
    /// has neither the `rust_decimal` nor `bigdecimal` feature enabled, and
    /// this is a read-only display value, not an arithmetic one.
    pub price: f64,
    pub cover_photo_url: Option<String>,
    pub owner_id: Uuid,
    pub owner_first_name: Option<String>,
    pub owner_last_name: Option<String>,
}

/// Full row for `GET /listings/:id`, excluding media (fetched separately via
/// [`ListingMediaRow`] since it's a one-to-many relation).
#[derive(Debug)]
pub struct ListingDetailRow {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub listing_type: ListingType,
    pub status: ListingStatus,
    pub city: String,
    pub neighborhood: Option<String>,
    pub price: f64,
    pub surface_m2: Option<i32>,
    pub rooms: Option<i32>,
    /// Pre-formatted as ISO 8601 (`to_char(... , 'YYYY-MM-DD"T"HH24:MI:SS"Z"')`)
    /// in SQL, for the same reason `price` is cast to `float8`: no `chrono`/
    /// `time` feature enabled on `sqlx` in this crate.
    pub created_at: String,
    pub owner_id: Uuid,
    pub owner_first_name: Option<String>,
    pub owner_last_name: Option<String>,
    pub owner_avatar_url: Option<String>,
}

/// One `listing_media` row attached to a listing detail response.
#[derive(Debug, FromRow)]
pub struct ListingMediaRow {
    pub id: Uuid,
    pub url: String,
    pub is_cover: bool,
    pub position: i16,
}
