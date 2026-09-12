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
use crate::shared::storage_key::owner_request_document_key;
use crate::shared::types::OwnerRequestId;
use crate::shared::validation::{optional_phone, required_name, required_phone};

use super::model::{
    IdentityData, OwnerRequestDocument, OwnerRequestRow, OwnerRequestSubmission, UploadedDocument,
    ValidatedIdentityData,
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

pub async fn get_status(pool: &PgPool, user_id: Uuid) -> Result<OwnerRequestRow, AppError> {
    repository::find_current_for_user(pool, user_id)
        .await?
        .ok_or(AppError::OwnerRequestNotFound)
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
}
