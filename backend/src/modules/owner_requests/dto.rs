use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use super::model::{OwnerRequestRow, OwnerRequestStatus};

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
}
