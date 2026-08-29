use axum::extract::FromRef;
use sqlx::PgPool;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::infra::cache::AppCacheProvider;
use crate::infra::mailer::Mailer;
use crate::modules::notifications;
use crate::shared::crypto;
use crate::shared::errors::AppError;
use crate::shared::rbac::Role;
use crate::shared::token_decoder::TokenClaims;
use crate::shared::types::{PendingOtp, RefreshTokenId};

use super::model::RefreshTokenLookup;
use super::repository;

pub struct RefreshOutcome {
    pub access_token: String,
    pub raw_refresh_token: String,
}

pub(in crate::modules::auth) struct RefreshConfig {
    jwt_secret: Vec<u8>,
    jwt_access_ttl_seconds: u64,
    pub jwt_refresh_ttl_days: u64,
}
impl FromRef<AppState> for RefreshConfig {
    fn from_ref(state: &AppState) -> Self {
        let config = state.config();
        Self {
            jwt_secret: config.jwt_secret.as_bytes().to_vec(),
            jwt_access_ttl_seconds: config.jwt_access_ttl_seconds,
            jwt_refresh_ttl_days: config.jwt_refresh_ttl_days,
        }
    }
}
/// Entry point wired by the handler.
pub (in crate::modules::auth) async fn refresh(
    pool: &PgPool,
    grace_cache: &dyn AppCacheProvider<RefreshTokenId, String>,
    config: &RefreshConfig,
    raw_token: &str,
) -> Result<RefreshOutcome, AppError> {
    let token_hash = crypto::hash_refresh_token(raw_token);
    let row = repository::find_by_hash(pool, &token_hash)
        .await?
        .ok_or(AppError::RefreshTokenInvalid)?;

    if row.is_revoked {
        return handle_revoked(
            pool,
            &row,
            grace_cache,
            &config.jwt_secret,
            config.jwt_access_ttl_seconds,
        )
        .await;
    }
    if row.is_expired {
        return Err(AppError::RefreshTokenInvalid);
    }

    let new_raw_token = Uuid::new_v4().to_string();
    let new_hash = crypto::hash_refresh_token(&new_raw_token);
    let row_id = RefreshTokenId::new(row.id);

    match repository::rotate(
        pool,
        row.id,
        row.user_id,
        &new_hash,
        config.jwt_refresh_ttl_days as i32,
    )
    .await?
    {
        Some(_new_id) => {
            grace_cache.insert(row_id, new_raw_token.clone()).await;
            Ok(RefreshOutcome {
                access_token: mint(
                    row.user_id,
                    row.role,
                    &config.jwt_secret,
                    config.jwt_access_ttl_seconds,
                )?,
                raw_refresh_token: new_raw_token,
            })
        }
        // Lost a concurrent rotation race — someone else already revoked
        // this row between our SELECT and our UPDATE. Re-check: it now sees
        // the winner's committed state and falls into the same reuse path
        // a sequential re-request would hit.
        None => {
            let row = repository::find_by_hash(pool, &token_hash)
                .await?
                .ok_or(AppError::RefreshTokenInvalid)?;
            handle_revoked(
                pool,
                &row,
                grace_cache,
                &config.jwt_secret,
                config.jwt_access_ttl_seconds,
            )
            .await
        }
    }
}

async fn handle_revoked(
    pool: &PgPool,
    row: &RefreshTokenLookup,
    grace_cache: &dyn AppCacheProvider<RefreshTokenId, String>,
    jwt_secret: &[u8],
    jwt_access_ttl_seconds: u64,
) -> Result<RefreshOutcome, AppError> {
    let row_id = RefreshTokenId::new(row.id);
    if let Some(raw_replacement) = grace_cache.get(&row_id).await {
        return Ok(RefreshOutcome {
            access_token: mint(row.user_id, row.role, jwt_secret, jwt_access_ttl_seconds)?,
            raw_refresh_token: raw_replacement,
        });
    }
    tracing::warn!(user_id = %row.user_id, "refresh token reuse outside grace window — revoking token family");
    repository::revoke_all_for_user(pool, row.user_id).await?;
    Err(AppError::RefreshTokenInvalid)
}

/// Entry point wired by the handler. Revokes only the single current token
/// — never the whole family, unlike [`handle_revoked`]'s theft response.
pub async fn logout(pool: &PgPool, raw_token: &str) -> Result<(), AppError> {
    let token_hash = crypto::hash_refresh_token(raw_token);
    repository::revoke(pool, &token_hash).await
}

fn mint(user_id: Uuid, role: Role, secret: &[u8], ttl_seconds: u64) -> Result<String, AppError> {
    crypto::issue_access_token(TokenClaims { user_id, role }, secret, ttl_seconds)
}

/// Entry point wired by the handler. Never reveals to the caller whether
/// `email` is already registered — every branch returns the same `Ok(())`.
pub async fn otp_request(
    pool: &PgPool,
    otp_cache: &dyn AppCacheProvider<String, PendingOtp>,
    rate_limit_cache: &dyn AppCacheProvider<String, ()>,
    mailer: &Mailer,
    otp_ttl_seconds: u64,
    email: &str,
) -> Result<(), AppError> {
    if rate_limit_cache.get(&email.to_string()).await.is_some() {
        return Err(AppError::OtpRateLimited);
    }

    let is_new = !repository::email_exists(pool, email).await?;
    let raw_code = crypto::generate_otp_code();
    let code_hash = crypto::hash_otp_code(&raw_code);

    otp_cache
        .insert(
            email.to_string(),
            PendingOtp {
                code_hash,
                is_new,
                attempts: 0,
            },
        )
        .await;
    rate_limit_cache.insert(email.to_string(), ()).await;

    notifications::service::send_otp_email(mailer, email, &raw_code, otp_ttl_seconds / 60).await;

    Ok(())
}

pub struct VerifyOutcome {
    pub access_token: String,
    pub raw_refresh_token: String,
    pub is_new_user: bool,
}

pub(in crate::modules::auth) struct OtpVerifyConfig {
    jwt_secret: Vec<u8>,
    jwt_access_ttl_seconds: u64,
    pub jwt_refresh_ttl_days: u64,
    otp_max_attempts: u32,
}
impl FromRef<AppState> for OtpVerifyConfig {
    fn from_ref(state: &AppState) -> Self {
        let config = state.config();
        Self {
            jwt_secret: config.jwt_secret.as_bytes().to_vec(),
            jwt_access_ttl_seconds: config.jwt_access_ttl_seconds,
            jwt_refresh_ttl_days: config.jwt_refresh_ttl_days,
            otp_max_attempts: config.otp_max_attempts,
        }
    }
}
/// Entry point wired by the handler.
pub(in crate::modules::auth) async fn verify_otp(
    pool: &PgPool,
    otp_cache: &dyn AppCacheProvider<String, PendingOtp>,
    mailer: &Mailer,
    config: &OtpVerifyConfig,
    email: &str,
    code: &str,
) -> Result<VerifyOutcome, AppError> {
    let entry = otp_cache
        .get(&email.to_string())
        .await
        .ok_or(AppError::OtpInvalid)?;

    // Constant-time-ish comparison isn't the point here — both sides are
    // already SHA-256 digests, so a length/prefix leak reveals nothing about
    // the raw code.
    if crypto::hash_otp_code(code) != entry.code_hash {
        let attempts = entry.attempts + 1;
        if attempts >= config.otp_max_attempts {
            // 3rd (or config-configured Nth) mismatch permanently
            // invalidates the code — deleting the entry, not just marking it
            // exhausted, so a later correct-code retry also fails (AUTH-06).
            otp_cache.invalidate(&email.to_string()).await;
        } else {
            otp_cache
                .insert(email.to_string(), PendingOtp { attempts, ..entry })
                .await;
        }
        return Err(AppError::OtpInvalid);
    }

    // Single-use: the entry is gone before any account/session mutation
    // happens, so a replayed second call with the same code can never
    // succeed twice.
    otp_cache.invalidate(&email.to_string()).await;

    let (user_id, role, is_new_user) = if entry.is_new {
        // A concurrent verify call for the same still-valid code (double
        // click, client retry) can win the `users.email` unique constraint
        // first. Fall back to the row it created instead of surfacing that
        // race as a 500 to the loser — same contract as `rotate`'s lost race.
        let user_id = match repository::create_seeker(pool, email).await? {
            Some(user_id) => {
                notifications::service::send_welcome_email(mailer, email, email).await;
                user_id
            }
            None => {
                repository::find_user_by_email(pool, email)
                    .await?
                    .ok_or(AppError::OtpInvalid)?
                    .0
            }
        };
        (user_id, Role::Seeker, true)
    } else {
        let (user_id, role) = repository::find_user_by_email(pool, email)
            .await?
            .ok_or(AppError::OtpInvalid)?;
        (user_id, role, false)
    };

    let access_token = mint(
        user_id,
        role,
        config.jwt_secret.as_slice(),
        config.jwt_access_ttl_seconds,
    )?;

    let raw_refresh_token = Uuid::new_v4().to_string();
    let refresh_token_hash = crypto::hash_refresh_token(&raw_refresh_token);
    repository::insert_refresh_token(
        pool,
        user_id,
        &refresh_token_hash,
        config.jwt_refresh_ttl_days as i32,
    )
    .await?;

    Ok(VerifyOutcome {
        access_token,
        raw_refresh_token,
        is_new_user,
    })
}
