use axum::extract::{Path, Query, State};
use axum::Json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::modules::owner_requests::dto::{
    AdminOwnerRequestDetailResponse, AdminOwnerRequestDto, ListOwnerRequestsQuery,
};
use crate::modules::owner_requests::service;
use crate::shared::errors::AppError;
use crate::shared::extractors::AuthUser;
use crate::shared::pagination::PaginatedResponse;
use crate::shared::rbac::Role;

/// Admin-only queue of owner requests. Delegates entirely to
/// `owner_requests::service` — this module never opens its own access to the
/// `owner_requests` table.
#[utoipa::path(
    get,
    path = "/admin/owner-requests",
    tag = "admin",
    params(ListOwnerRequestsQuery),
    responses(
        (status = 200, description = "Paginated owner-request queue", body = PaginatedResponse<AdminOwnerRequestDto>),
        (status = 400, description = "Invalid `status` filter value"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 403, description = "Caller is not an admin"),
    )
)]
pub async fn list_owner_requests(
    State(state): State<AppState>,
    user: AuthUser,
    Query(query): Query<ListOwnerRequestsQuery>,
) -> Result<Json<PaginatedResponse<AdminOwnerRequestDto>>, AppError> {
    user.require_role(&[Role::Admin])?;

    let response = service::list_for_admin(
        state.db(),
        query.status.as_deref(),
        query.page,
        query.per_page,
    )
    .await?;

    Ok(Json(response))
}

/// One request's full detail, including `identity_data` and every document
/// descriptor — the only admin-facing route allowed to return them.
#[utoipa::path(
    get,
    path = "/admin/owner-requests/{id}",
    tag = "admin",
    params(("id" = Uuid, Path, description = "Owner request id")),
    responses(
        (status = 200, description = "Full request detail", body = AdminOwnerRequestDetailResponse),
        (status = 401, description = "Missing or invalid access token"),
        (status = 403, description = "Caller is not an admin"),
        (status = 404, description = "No request with this id"),
    )
)]
pub async fn get_owner_request(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<AdminOwnerRequestDetailResponse>, AppError> {
    user.require_role(&[Role::Admin])?;

    let data = service::get_for_admin(state.db(), id).await?;
    Ok(Json(AdminOwnerRequestDetailResponse { data }))
}
