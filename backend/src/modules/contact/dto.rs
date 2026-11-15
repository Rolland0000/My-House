use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Phone numbers of a listing's owner, exactly as stored.
#[derive(Debug, Serialize, ToSchema)]
pub struct ContactDto {
    /// The owner's profile phone.
    pub phone: String,
    /// Secondary number from the owner's most recent approved owner request.
    #[schema(required = true)]
    pub secondary_phone: Option<String>,
}

/// Envelope for `GET /listings/:id/contact`.
#[derive(Debug, Serialize, ToSchema)]
pub struct ContactResponse {
    pub data: ContactDto,
}

/// Body of `POST /contact/messages`.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ContactMessageDto {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ContactMessageAck {
    pub message: String,
}

/// Envelope for `POST /contact/messages`.
#[derive(Debug, Serialize, ToSchema)]
pub struct ContactMessageResponse {
    pub data: ContactMessageAck,
}
