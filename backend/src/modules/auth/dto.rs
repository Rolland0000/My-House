use serde::Serialize;
use utoipa::ToSchema;

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
