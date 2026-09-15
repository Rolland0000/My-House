use bytes::Bytes;
use sqlx::PgPool;
use uuid::Uuid;

use crate::infra::mailer::Mailer;
use crate::infra::storage::StorageProvider;
use crate::modules::notifications;
use crate::shared::errors::AppError;
use crate::shared::file_validation::{
    validate_image, validate_pdf, MAX_IMAGE_SIZE_BYTES, MAX_PDF_SIZE_BYTES,
};
use crate::shared::pagination::{PaginatedResponse, PaginationMeta};
use crate::shared::storage_key::owner_request_document_key;
use crate::shared::types::OwnerRequestId;
use crate::shared::validation::{optional_note, optional_phone, required_name, required_phone};

use super::dto::{AdminOwnerRequestDetailDto, AdminOwnerRequestDto, ReviewDecision};
use super::model::{
    IdentityData, OwnerRequestDocument, OwnerRequestRow, OwnerRequestStatus,
    OwnerRequestSubmission, StoredDocumentKey, UploadedDocument, ValidatedIdentityData,
};
use super::repository;

struct ClassifiedDocument {
    bytes: Bytes,
    original_filename: String,
    extension: &'static str,
    content_type: &'static str,
    side: Option<&'static str>,
}

/// Submits a new owner request: validates everything in memory first, then
/// uploads each document, then writes the DB row. A failure at any of the
/// last two steps deletes whatever was already uploaded for this request, so
/// a rejected submission never leaves a document with no owning row — the
/// reverse (a row with a missing document) is likewise prevented by writing
/// the row only after every upload has succeeded.
pub async fn submit(
    pool: &PgPool,
    storage: &dyn StorageProvider,
    mailer: &Mailer,
    admin_email: &str,
    user_id: Uuid,
    requester_email: &str,
    submission: OwnerRequestSubmission,
) -> Result<OwnerRequestRow, AppError> {
    let phone = required_phone(&submission.phone)?.to_string();
    let secondary_phone =
        optional_phone(submission.secondary_phone.as_deref())?.map(str::to_string);
    let identity_data = parse_identity_data(&submission.identity_data_raw)?;

    if repository::pending_exists_for_user(pool, user_id).await? {
        return Err(AppError::OwnerRequestAlreadyPending);
    }

    let classified = classify_documents(submission.documents)?;
    let request_id = OwnerRequestId::generate();
    let mut uploaded_keys = Vec::with_capacity(classified.len());
    let mut document_entries = Vec::with_capacity(classified.len());

    for doc in classified {
        let key = owner_request_document_key(request_id, doc.extension);
        if let Err(error) = storage.upload(&key, doc.bytes, doc.content_type).await {
            cleanup_uploaded(storage, &uploaded_keys, request_id).await;
            return Err(error);
        }
        document_entries.push(OwnerRequestDocument {
            doc_id: Uuid::new_v4(),
            storage_key: key.clone(),
            original_filename: doc.original_filename,
            content_type: doc.content_type.to_string(),
            side: doc.side,
        });
        uploaded_keys.push(key);
    }

    let identity_data_json = serde_json::to_value(&identity_data)
        .map_err(|error| AppError::BadRequest(error.to_string()))?;
    let documents_json = serde_json::to_value(&document_entries)
        .map_err(|error| AppError::BadRequest(error.to_string()))?;

    let row = match repository::insert_pending(
        pool,
        request_id.into_inner(),
        user_id,
        &phone,
        secondary_phone.as_deref(),
        identity_data_json,
        documents_json,
    )
    .await
    {
        Ok(row) => row,
        Err(error) => {
            cleanup_uploaded(storage, &uploaded_keys, request_id).await;
            return Err(error);
        }
    };

    notifications::service::send_owner_request_received_email(
        mailer,
        &identity_data.full_name,
        requester_email,
        admin_email,
    )
    .await;

    Ok(row)
}

pub async fn get_status(pool: &PgPool, user_id: Uuid) -> Result<Option<OwnerRequestRow>, AppError> {
    repository::find_current_for_user(pool, user_id).await
}

/// Backs `GET /admin/owner-requests`. `raw_status` comes straight off the
/// query string; an unrecognized value is a client error, not a silently
/// ignored filter.
pub async fn list_for_admin(
    pool: &PgPool,
    raw_status: Option<&str>,
    page: Option<u32>,
    per_page: Option<u32>,
) -> Result<PaginatedResponse<AdminOwnerRequestDto>, AppError> {
    let status = parse_status_filter(raw_status)?;

    let total = repository::count_for_admin(pool, status).await?;
    let meta = PaginationMeta::new(page, per_page, total as u64);

    let rows = repository::list_for_admin(pool, status, meta.per_page as i64, meta.offset() as i64)
        .await?;
    let data = rows.into_iter().map(AdminOwnerRequestDto::from).collect();
    Ok(PaginatedResponse::new(data, meta))
}

/// Backs `GET /admin/owner-requests/:id` — the only place `identity_data`
/// and document descriptors are exposed, one request at a time.
pub async fn get_for_admin(
    pool: &PgPool,
    request_id: Uuid,
) -> Result<AdminOwnerRequestDetailDto, AppError> {
    let row = repository::find_by_id_for_admin(pool, request_id)
        .await?
        .ok_or(AppError::OwnerRequestNotFound)?;
    Ok(AdminOwnerRequestDetailDto::from(row))
}

/// Backs `PATCH /admin/owner-requests/:id`: records the admin's decision,
/// promotes the user on approval, and sends the matching email once the
/// decision is durably committed.
pub async fn review(
    pool: &PgPool,
    mailer: &Mailer,
    request_id: Uuid,
    admin_id: Uuid,
    status: ReviewDecision,
    admin_note: Option<&str>,
) -> Result<AdminOwnerRequestDetailDto, AppError> {
    let new_status = match status {
        ReviewDecision::Approved => OwnerRequestStatus::Approved,
        ReviewDecision::Rejected => OwnerRequestStatus::Rejected,
    };
    let admin_note = optional_note(admin_note, "admin_note")?;

    let row =
        repository::review_for_admin(pool, request_id, admin_id, new_status, admin_note).await?;

    match status {
        ReviewDecision::Approved => {
            notifications::service::send_owner_request_approved_email(
                mailer,
                &row.email,
                &row.identity_data.0.full_name,
            )
            .await;
        }
        ReviewDecision::Rejected => {
            notifications::service::send_owner_request_rejected_email(
                mailer,
                &row.email,
                &row.identity_data.0.full_name,
                admin_note,
            )
            .await;
        }
    }

    Ok(AdminOwnerRequestDetailDto::from(row))
}

/// One identity document's bytes, ready to stream back as the HTTP response.
pub struct OwnerRequestDocumentFile {
    pub bytes: Bytes,
    pub content_type: String,
    pub original_filename: String,
}

/// Backs `GET /admin/owner-requests/:id/documents/:doc_id` — the one place
/// `storage_key` leaves the database, used only to fetch the file itself. A
/// `doc_id` that belongs to a different request never matches here, since
/// the lookup is scoped to `request_id`'s own `identity_documents` array.
pub async fn get_document_for_admin(
    pool: &PgPool,
    storage: &dyn StorageProvider,
    request_id: Uuid,
    doc_id: Uuid,
) -> Result<OwnerRequestDocumentFile, AppError> {
    let row = repository::find_by_id_for_admin(pool, request_id)
        .await?
        .ok_or(AppError::OwnerRequestNotFound)?;

    let entry = find_document_entry(row.identity_documents, request_id, doc_id)?;
    let bytes = storage
        .read(&entry.storage_key)
        .await
        .map_err(|error| as_document_not_found(error, request_id, doc_id, &entry.storage_key))?;

    Ok(OwnerRequestDocumentFile {
        bytes,
        content_type: entry.content_type,
        original_filename: entry.original_filename,
    })
}

/// A DB row pointing at a `storage_key` the storage backend no longer has is
/// an ops inconsistency worth its own log line, not a generic 500 — surface
/// it as the same 404 a missing `doc_id` would give. Any other storage error
/// (disk full, permission denied) passes through unchanged.
fn as_document_not_found(
    error: AppError,
    request_id: Uuid,
    doc_id: Uuid,
    storage_key: &str,
) -> AppError {
    match error {
        AppError::StorageKeyNotFound(_) => {
            tracing::error!(
                %request_id, %doc_id, storage_key,
                "identity document missing from storage"
            );
            AppError::OwnerRequestDocumentNotFound
        }
        other => other,
    }
}

/// Resolves one document within a request's own `identity_documents` array.
/// A `doc_id` from a different request simply isn't in this array, so this
/// is where cross-request document access actually gets rejected.
fn find_document_entry(
    identity_documents: serde_json::Value,
    request_id: Uuid,
    doc_id: Uuid,
) -> Result<StoredDocumentKey, AppError> {
    let documents: Vec<StoredDocumentKey> =
        serde_json::from_value(identity_documents).map_err(|error| {
            tracing::warn!(%request_id, error = %error, "failed to parse identity_documents");
            AppError::OwnerRequestDocumentNotFound
        })?;

    documents
        .into_iter()
        .find(|doc| doc.doc_id == doc_id)
        .ok_or(AppError::OwnerRequestDocumentNotFound)
}

fn parse_status_filter(raw: Option<&str>) -> Result<Option<OwnerRequestStatus>, AppError> {
    match raw {
        None => Ok(None),
        Some("pending") => Ok(Some(OwnerRequestStatus::Pending)),
        Some("approved") => Ok(Some(OwnerRequestStatus::Approved)),
        Some("rejected") => Ok(Some(OwnerRequestStatus::Rejected)),
        Some(other) => Err(AppError::InvalidQueryParam(format!(
            "status must be one of pending, approved, rejected (got `{other}`)."
        ))),
    }
}

fn parse_identity_data(raw: &str) -> Result<ValidatedIdentityData, AppError> {
    let parsed: IdentityData = serde_json::from_str(raw)
        .map_err(|_| AppError::BadRequest("identity_data must be valid JSON.".to_string()))?;

    Ok(ValidatedIdentityData {
        full_name: required_name(&parsed.full_name, "identity_data.full_name")?.to_string(),
        id_type: required_name(&parsed.id_type, "identity_data.id_type")?.to_string(),
        id_number: required_name(&parsed.id_number, "identity_data.id_number")?.to_string(),
    })
}

/// Classifies `documents` into one of the two accepted shapes and validates
/// each file's magic bytes and size in the same pass. A file whose format
/// doesn't match its slot (a single non-PDF file, or a non-image among a
/// pair) is a shape violation, not a per-file one, so it surfaces as
/// `InvalidDocument` rather than `InvalidFile`; an oversize file still
/// surfaces as `PayloadTooLarge` regardless of shape.
fn classify_documents(
    documents: Vec<UploadedDocument>,
) -> Result<Vec<ClassifiedDocument>, AppError> {
    match documents.len() {
        1 => {
            let doc = documents.into_iter().next().expect("length checked above");
            let validated = validate_pdf(&doc.bytes, MAX_PDF_SIZE_BYTES).map_err(as_shape_error)?;
            Ok(vec![ClassifiedDocument {
                bytes: doc.bytes,
                original_filename: doc.original_filename,
                extension: validated.extension,
                content_type: validated.content_type,
                side: None,
            }])
        }
        2 => ["front", "back"]
            .into_iter()
            .zip(documents)
            .map(|(side, doc)| {
                let validated =
                    validate_image(&doc.bytes, MAX_IMAGE_SIZE_BYTES).map_err(as_shape_error)?;
                Ok(ClassifiedDocument {
                    bytes: doc.bytes,
                    original_filename: doc.original_filename,
                    extension: validated.extension,
                    content_type: validated.content_type,
                    side: Some(side),
                })
            })
            .collect(),
        _ => Err(AppError::InvalidDocument),
    }
}

/// A format mismatch means the wrong kind of file was sent for this shape;
/// an oversize file keeps its own status regardless of shape.
fn as_shape_error(error: AppError) -> AppError {
    match error {
        AppError::InvalidFile => AppError::InvalidDocument,
        other => other,
    }
}

/// Best-effort delete of every key already uploaded for `request_id`. A
/// failure here is logged and left as an orphan rather than retried or
/// surfaced — the caller is already returning the real error.
async fn cleanup_uploaded(
    storage: &dyn StorageProvider,
    keys: &[String],
    request_id: OwnerRequestId,
) {
    for key in keys {
        if let Err(error) = storage.delete(key).await {
            tracing::warn!(%request_id, key, error = %error, "failed to delete orphaned owner-request document; leaving as orphan");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JPEG: &[u8] = &[0xFF, 0xD8, 0xFF, 0xE0];
    const PDF: &[u8] = b"%PDF-1.7\n1 0 obj\n";

    fn doc(bytes: &[u8]) -> UploadedDocument {
        UploadedDocument {
            bytes: Bytes::from(bytes.to_vec()),
            original_filename: "file".to_string(),
        }
    }

    #[test]
    fn accepts_two_images_as_front_and_back() {
        let classified = classify_documents(vec![doc(JPEG), doc(JPEG)]).unwrap();
        assert_eq!(classified[0].side, Some("front"));
        assert_eq!(classified[1].side, Some("back"));
    }

    #[test]
    fn accepts_one_pdf_with_no_side() {
        let classified = classify_documents(vec![doc(PDF)]).unwrap();
        assert_eq!(classified.len(), 1);
        assert_eq!(classified[0].side, None);
    }

    #[test]
    fn rejects_every_invalid_combination() {
        let cases = vec![
            vec![doc(JPEG)],                       // one image where a PDF was expected
            vec![doc(JPEG), doc(JPEG), doc(JPEG)], // three files
            vec![doc(JPEG), doc(PDF)],             // mixed pair
            vec![],                                // no files
        ];
        for documents in cases {
            assert!(matches!(
                classify_documents(documents),
                Err(AppError::InvalidDocument)
            ));
        }
    }

    #[test]
    fn a_pdf_renamed_into_an_image_slot_is_a_shape_violation() {
        let classified = classify_documents(vec![doc(PDF), doc(JPEG)]);
        assert!(matches!(classified, Err(AppError::InvalidDocument)));
    }

    #[test]
    fn oversize_file_stays_payload_too_large_regardless_of_shape() {
        let mut oversized_pdf = PDF.to_vec();
        oversized_pdf.resize(MAX_PDF_SIZE_BYTES + 1, 0);
        assert!(matches!(
            classify_documents(vec![doc(&oversized_pdf)]),
            Err(AppError::PayloadTooLarge)
        ));

        let mut oversized_image = JPEG.to_vec();
        oversized_image.resize(MAX_IMAGE_SIZE_BYTES + 1, 0);
        assert!(matches!(
            classify_documents(vec![doc(&oversized_image), doc(JPEG)]),
            Err(AppError::PayloadTooLarge)
        ));
    }

    #[test]
    fn parses_and_trims_identity_data() {
        let raw = r#"{"full_name":" Awa Diop ","id_type":"passport","id_number":"AB1234"}"#;
        let parsed = parse_identity_data(raw).unwrap();
        assert_eq!(parsed.full_name, "Awa Diop");
        assert_eq!(parsed.id_number, "AB1234");
    }

    #[test]
    fn rejects_malformed_identity_data_json() {
        assert!(matches!(
            parse_identity_data("not json"),
            Err(AppError::BadRequest(_))
        ));
    }

    #[test]
    fn status_filter_accepts_the_three_known_values() {
        assert_eq!(
            parse_status_filter(Some("pending")).unwrap(),
            Some(OwnerRequestStatus::Pending)
        );
        assert_eq!(
            parse_status_filter(Some("approved")).unwrap(),
            Some(OwnerRequestStatus::Approved)
        );
        assert_eq!(
            parse_status_filter(Some("rejected")).unwrap(),
            Some(OwnerRequestStatus::Rejected)
        );
    }

    #[test]
    fn status_filter_defaults_to_none_when_absent() {
        assert_eq!(parse_status_filter(None).unwrap(), None);
    }

    #[test]
    fn status_filter_rejects_an_unknown_value() {
        assert!(matches!(
            parse_status_filter(Some("archived")),
            Err(AppError::InvalidQueryParam(_))
        ));
    }

    fn documents_json() -> serde_json::Value {
        serde_json::json!([
            {
                "doc_id": "5b1f7e2a-3c4d-4e5f-8a9b-0c1d2e3f4a5b",
                "storage_key": "owner-requests/req-1/front.jpg",
                "original_filename": "front.jpg",
                "content_type": "image/jpeg"
            },
            {
                "doc_id": "6c2f8e3b-4d5e-5f6a-9b0c-1d2e3f4a5b6c",
                "storage_key": "owner-requests/req-1/back.jpg",
                "original_filename": "back.jpg",
                "content_type": "image/jpeg"
            }
        ])
    }

    #[test]
    fn finds_the_document_matching_doc_id_in_its_own_request() {
        let doc_id = Uuid::parse_str("6c2f8e3b-4d5e-5f6a-9b0c-1d2e3f4a5b6c").unwrap();
        let entry = find_document_entry(documents_json(), Uuid::nil(), doc_id).unwrap();
        assert_eq!(entry.storage_key, "owner-requests/req-1/back.jpg");
    }

    #[test]
    fn a_doc_id_from_a_different_request_does_not_resolve() {
        let foreign_doc_id = Uuid::new_v4();
        let result = find_document_entry(documents_json(), Uuid::nil(), foreign_doc_id);
        assert!(matches!(
            result,
            Err(AppError::OwnerRequestDocumentNotFound)
        ));
    }

    #[test]
    fn a_missing_storage_object_surfaces_as_document_not_found() {
        let error = AppError::StorageKeyNotFound("owner-requests/x/front.jpg".to_string());
        let result = as_document_not_found(
            error,
            Uuid::nil(),
            Uuid::nil(),
            "owner-requests/x/front.jpg",
        );
        assert!(matches!(result, AppError::OwnerRequestDocumentNotFound));
    }

    #[test]
    fn other_storage_errors_are_not_reinterpreted_as_not_found() {
        let error = AppError::Storage("disk full".to_string());
        let result = as_document_not_found(
            error,
            Uuid::nil(),
            Uuid::nil(),
            "owner-requests/x/front.jpg",
        );
        assert!(matches!(result, AppError::Storage(_)));
    }
}
