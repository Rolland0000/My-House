use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct OtpRequestDto {
    pub email: String,
}

/// `{ "data": { "message": "..." } }` — identical shape whether `email` is
/// known or unknown, so the response can never be used to enumerate
/// registered accounts.
#[derive(Debug, Serialize, ToSchema)]
pub struct OtpRequestMessageDto {
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OtpRequestResponse {
    pub data: OtpRequestMessageDto,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RefreshTokenDto {
    pub access_token: String,
}

/// `{ "data": { "access_token": "..." } }` — matches the envelope used
/// elsewhere (e.g. `ListingDetailResponse`) and by the sibling
/// `/auth/otp/verify` endpoint's actual JSON example.
#[derive(Debug, Serialize, ToSchema)]
pub struct RefreshResponse {
    pub data: RefreshTokenDto,
}
