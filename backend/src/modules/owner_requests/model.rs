use bytes::Bytes;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "owner_request_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum OwnerRequestStatus {
    Pending,
    Approved,
    Rejected,
}

pub struct OwnerRequestRow {
    pub id: Uuid,
    pub status: OwnerRequestStatus,
    /// Pre-formatted as ISO 8601 in SQL, like `UserRow::created_at` — no
    /// `chrono`/`time` feature enabled on `sqlx` in this crate.
    pub created_at: String,
    pub reviewed_at: Option<String>,
    pub admin_note: Option<String>,
}

/// One entry of the `identity_documents` JSONB array. `storage_key` must keep
/// this exact field name: `users::repository::list_owner_request_document_keys`
/// reads it back for account-deletion cleanup.
#[derive(Serialize)]
pub struct OwnerRequestDocument {
    pub doc_id: Uuid,
    pub storage_key: String,
    pub original_filename: String,
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<&'static str>,
}

/// Parsed shape of the `identity_data` multipart field (a JSON string).
#[derive(Deserialize)]
pub struct IdentityData {
    pub full_name: String,
    pub id_type: String,
    pub id_number: String,
}

/// Validated, re-serializable form of [`IdentityData`] — only the trimmed,
/// bounds-checked fields ever reach the database, never the client's raw JSON.
#[derive(Serialize)]
pub struct ValidatedIdentityData {
    pub full_name: String,
    pub id_type: String,
    pub id_number: String,
}

/// One `documents` multipart part, in the order it was received. The
/// frontend always sends front then back for the photo modality, which is
/// what lets the service assign sides by position.
pub struct UploadedDocument {
    pub bytes: Bytes,
    pub original_filename: String,
}

/// Everything read off the multipart body, handed to the service as one
/// value instead of one argument per field.
pub struct OwnerRequestSubmission {
    pub phone: String,
    pub secondary_phone: Option<String>,
    pub identity_data_raw: String,
    pub documents: Vec<UploadedDocument>,
}
