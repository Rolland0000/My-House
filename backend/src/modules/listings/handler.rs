use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::extractors::{AppJson, AuthUser};
use crate::shared::pagination::PaginatedResponse;
use crate::shared::rbac::Role;

use super::dto::{
    CreateListingRequest, ListListingsQuery, ListingDetailResponse, ListingSummaryDto,
};
use super::service;

const LISTING_WRITE_ROLES: &[Role] = &[Role::Owner];

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

/// Creates a listing owned by the caller, with status `available`.
#[utoipa::path(
    post,
    path = "/listings",
    tag = "listings",
    request_body = CreateListingRequest,
    responses(
        (status = 201, description = "Listing created", body = ListingDetailResponse),
        (status = 400, description = "Malformed JSON body"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 403, description = "Caller is not an owner"),
        (status = 422, description = "One or more fields violate a rule; `error.fields` lists each one"),
    )
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthUser,
    // Consumes the request body, so it must stay the last argument.
    AppJson(payload): AppJson<CreateListingRequest>,
) -> Result<(StatusCode, Json<ListingDetailResponse>), AppError> {
    user.require_role(LISTING_WRITE_ROLES)?;

    let data = service::create_listing(state.db(), user.user_id, payload).await?;
    Ok((StatusCode::CREATED, Json(ListingDetailResponse { data })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::rbac::require_role;

    #[test]
    fn only_owners_may_create_a_listing() {
        assert!(require_role(Role::Owner, LISTING_WRITE_ROLES).is_ok());
        for role in [Role::Seeker, Role::Admin] {
            assert!(matches!(
                require_role(role, LISTING_WRITE_ROLES),
                Err(AppError::Forbidden)
            ));
        }
    }
}
