use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::cookie::CookieJar;
use bytes::Bytes;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::extractors::{AppJson, AppQuery, AuthUser};
use crate::shared::pagination::PaginatedResponse;
use crate::shared::rbac::Role;
use crate::shared::session_cookie::clear_refresh_cookie;

use super::dto::{
    AdminUserDto, AdminUserListQuery, AvatarUploadForm, UpdateAccountStateDto, UpdateMeDto,
    UserDto, UserResponse,
};
use super::service;

/// Multipart field carrying the image (`TECHNICAL_SPEC_MVP.md §4.2`).
const AVATAR_FIELD_NAME: &str = "file";

#[utoipa::path(
    get,
    path = "/users/me",
    tag = "users",
    responses(
        (status = 200, description = "Caller profile", body = UserResponse),
        (status = 401, description = "Missing or invalid access token"),
        (status = 404, description = "User not found"),
    )
)]
pub async fn get_me(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<UserResponse>, AppError> {
    let row = service::get_me(state.db(), user.user_id).await?;

    Ok(Json(UserResponse {
        data: UserDto::from(row),
    }))
}

#[utoipa::path(
    put,
    path = "/users/me",
    tag = "users",
    request_body = UpdateMeDto,
    responses(
        (status = 200, description = "Profile updated", body = UserResponse),
        (status = 400, description = "Missing or oversized first_name, last_name or phone"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 404, description = "User not found"),
    )
)]
pub async fn update_me(
    State(state): State<AppState>,
    user: AuthUser,
    AppJson(payload): AppJson<UpdateMeDto>,
) -> Result<Json<UserResponse>, AppError> {
    let row = service::update_me(state.db(), user.user_id, payload).await?;

    Ok(Json(UserResponse {
        data: UserDto::from(row),
    }))
}

#[utoipa::path(
    post,
    path = "/users/me/avatar",
    tag = "users",
    request_body(
        content = AvatarUploadForm,
        content_type = "multipart/form-data",
        description = "Image (JPEG, PNG or WebP, 5 MB max) in a `file` field"
    ),
    responses(
        (status = 200, description = "Avatar replaced", body = UserResponse),
        (status = 400, description = "Malformed multipart body or missing `file` field"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 404, description = "User not found"),
        (status = 413, description = "File exceeds the maximum allowed size"),
        (status = 422, description = "Unsupported image format"),
    )
)]
pub async fn upload_avatar(
    State(state): State<AppState>,
    user: AuthUser,
    // Consumes the request body, so it must stay the last argument.
    mut multipart: Multipart,
) -> Result<Json<UserResponse>, AppError> {
    let bytes = read_file_field(&mut multipart).await?;
    let row =
        service::replace_avatar(state.db(), state.storage().as_ref(), user.user_id, bytes).await?;

    Ok(Json(UserResponse {
        data: UserDto::from(row),
    }))
}

/// Deletes the caller's own account only — `user_id` comes from the access
/// token, never from a path or body parameter. The refresh cookie is cleared
/// unconditionally: the account row (and its refresh tokens, via cascade) is
/// already gone by the time this runs.
#[utoipa::path(
    delete,
    path = "/users/me",
    tag = "users",
    responses(
        (status = 200, description = "Account deleted; session cookie cleared"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 404, description = "User not found"),
    )
)]
pub async fn delete_me(
    State(state): State<AppState>,
    user: AuthUser,
    jar: CookieJar,
) -> Result<CookieJar, AppError> {
    service::delete_account(state.db(), state.storage().as_ref(), user.user_id).await?;

    let cookie_domain = state.config().cookie_domain.clone();
    Ok(jar.remove(clear_refresh_cookie(cookie_domain)))
}

#[utoipa::path(
    get,
    path = "/admin/users",
    tag = "admin",
    params(AdminUserListQuery),
    responses(
        (status = 200, description = "Paginated account list, newest first", body = PaginatedResponse<AdminUserDto>),
        (status = 400, description = "Invalid `role`, `state`, `q` or pagination value"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 403, description = "Caller is not an admin"),
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
    user: AuthUser,
    AppQuery(query): AppQuery<AdminUserListQuery>,
) -> Result<Json<PaginatedResponse<AdminUserDto>>, AppError> {
    user.require_role(&[Role::Admin])?;

    let response = service::list_for_admin(state.db(), query).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/admin/users/{id}",
    tag = "admin",
    params(("id" = Uuid, Path, description = "User id")),
    responses(
        (status = 204, description = "Account, data and files deleted"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 403, description = "Caller is not an admin (`FORBIDDEN`), or the target is an admin account (`ADMIN_ACCOUNT_PROTECTED`)"),
        (status = 404, description = "No user with this id"),
    )
)]
pub async fn delete_user(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    user.require_role(&[Role::Admin])?;

    service::delete_for_admin(
        state.db(),
        state.storage().as_ref(),
        state.cache().is_active_status().as_ref(),
        id,
    )
    .await?;

    tracing::info!(admin_id = %user.user_id, deleted_user_id = %id, "user account deleted by admin");
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    patch,
    path = "/admin/users/{id}",
    tag = "admin",
    params(("id" = Uuid, Path, description = "User id")),
    request_body = UpdateAccountStateDto,
    responses(
        (status = 200, description = "Account suspended or reactivated; the current value also returns 200", body = AdminUserDto),
        (status = 400, description = "Malformed JSON body"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 403, description = "Caller is not an admin (`FORBIDDEN`), or the target is an admin account (`ADMIN_ACCOUNT_PROTECTED`)"),
        (status = 404, description = "No user with this id"),
        (status = 422, description = "`is_active` is missing or not a boolean"),
    )
)]
pub async fn update_user_state(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    AppJson(payload): AppJson<UpdateAccountStateDto>,
) -> Result<Json<AdminUserDto>, AppError> {
    user.require_role(&[Role::Admin])?;

    let updated = service::set_active_for_admin(
        state.db(),
        state.cache().is_active_status().as_ref(),
        user.user_id,
        id,
        payload,
    )
    .await?;

    Ok(Json(updated))
}

/// Reads the `file` part, skipping any other field. The part's declared
/// filename and content type are deliberately ignored — the format is decided
/// from the bytes in `shared::file_validation`.
async fn read_file_field(multipart: &mut Multipart) -> Result<Bytes, AppError> {
    while let Some(field) = multipart.next_field().await.map_err(multipart_error)? {
        if field.name() == Some(AVATAR_FIELD_NAME) {
            return field.bytes().await.map_err(multipart_error);
        }
    }

    Err(AppError::BadRequest(format!(
        "Missing `{AVATAR_FIELD_NAME}` field."
    )))
}

/// The body-size limit set on the avatar route surfaces here, so preserve the
/// 413 instead of flattening every multipart failure to 400.
fn multipart_error(error: axum::extract::multipart::MultipartError) -> AppError {
    if error.status() == StatusCode::PAYLOAD_TOO_LARGE {
        return AppError::PayloadTooLarge;
    }
    AppError::BadRequest(error.body_text())
}
