use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use super::model::{OwnerRequestRow, OwnerRequestStatus};

#[derive(Debug, Serialize, ToSchema)]
pub struct OwnerRequestDto {
    pub id: Uuid,
    pub status: OwnerRequestStatus,
    pub created_at: String,
}

impl From<OwnerRequestRow> for OwnerRequestDto {
    fn from(row: OwnerRequestRow) -> Self {
        Self {
            id: row.id,
            status: row.status,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OwnerRequestResponse {
    pub data: OwnerRequestDto,
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
