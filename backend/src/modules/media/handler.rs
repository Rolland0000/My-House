use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::extractors::AuthUser;
use crate::shared::rbac::Role;

use super::dto::{MediaDto, MediaResponse, UploadMediaForm};
use super::model::UploadMediaSubmission;
use super::service;

const LISTING_ID_FIELD_NAME: &str = "listing_id";
const FILE_FIELD_NAME: &str = "file";

#[utoipa::path(
    post,
    path = "/media/upload",
    tag = "media",
    request_body(
        content = UploadMediaForm,
        content_type = "multipart/form-data",
        description = "listing_id (uuid) and one image file (JPEG, PNG or WebP, 5 MB max)"
    ),
    responses(
        (status = 201, description = "Photo uploaded", body = MediaResponse),
        (status = 400, description = "Malformed multipart body or missing/invalid listing_id"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 403, description = "Caller is not an owner"),
        (status = 404, description = "Listing not found, or not owned by the caller"),
        (status = 409, description = "The listing already has 5 photos"),
        (status = 413, description = "File exceeds the maximum allowed size"),
        (status = 422, description = "Unsupported image format"),
    )
)]
pub async fn upload_media(
    State(state): State<AppState>,
    user: AuthUser,
    // Consumes the request body, so it must stay the last argument.
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<MediaResponse>), AppError> {
    user.require_role(&[Role::Owner])?;

    let submission = read_submission(&mut multipart).await?;

    let row = service::upload(
        state.db(),
        state.storage().as_ref(),
        submission.listing_id,
        user.user_id,
        submission.bytes,
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(MediaResponse {
            data: MediaDto::from(row),
        }),
    ))
}

#[utoipa::path(
    delete,
    path = "/media/{id}",
    tag = "media",
    params(("id" = Uuid, Path, description = "Media id")),
    responses(
        (status = 204, description = "Photo deleted"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 403, description = "Caller is not an owner"),
        (status = 404, description = "Media not found, or not owned by the caller"),
        (status = 409, description = "The cover cannot be deleted while other photos remain"),
    )
)]
pub async fn delete_media(
    State(state): State<AppState>,
    user: AuthUser,
    Path(media_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    user.require_role(&[Role::Owner])?;

    service::delete(state.db(), state.storage().as_ref(), media_id, user.user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn read_submission(multipart: &mut Multipart) -> Result<UploadMediaSubmission, AppError> {
    let mut listing_id = None;
    let mut bytes = None;

    while let Some(field) = multipart.next_field().await.map_err(multipart_error)? {
        match field.name() {
            Some(LISTING_ID_FIELD_NAME) => {
                listing_id = Some(field.text().await.map_err(multipart_error)?)
            }
            Some(FILE_FIELD_NAME) => bytes = Some(field.bytes().await.map_err(multipart_error)?),
            _ => {}
        }
    }

    let listing_id = listing_id
        .ok_or_else(|| AppError::BadRequest("Missing `listing_id` field.".to_string()))?;
    let listing_id = Uuid::parse_str(&listing_id)
        .map_err(|_| AppError::BadRequest("`listing_id` must be a valid UUID.".to_string()))?;
    let bytes = bytes.ok_or_else(|| AppError::BadRequest("Missing `file` field.".to_string()))?;

    Ok(UploadMediaSubmission { listing_id, bytes })
}

/// The body-size limit set on this route surfaces here, so preserve the 413
/// instead of flattening every multipart failure to 400.
fn multipart_error(error: axum::extract::multipart::MultipartError) -> AppError {
    if error.status() == StatusCode::PAYLOAD_TOO_LARGE {
        return AppError::PayloadTooLarge;
    }
    AppError::BadRequest(error.body_text())
}
