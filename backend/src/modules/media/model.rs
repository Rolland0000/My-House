use bytes::Bytes;
use uuid::Uuid;

/// Mirrors the `RETURNING` clause of the `listing_media` insert — kept local
/// to this module rather than reusing `listings::model::ListingMediaRow`.
pub struct MediaRow {
    pub id: Uuid,
    pub url: String,
    pub is_cover: bool,
    pub position: i16,
}

/// A photo joined with its listing's owner, read under lock before deletion.
pub struct MediaForDeletion {
    pub listing_id: Uuid,
    pub owner_id: Uuid,
    pub storage_key: String,
    pub is_cover: bool,
}

/// Everything read off the multipart body, handed to the service as one value.
pub struct UploadMediaSubmission {
    pub listing_id: Uuid,
    pub bytes: Bytes,
}
