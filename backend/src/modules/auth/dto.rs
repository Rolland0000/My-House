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

#[derive(Debug, Deserialize, ToSchema)]
pub struct OtpVerifyDto {
    pub email: String,
    pub code: String,
}

/// `is_new_user` discriminates the two shapes: `false` carries `access_token`,
/// `true` carries `registration_ticket` for `POST /auth/register`.
#[derive(Debug, Serialize, ToSchema)]
pub struct OtpVerifyTokenDto {
    pub is_new_user: bool,
    pub access_token: Option<String>,
    pub registration_ticket: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OtpVerifyResponse {
    pub data: OtpVerifyTokenDto,
}

/// `last_name` and `phone` are mandatory for every non-admin account.
#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterDto {
    pub registration_ticket: String,
    pub first_name: Option<String>,
    pub last_name: String,
    pub phone: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterTokenDto {
    pub access_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub data: RegisterTokenDto,
}
