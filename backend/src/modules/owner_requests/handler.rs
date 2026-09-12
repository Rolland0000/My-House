use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::Json;
use bytes::Bytes;

use crate::app_state::AppState;
use crate::modules::users;
use crate::shared::errors::AppError;
use crate::shared::extractors::AuthUser;
use crate::shared::rbac::Role;

use super::dto::{OwnerRequestDto, OwnerRequestResponse, OwnerRequestSubmissionForm};
use super::model::{OwnerRequestSubmission, UploadedDocument};
use super::service;

const DOCUMENTS_FIELD_NAME: &str = "documents";

#[utoipa::path(
    post,
    path = "/owner-requests",
    tag = "owner_requests",
    request_body(
        content = OwnerRequestSubmissionForm,
        content_type = "multipart/form-data",
        description = "phone, optional secondary_phone, identity_data (JSON string), and either two image documents (front, back) or one PDF"
    ),
    responses(
        (status = 201, description = "Request created", body = OwnerRequestResponse),
        (status = 400, description = "Malformed multipart body or identity_data"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 403, description = "Caller is an admin"),
        (status = 409, description = "A pending request already exists for this user"),
        (status = 413, description = "A document exceeds its maximum allowed size"),
        (status = 422, description = "Wrong number or combination of documents, or an unsupported format"),
    )
)]
pub async fn submit_owner_request(
    State(state): State<AppState>,
    user: AuthUser,
    // Consumes the request body, so it must stay the last argument.
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<OwnerRequestResponse>), AppError> {
    user.require_role(&[Role::Seeker, Role::Owner])?;

    let requester = users::service::get_me(state.db(), user.user_id).await?;
    let submission = read_submission(&mut multipart).await?;

    let row = service::submit(
        state.db(),
        state.storage().as_ref(),
        state.mailer(),
        &state.config().admin_notification_email,
        user.user_id,
        &requester.email,
        submission,
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(OwnerRequestResponse {
            data: OwnerRequestDto::from(row),
        }),
    ))
}

#[utoipa::path(
    get,
    path = "/users/me/owner-request",
    tag = "owner_requests",
    responses(
        (status = 200, description = "Caller's current or most recent request", body = OwnerRequestResponse),
        (status = 401, description = "Missing or invalid access token"),
        (status = 404, description = "No request on file for this user"),
    )
)]
pub async fn get_owner_request_status(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<OwnerRequestResponse>, AppError> {
    let row = service::get_status(state.db(), user.user_id).await?;

    Ok(Json(OwnerRequestResponse {
        data: OwnerRequestDto::from(row),
    }))
}

/// Walks every multipart field once, in order. `documents` may repeat (one
/// part per file); the rest are read once each, last write winning if a
/// client sends a field twice.
async fn read_submission(multipart: &mut Multipart) -> Result<OwnerRequestSubmission, AppError> {
    let mut phone = None;
    let mut secondary_phone = None;
    let mut identity_data_raw = None;
    let mut documents = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(multipart_error)? {
        match field.name() {
            Some("phone") => phone = Some(field.text().await.map_err(multipart_error)?),
            Some("secondary_phone") => {
                secondary_phone = Some(field.text().await.map_err(multipart_error)?)
            }
            Some("identity_data") => {
                identity_data_raw = Some(field.text().await.map_err(multipart_error)?)
            }
            Some(DOCUMENTS_FIELD_NAME) => {
                let original_filename = field.file_name().unwrap_or("document").to_string();
                let bytes: Bytes = field.bytes().await.map_err(multipart_error)?;
                documents.push(UploadedDocument {
                    bytes,
                    original_filename,
                });
            }
            _ => {}
        }
    }

    Ok(OwnerRequestSubmission {
        phone: phone.ok_or_else(|| AppError::BadRequest("Missing `phone` field.".to_string()))?,
        secondary_phone,
        identity_data_raw: identity_data_raw
            .ok_or_else(|| AppError::BadRequest("Missing `identity_data` field.".to_string()))?,
        documents,
    })
}

/// The body-size limit set on this route surfaces here, so preserve the 413
/// instead of flattening every multipart failure to 400.
fn multipart_error(error: axum::extract::multipart::MultipartError) -> AppError {
    if error.status() == StatusCode::PAYLOAD_TOO_LARGE {
        return AppError::PayloadTooLarge;
    }
    AppError::BadRequest(error.body_text())
}
