use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use super::model::{
    AdminOwnerRequestDetailRow, AdminOwnerRequestRow, OwnerRequestRow, OwnerRequestStatus,
    StoredDocumentEntry, ValidatedIdentityData,
};

#[derive(Debug, Serialize, ToSchema)]
pub struct OwnerRequestDto {
    pub id: Uuid,
    pub status: OwnerRequestStatus,
    pub created_at: String,
    pub reviewed_at: Option<String>,
    /// Only ever populated when `status` is `rejected`.
    pub admin_note: Option<String>,
}

impl From<OwnerRequestRow> for OwnerRequestDto {
    fn from(row: OwnerRequestRow) -> Self {
        let admin_note = match row.status {
            OwnerRequestStatus::Rejected => row.admin_note,
            _ => None,
        };
        Self {
            id: row.id,
            status: row.status,
            created_at: row.created_at,
            reviewed_at: row.reviewed_at,
            admin_note,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OwnerRequestResponse {
    pub data: OwnerRequestDto,
}

/// Body of `GET /users/me/owner-request` — `data` is `null` when the caller
/// has never applied, rather than a 404.
#[derive(Debug, Serialize, ToSchema)]
pub struct OwnerRequestStatusResponse {
    pub data: Option<OwnerRequestDto>,
}

/// Documents the submission body for the OpenAPI schema. Nothing
/// deserializes into it — the handler walks the multipart fields itself.
#[derive(ToSchema)]
pub struct OwnerRequestSubmissionForm {
    pub phone: String,
    pub secondary_phone: Option<String>,
    /// JSON-encoded `{ full_name, id_type, id_number }`.
    pub identity_data: String,
    /// Two images (front, back) or one PDF.
    #[schema(value_type = Vec<String>, format = Binary)]
    pub documents: Vec<String>,
}

// ─────────────────────────────
// GET /admin/owner-requests
// ─────────────────────────────

/// Query parameters for the admin queue. `status` stays a raw string so an
/// invalid value surfaces as `AppError::InvalidQueryParam` (400) instead of
/// axum's default deserialization rejection.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListOwnerRequestsQuery {
    pub status: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// One document descriptor in the admin queue — never carries `storage_key`.
#[derive(Debug, Serialize, ToSchema)]
pub struct AdminOwnerRequestDocumentDto {
    pub doc_id: Uuid,
    pub original_filename: String,
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
}

impl From<StoredDocumentEntry> for AdminOwnerRequestDocumentDto {
    fn from(entry: StoredDocumentEntry) -> Self {
        Self {
            doc_id: entry.doc_id,
            original_filename: entry.original_filename,
            content_type: entry.content_type,
            side: entry.side,
        }
    }
}

/// One row of the admin owner-request queue. Carries only what triage needs
/// at a glance — no `identity_data`, no documents; those live behind
/// [`AdminOwnerRequestDetailDto`].
#[derive(Debug, Serialize, ToSchema)]
pub struct AdminOwnerRequestDto {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub phone: String,
    pub secondary_phone: Option<String>,
    pub status: OwnerRequestStatus,
    pub created_at: String,
    pub reviewed_at: Option<String>,
    pub admin_note: Option<String>,
}

impl From<AdminOwnerRequestRow> for AdminOwnerRequestDto {
    fn from(row: AdminOwnerRequestRow) -> Self {
        Self {
            id: row.id,
            email: row.email,
            full_name: applicant_full_name(row.first_name.as_deref(), row.last_name.as_deref()),
            phone: row.phone,
            secondary_phone: row.secondary_phone,
            status: row.status,
            created_at: row.created_at,
            reviewed_at: row.reviewed_at,
            admin_note: row.admin_note,
        }
    }
}

// ─────────────────────────────
// GET /admin/owner-requests/:id
// ─────────────────────────────

/// One request's full detail, including `identity_data` and every document
/// descriptor — only ever fetched one id at a time, never listed.
#[derive(Debug, Serialize, ToSchema)]
pub struct AdminOwnerRequestDetailDto {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub phone: String,
    pub secondary_phone: Option<String>,
    pub identity_data: ValidatedIdentityData,
    pub status: OwnerRequestStatus,
    pub created_at: String,
    pub reviewed_at: Option<String>,
    pub admin_note: Option<String>,
    pub documents: Vec<AdminOwnerRequestDocumentDto>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminOwnerRequestDetailResponse {
    pub data: AdminOwnerRequestDetailDto,
}

impl From<AdminOwnerRequestDetailRow> for AdminOwnerRequestDetailDto {
    fn from(row: AdminOwnerRequestDetailRow) -> Self {
        Self {
            id: row.id,
            email: row.email,
            full_name: applicant_full_name(row.first_name.as_deref(), row.last_name.as_deref()),
            phone: row.phone,
            secondary_phone: row.secondary_phone,
            identity_data: row.identity_data.0,
            status: row.status,
            created_at: row.created_at,
            reviewed_at: row.reviewed_at,
            admin_note: row.admin_note,
            documents: parse_documents(row.id, row.identity_documents),
        }
    }
}

fn applicant_full_name(first_name: Option<&str>, last_name: Option<&str>) -> String {
    match (first_name, last_name) {
        (Some(first), Some(last)) => format!("{first} {last}"),
        (None, Some(last)) => last.to_string(),
        (Some(first), None) => first.to_string(),
        (None, None) => String::new(),
    }
}

/// A parse failure here means a row written by our own code no longer
/// deserializes as expected — logged and treated as an empty document list
/// rather than failing the whole queue response.
fn parse_documents(request_id: Uuid, raw: serde_json::Value) -> Vec<AdminOwnerRequestDocumentDto> {
    match serde_json::from_value::<Vec<StoredDocumentEntry>>(raw) {
        Ok(entries) => entries.into_iter().map(Into::into).collect(),
        Err(error) => {
            tracing::warn!(%request_id, error = %error, "failed to parse identity_documents");
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(status: OwnerRequestStatus) -> OwnerRequestRow {
        OwnerRequestRow {
            id: Uuid::nil(),
            status,
            created_at: "2026-08-30T10:00:00Z".to_string(),
            reviewed_at: Some("2026-08-31T09:00:00Z".to_string()),
            admin_note: Some("Blurry ID photo".to_string()),
        }
    }

    #[test]
    fn admin_note_surfaces_only_when_rejected() {
        assert_eq!(
            OwnerRequestDto::from(row(OwnerRequestStatus::Rejected))
                .admin_note
                .as_deref(),
            Some("Blurry ID photo")
        );
        assert_eq!(
            OwnerRequestDto::from(row(OwnerRequestStatus::Pending)).admin_note,
            None
        );
        assert_eq!(
            OwnerRequestDto::from(row(OwnerRequestStatus::Approved)).admin_note,
            None
        );
    }

    #[test]
    fn status_response_serializes_null_data_when_no_request_exists() {
        let json = serde_json::to_value(OwnerRequestStatusResponse { data: None }).unwrap();
        assert_eq!(json, serde_json::json!({ "data": null }));
    }

    #[test]
    fn status_response_serializes_the_populated_request() {
        let dto = OwnerRequestDto::from(row(OwnerRequestStatus::Rejected));
        let json = serde_json::to_value(OwnerRequestStatusResponse { data: Some(dto) }).unwrap();
        let data = &json["data"];
        assert_eq!(data["status"], "rejected");
        assert_eq!(data["reviewed_at"], "2026-08-31T09:00:00Z");
        assert_eq!(data["admin_note"], "Blurry ID photo");
    }

    fn admin_row() -> AdminOwnerRequestRow {
        AdminOwnerRequestRow {
            id: Uuid::nil(),
            email: "awa@example.com".to_string(),
            first_name: Some("Awa".to_string()),
            last_name: Some("Diop".to_string()),
            phone: "+221771234567".to_string(),
            secondary_phone: None,
            status: OwnerRequestStatus::Pending,
            created_at: "2026-09-01T10:00:00Z".to_string(),
            reviewed_at: None,
            admin_note: None,
        }
    }

    fn admin_detail_row(identity_documents: serde_json::Value) -> AdminOwnerRequestDetailRow {
        AdminOwnerRequestDetailRow {
            id: Uuid::nil(),
            email: "awa@example.com".to_string(),
            first_name: Some("Awa".to_string()),
            last_name: Some("Diop".to_string()),
            phone: "+221771234567".to_string(),
            secondary_phone: None,
            identity_data: sqlx::types::Json(ValidatedIdentityData {
                full_name: "Awa Diop".to_string(),
                id_type: "passport".to_string(),
                id_number: "AB1234".to_string(),
            }),
            identity_documents,
            status: OwnerRequestStatus::Pending,
            created_at: "2026-09-01T10:00:00Z".to_string(),
            reviewed_at: None,
            admin_note: None,
        }
    }

    #[test]
    fn full_name_combines_first_and_last() {
        assert_eq!(applicant_full_name(Some("Awa"), Some("Diop")), "Awa Diop");
        assert_eq!(applicant_full_name(None, Some("Diop")), "Diop");
        assert_eq!(applicant_full_name(Some("Awa"), None), "Awa");
        assert_eq!(applicant_full_name(None, None), "");
    }

    #[test]
    fn admin_list_dto_omits_identity_data_and_documents() {
        let dto = AdminOwnerRequestDto::from(admin_row());
        let json = serde_json::to_value(&dto).unwrap();
        assert!(json.get("identity_data").is_none());
        assert!(json.get("documents").is_none());
        assert_eq!(json["full_name"], "Awa Diop");
    }

    #[test]
    fn admin_detail_dto_never_exposes_storage_key() {
        let documents = serde_json::json!([{
            "doc_id": "5b1f7e2a-3c4d-4e5f-8a9b-0c1d2e3f4a5b",
            "storage_key": "owner-requests/abc/front.jpg",
            "original_filename": "front.jpg",
            "content_type": "image/jpeg",
            "side": "front"
        }]);
        let dto = AdminOwnerRequestDetailDto::from(admin_detail_row(documents));
        let json = serde_json::to_value(&dto).unwrap();
        assert_eq!(json["documents"][0]["side"], "front");
        assert!(json["documents"][0].get("storage_key").is_none());
        assert_eq!(json["identity_data"]["full_name"], "Awa Diop");
    }

    #[test]
    fn admin_detail_dto_falls_back_to_empty_documents_on_malformed_json() {
        let dto =
            AdminOwnerRequestDetailDto::from(admin_detail_row(serde_json::json!("not an array")));
        assert!(dto.documents.is_empty());
    }
}
