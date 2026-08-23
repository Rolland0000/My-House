use axum::extract::State;
use axum::Json;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};

use crate::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::extractors::AuthUser;

use super::dto::{
    OtpRequestDto, OtpRequestMessageDto, OtpRequestResponse, RefreshResponse, RefreshTokenDto,
};
use super::service;

pub const REFRESH_TOKEN_COOKIE: &str = "refresh_token";

/// Identical 200 response whether `payload.email` is registered or not —
/// the endpoint must never let a caller enumerate accounts.
#[utoipa::path(
    post,
    path = "/auth/otp/request",
    tag = "auth",
    request_body = OtpRequestDto,
    responses(
        (status = 200, description = "OTP code generated and emailed", body = OtpRequestResponse),
        (status = 429, description = "Too many requests for this email within the rate-limit window"),
    )
)]
pub async fn otp_request(
    State(state): State<AppState>,
    Json(payload): Json<OtpRequestDto>,
) -> Result<Json<OtpRequestResponse>, AppError> {
    let config = state.config();
    service::otp_request(
        state.db(),
        state.cache().otp().as_ref(),
        state.cache().otp_rate_limit().as_ref(),
        state.mailer(),
        config.otp_ttl_seconds,
        &payload.email,
    )
    .await?;

    Ok(Json(OtpRequestResponse {
        data: OtpRequestMessageDto {
            message: "OTP code sent".to_string(),
        },
    }))
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "auth",
    responses(
        (status = 200, description = "New access token issued; refresh cookie rotated", body = RefreshResponse),
        (status = 401, description = "Missing, unknown, expired, or reused refresh token"),
    )
)]
pub async fn refresh(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, Json<RefreshResponse>), AppError> {
    let raw_token = jar
        .get(REFRESH_TOKEN_COOKIE)
        .map(|c| c.value().to_string())
        .ok_or(AppError::RefreshTokenInvalid)?;

    let config = state.config();
    let outcome = service::refresh(
        state.db(),
        state.cache().refresh_replay().as_ref(),
        config.jwt_secret.as_bytes(),
        config.jwt_access_ttl_seconds,
        config.jwt_refresh_ttl_days,
        &raw_token,
    )
    .await?;

    let max_age_seconds = (config.jwt_refresh_ttl_days * 86_400) as i64;
    let cookie = Cookie::build((REFRESH_TOKEN_COOKIE, outcome.raw_refresh_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .domain(config.cookie_domain.clone())
        .path("/auth")
        .max_age(time::Duration::seconds(max_age_seconds))
        .build();

    Ok((
        jar.add(cookie),
        Json(RefreshResponse {
            data: RefreshTokenDto {
                access_token: outcome.access_token,
            },
        }),
    ))
}

/// The access token itself keeps authenticating requests until its own
/// (max 15 min) expiry — MH-35's stateless JWTs can't be invalidated early.
/// Logout only guarantees the session can't be *refreshed* again.
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "auth",
    responses(
        (status = 200, description = "Current refresh token revoked; cookie cleared"),
        (status = 401, description = "Missing or invalid access token"),
    )
)]
pub async fn logout(
    State(state): State<AppState>,
    _user: AuthUser,
    jar: CookieJar,
) -> Result<CookieJar, AppError> {
    if let Some(raw_token) = jar.get(REFRESH_TOKEN_COOKIE).map(|c| c.value().to_string()) {
        service::logout(state.db(), &raw_token).await?;
    }

    let config = state.config();
    let removal_cookie = Cookie::build(REFRESH_TOKEN_COOKIE)
        .domain(config.cookie_domain.clone())
        .path("/auth")
        .build();

    Ok(jar.remove(removal_cookie))
}
