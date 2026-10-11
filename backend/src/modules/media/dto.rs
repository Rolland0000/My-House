use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::model::{MediaRow, OrderedMediaRow};

#[derive(Debug, Serialize, ToSchema)]
pub struct MediaDto {
    pub id: Uuid,
    pub url: String,
    pub is_cover: bool,
    pub position: i16,
}

impl From<MediaRow> for MediaDto {
    fn from(row: MediaRow) -> Self {
        Self {
            id: row.id,
            url: row.url,
            is_cover: row.is_cover,
            position: row.position,
        }
    }
}

impl From<OrderedMediaRow> for MediaDto {
    fn from(row: OrderedMediaRow) -> Self {
        Self {
            id: row.id,
            url: row.url,
            is_cover: row.is_cover,
            position: row.position,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MediaResponse {
    pub data: MediaDto,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MediaListResponse {
    pub data: Vec<MediaDto>,
}

/// A missing or `null` list deserializes to `None`, which the service reports
/// as a 422 on `media_ids`.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ReorderMediaRequest {
    pub media_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PromoteCoverRequest {
    pub media_id: Uuid,
}

/// Documents the upload body for the OpenAPI schema. Nothing deserializes
/// into it — the handler walks the multipart fields itself.
#[derive(ToSchema)]
pub struct UploadMediaForm {
    pub listing_id: String,
    #[schema(value_type = String, format = Binary)]
    pub file: String,
}
