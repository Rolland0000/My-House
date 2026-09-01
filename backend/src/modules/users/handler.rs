use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::Json;
use bytes::Bytes;

use crate::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::extractors::{AppJson, AuthUser};

use super::dto::{AvatarUploadForm, UpdateMeDto, UserDto, UserResponse};
use super::service;

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

/// Multipart field carrying the image (`TECHNICAL_SPEC_MVP.md §4.2`).
const AVATAR_FIELD_NAME: &str = "file";

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
