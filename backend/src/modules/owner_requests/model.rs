use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
/// Also the shape read back out of `identity_data` for the admin detail read
/// (`Deserialize`), and its OpenAPI-documented form (`ToSchema`).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

/// One queue row for `GET /admin/owner-requests` — `owner_requests` joined to
/// `users` for the applicant's identifying fields. Excludes
/// `identity_data`/`identity_documents` on purpose: those stay behind
/// [`AdminOwnerRequestDetailRow`], fetched one row at a time instead of
/// riding along on every page of the list.
pub struct AdminOwnerRequestRow {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: String,
    pub secondary_phone: Option<String>,
    pub status: OwnerRequestStatus,
    pub created_at: String,
    pub reviewed_at: Option<String>,
    pub admin_note: Option<String>,
}

/// One request's full detail for `GET /admin/owner-requests/:id` — the only
/// query that reads `identity_data`/`identity_documents`, by id alone,
/// never filtered or paginated alongside other requests.
pub struct AdminOwnerRequestDetailRow {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: String,
    pub secondary_phone: Option<String>,
    pub identity_data: sqlx::types::Json<ValidatedIdentityData>,
    pub identity_documents: Value,
    pub status: OwnerRequestStatus,
    pub created_at: String,
    pub reviewed_at: Option<String>,
    pub admin_note: Option<String>,
}

/// Read-side counterpart to [`OwnerRequestDocument`], used to parse
/// `identity_documents` back out of the JSONB column. `storage_key` is
/// deliberately absent — it is never re-exposed once written.
#[derive(Deserialize)]
pub struct StoredDocumentEntry {
    pub doc_id: Uuid,
    pub original_filename: String,
    pub content_type: String,
    #[serde(default)]
    pub side: Option<String>,
}
