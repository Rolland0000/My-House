use crate::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::extractors::AuthUser;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};

use super::dto::{
    OtpRequestDto, OtpRequestMessageDto, OtpRequestResponse, OtpVerifyDto, OtpVerifyResponse,
    OtpVerifyTokenDto, RefreshResponse, RefreshTokenDto, RegisterDto, RegisterResponse,
    RegisterTokenDto,
};
use super::service;

pub const REFRESH_TOKEN_COOKIE: &str = "refresh_token";

/// Single definition of the refresh cookie's attributes, shared by every
/// endpoint that opens or rotates a session.
fn refresh_cookie(raw_token: String, cookie_domain: String, ttl_days: u64) -> Cookie<'static> {
    Cookie::build((REFRESH_TOKEN_COOKIE, raw_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .domain(cookie_domain)
        .path("/api/v1/auth")
        .max_age(time::Duration::seconds((ttl_days * 86_400) as i64))
        .build()
}

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
        state.cache().auth_challenge().as_ref(),
        state.cache().otp_rate_limit().as_ref(),
        state.mailer(),
        config.otp_ttl_seconds,
        config.otp_rate_limit_seconds,
        &payload.email,
    )
    .await?;

    Ok(Json(OtpRequestResponse {
        data: OtpRequestMessageDto {
            message: "OTP code sent".to_string(),
        },
    }))
}

/// Second half of the unified auth endpoint (seeker_auth_flow.mermaid §2):
/// verifies the code cached by `/auth/otp/request`. A known email gets its
/// session straight away; an unknown one gets a single-use registration
/// ticket — no account row and no cookie until `/auth/register` runs.
#[utoipa::path(
    post,
    path = "/auth/otp/verify",
    tag = "auth",
    request_body = OtpVerifyDto,
    responses(
        (status = 200, description = "OTP verified; session issued, or registration ticket returned for an unknown email", body = OtpVerifyResponse),
        (status = 401, description = "Invalid, expired, or attempt-exhausted OTP code"),
    )
)]
pub async fn otp_verify(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<OtpVerifyDto>,
) -> Result<(CookieJar, Json<OtpVerifyResponse>), AppError> {
    let app_config = state.config();
    let cookie_domain = app_config.cookie_domain.clone();
    let config = service::OtpVerifyConfig::new(
        state.jwt_secret().clone(),
        app_config.jwt_access_ttl_seconds,
        app_config.jwt_refresh_ttl_days,
        app_config.otp_max_attempts,
    );
    let outcome = service::verify_otp(
        state.db(),
        state.cache().auth_challenge().as_ref(),
        &config,
        &payload.email,
        &payload.code,
    )
    .await?;

    let (jar, data) = match outcome {
        service::VerifyOutcome::Session {
            access_token,
            raw_refresh_token,
        } => (
            jar.add(refresh_cookie(
                raw_refresh_token,
                cookie_domain,
                config.jwt_refresh_ttl_days,
            )),
            OtpVerifyTokenDto {
                is_new_user: false,
                access_token: Some(access_token),
                registration_ticket: None,
            },
        ),
        service::VerifyOutcome::RegistrationRequired {
            registration_ticket,
        } => (
            jar,
            OtpVerifyTokenDto {
                is_new_user: true,
                access_token: None,
                registration_ticket: Some(registration_ticket),
            },
        ),
    };

    Ok((jar, Json(OtpVerifyResponse { data })))
}

/// Creates the account in one transaction from the ticket handed out by
/// `/auth/otp/verify`, and opens the session.
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = RegisterDto,
    responses(
        (status = 200, description = "Account created; session issued", body = RegisterResponse),
        (status = 400, description = "Missing or over-long last_name / phone"),
        (status = 401, description = "Unknown, expired, or already-used registration ticket"),
        (status = 409, description = "An account already exists for this email"),
    )
)]
pub async fn register(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<RegisterDto>,
) -> Result<(CookieJar, Json<RegisterResponse>), AppError> {
    let app_config = state.config();
    let cookie_domain = app_config.cookie_domain.clone();
    let config = service::SessionConfig::new(
        state.jwt_secret().clone(),
        app_config.jwt_access_ttl_seconds,
        app_config.jwt_refresh_ttl_days,
    );
    let outcome = service::register(
        state.db(),
        state.cache().auth_challenge().as_ref(),
        state.mailer(),
        &config,
        service::RegisterInput {
            registration_ticket: &payload.registration_ticket,
            first_name: payload.first_name.as_deref(),
            last_name: &payload.last_name,
            phone: &payload.phone,
        },
    )
    .await?;

    Ok((
        jar.add(refresh_cookie(
            outcome.raw_refresh_token,
            cookie_domain,
            config.jwt_refresh_ttl_days,
        )),
        Json(RegisterResponse {
            data: RegisterTokenDto {
                access_token: outcome.access_token,
            },
        }),
    ))
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

    let app_config = state.config();
    let cookie_domain = app_config.cookie_domain.clone();
    let config = service::SessionConfig::new(
        state.jwt_secret().clone(),
        app_config.jwt_access_ttl_seconds,
        app_config.jwt_refresh_ttl_days,
    );
    let outcome = service::refresh(
        state.db(),
        state.cache().refresh_replay().as_ref(),
        &config,
        &raw_token,
    )
    .await?;

    Ok((
        jar.add(refresh_cookie(
            outcome.raw_refresh_token,
            cookie_domain,
            config.jwt_refresh_ttl_days,
        )),
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
        .path("/api/v1/auth")
        .build();

    Ok(jar.remove(removal_cookie))
}
