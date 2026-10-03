use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::extractors::{AppJson, AuthUser, MaybeAuthUser};
use crate::shared::pagination::PaginatedResponse;
use crate::shared::rbac::Role;

use super::dto::{
    ListListingsQuery, ListingDetailResponse, ListingRequest, ListingSummaryDto, OwnerListingsQuery,
};
use super::service;

const OWNER_ROLES: &[Role] = &[Role::Owner];

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
///
/// Authentication is optional: the owner always sees their own listing,
/// whatever its state; anyone else gets a 404 for a draft or a published
/// listing with no photo, identical to the 404 for an unknown id.
#[utoipa::path(
    get,
    path = "/listings/{id}",
    tag = "listings",
    params(("id" = Uuid, Path, description = "Listing id")),
    responses(
        (status = 200, description = "Listing detail", body = ListingDetailResponse),
        (status = 401, description = "Malformed, invalid, or expired access token, or suspended account"),
        (status = 404, description = "Listing not found, or not publicly visible to the caller"),
    )
)]
pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    caller: MaybeAuthUser,
) -> Result<Json<ListingDetailResponse>, AppError> {
    let data =
        service::get_listing_detail(state.db(), id, caller.0.map(|user| user.user_id)).await?;
    Ok(Json(ListingDetailResponse { data }))
}

/// Creates a listing owned by the caller, with status `available`.
#[utoipa::path(
    post,
    path = "/listings",
    tag = "listings",
    request_body = ListingRequest,
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
    AppJson(payload): AppJson<ListingRequest>,
) -> Result<(StatusCode, Json<ListingDetailResponse>), AppError> {
    user.require_role(OWNER_ROLES)?;

    let data = service::create_listing(state.db(), user.user_id, payload).await?;
    Ok((StatusCode::CREATED, Json(ListingDetailResponse { data })))
}

/// Replaces the editable fields of one of the caller's listings. Status,
/// publication state, owner and timestamps are never taken from the body.
#[utoipa::path(
    put,
    path = "/listings/{id}",
    tag = "listings",
    params(("id" = Uuid, Path, description = "Listing id")),
    request_body = ListingRequest,
    responses(
        (status = 200, description = "Listing updated", body = ListingDetailResponse),
        (status = 400, description = "Malformed JSON body"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 403, description = "Caller is not an owner"),
        (status = 404, description = "Listing not found, or owned by someone else"),
        (status = 422, description = "One or more fields violate a rule; `error.fields` lists each one"),
    )
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    // Consumes the request body, so it must stay the last argument.
    AppJson(payload): AppJson<ListingRequest>,
) -> Result<Json<ListingDetailResponse>, AppError> {
    user.require_role(OWNER_ROLES)?;

    let data = service::update_listing(state.db(), id, user.user_id, payload).await?;
    Ok(Json(ListingDetailResponse { data }))
}

/// The caller's listings in any state, drafts and listings without a photo included, newest first.
#[utoipa::path(
    get,
    path = "/users/me/listings",
    tag = "listings",
    params(OwnerListingsQuery),
    responses(
        (status = 200, description = "Paginated list of the caller's listings", body = PaginatedResponse<ListingSummaryDto>),
        (status = 401, description = "Missing or invalid access token"),
        (status = 403, description = "Caller is not an owner"),
    )
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthUser,
    Query(query): Query<OwnerListingsQuery>,
) -> Result<Json<PaginatedResponse<ListingSummaryDto>>, AppError> {
    user.require_role(OWNER_ROLES)?;

    let response = service::list_owner_listings(state.db(), user.user_id, query).await?;
    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::rbac::require_role;

    #[test]
    fn only_owners_may_create_list_or_edit_their_listings() {
        assert!(require_role(Role::Owner, OWNER_ROLES).is_ok());
        for role in [Role::Seeker, Role::Admin] {
            assert!(matches!(
                require_role(role, OWNER_ROLES),
                Err(AppError::Forbidden)
            ));
        }
    }
}
