use axum::extract::{Path, Query, State};
use axum::http::header::{
    CACHE_CONTROL, CONTENT_DISPOSITION, CONTENT_TYPE, X_CONTENT_TYPE_OPTIONS,
};
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::modules::owner_requests::dto::{
    AdminOwnerRequestDetailResponse, AdminOwnerRequestDto, ListOwnerRequestsQuery,
    ReviewOwnerRequestRequest,
};
use crate::modules::owner_requests::service;
use crate::shared::errors::AppError;
use crate::shared::extractors::{AppJson, AuthUser};
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

/// Streams one identity document's raw bytes back to an admin — the sole
/// backend endpoint that proxies storage reads instead of leaving a file to
/// be served statically (`ARCHITECTURE.md` §7.3). `content_type` comes from
/// the upload-time magic-byte check, not from client input, so it's safe to
/// echo straight into the response header.
#[utoipa::path(
    get,
    path = "/admin/owner-requests/{id}/documents/{doc_id}",
    tag = "admin",
    params(
        ("id" = Uuid, Path, description = "Owner request id"),
        ("doc_id" = Uuid, Path, description = "Document id"),
    ),
    responses(
        (status = 200, description = "Raw document bytes", content_type = "application/octet-stream"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 403, description = "Caller is not an admin"),
        (status = 404, description = "No such request or document"),
    )
)]
pub async fn get_owner_request_document(
    State(state): State<AppState>,
    user: AuthUser,
    Path((id, doc_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    user.require_role(&[Role::Admin])?;

    let file =
        service::get_document_for_admin(state.db(), state.storage().as_ref(), id, doc_id).await?;

    tracing::info!(
        admin_id = %user.user_id, request_id = %id, doc_id = %doc_id,
        "identity document accessed"
    );

    let disposition = format!(
        "inline; filename=\"{}\"",
        sanitize_for_header(&file.original_filename)
    );

    Ok((
        [
            (CONTENT_TYPE, file.content_type),
            (CONTENT_DISPOSITION, disposition),
            (X_CONTENT_TYPE_OPTIONS, "nosniff".to_string()),
            (CACHE_CONTROL, "no-store".to_string()),
        ],
        file.bytes,
    ))
}

/// Records an admin's approve/reject decision. Delegates the whole
/// transaction (status write, role promotion on approval, notification) to
/// `owner_requests::service::review` — this handler only authorizes and logs.
#[utoipa::path(
    patch,
    path = "/admin/owner-requests/{id}",
    tag = "admin",
    params(("id" = Uuid, Path, description = "Owner request id")),
    request_body = ReviewOwnerRequestRequest,
    responses(
        (status = 200, description = "Updated request detail", body = AdminOwnerRequestDetailResponse),
        (status = 400, description = "`status` is neither `approved` nor `rejected`"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 403, description = "Caller is not an admin"),
        (status = 404, description = "No request with this id"),
        (status = 409, description = "Request is no longer pending"),
    )
)]
pub async fn review_owner_request(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    AppJson(body): AppJson<ReviewOwnerRequestRequest>,
) -> Result<Json<AdminOwnerRequestDetailResponse>, AppError> {
    user.require_role(&[Role::Admin])?;

    let data = service::review(
        state.db(),
        state.mailer(),
        id,
        user.user_id,
        body.status,
        body.admin_note.as_deref(),
    )
    .await?;

    tracing::info!(
        admin_id = %user.user_id, request_id = %id, status = ?body.status,
        "owner request reviewed"
    );

    Ok(Json(AdminOwnerRequestDetailResponse { data }))
}

/// Strips characters that would break out of the quoted `filename="..."`
/// value — `original_filename` is whatever the uploader's browser sent, so
/// it reaches this header unsanitized otherwise.
fn sanitize_for_header(filename: &str) -> String {
    filename
        .chars()
        .filter(|c| !c.is_control() && *c != '"' && *c != '\\')
        .collect()
}
