use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::pagination::PaginatedResponse;

use super::dto::{ListListingsQuery, ListingDetailResponse, ListingSummaryDto};
use super::service;

/// Public paginated feed of listings (cover photo + summary).
#[utoipa::path(
    get,
    path = "/listings",
    tag = "listings",
    params(ListListingsQuery),
    responses(
        (status = 200, description = "Paginated list of listings", body = PaginatedResponse<ListingSummaryDto>),
    )
)]
pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListListingsQuery>,
) -> Result<Json<PaginatedResponse<ListingSummaryDto>>, AppError> {
    let response = service::list_listings(state.db(), query).await?;
    Ok(Json(response))
}

/// Full listing detail — description, media, owner info (no phone).
#[utoipa::path(
    get,
    path = "/listings/{id}",
    tag = "listings",
    params(("id" = Uuid, Path, description = "Listing id")),
    responses(
        (status = 200, description = "Listing detail", body = ListingDetailResponse),
        (status = 404, description = "Listing not found"),
    )
)]
pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ListingDetailResponse>, AppError> {
    let data = service::get_listing_detail(state.db(), id).await?;
    Ok(Json(ListingDetailResponse { data }))
}
