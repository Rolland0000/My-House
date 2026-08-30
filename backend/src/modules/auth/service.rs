use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use crate::infra::cache::AppCacheProvider;
use crate::infra::mailer::Mailer;
use crate::modules::notifications;
use crate::shared::crypto;
use crate::shared::errors::AppError;
use crate::shared::rbac::Role;
use crate::shared::token_decoder::TokenClaims;
use crate::shared::types::{otp_key, registration_key, AuthChallenge, PendingOtp, RefreshTokenId};
use crate::shared::validation::{optional_name, required_name, required_phone};

use super::model::{NewAccount, RefreshTokenLookup};
use super::repository;

pub struct SessionOutcome {
    pub access_token: String,
    pub raw_refresh_token: String,
}

pub(in crate::modules::auth) struct SessionConfig {
    jwt_secret: Arc<[u8]>,
    jwt_access_ttl_seconds: u64,
    pub jwt_refresh_ttl_days: u64,
}

impl SessionConfig {
    pub(in crate::modules::auth) fn new(
        jwt_secret: Arc<[u8]>,
        jwt_access_ttl_seconds: u64,
        jwt_refresh_ttl_days: u64,
    ) -> Self {
        Self {
            jwt_secret,
            jwt_access_ttl_seconds,
            jwt_refresh_ttl_days,
        }
    }
}

/// Entry point wired by the handler.
pub(in crate::modules::auth) async fn refresh(
    pool: &PgPool,
    grace_cache: &dyn AppCacheProvider<RefreshTokenId, String>,
    config: &SessionConfig,
    raw_token: &str,
) -> Result<SessionOutcome, AppError> {
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
            Ok(SessionOutcome {
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
) -> Result<SessionOutcome, AppError> {
    let row_id = RefreshTokenId::new(row.id);
    if let Some(raw_replacement) = grace_cache.get(&row_id).await {
        return Ok(SessionOutcome {
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
    challenge_cache: &dyn AppCacheProvider<String, AuthChallenge>,
    rate_limit_cache: &dyn AppCacheProvider<String, ()>,
    mailer: &Mailer,
    otp_ttl_seconds: u64,
    otp_rate_limit_seconds: u64,
    email: &str,
) -> Result<(), AppError> {
    if rate_limit_cache.get(&email.to_string()).await.is_some() {
        return Err(AppError::OtpRateLimited {
            retry_after_seconds: otp_rate_limit_seconds,
        });
    }

    let is_new = !repository::email_exists(pool, email).await?;
    let raw_code = crypto::generate_otp_code();
    let code_hash = crypto::hash_otp_code(&raw_code);

    challenge_cache
        .insert(
            otp_key(email),
            AuthChallenge::PendingOtp(PendingOtp {
                code_hash,
                is_new,
                attempts: 0,
            }),
        )
        .await;
    rate_limit_cache.insert(email.to_string(), ()).await;

    notifications::service::send_otp_email(mailer, email, &raw_code, otp_ttl_seconds / 60).await;

    Ok(())
}

/// Known email → session. Unknown email → ticket for `/auth/register`, with
/// nothing written to the database.
pub enum VerifyOutcome {
    Session {
        access_token: String,
        raw_refresh_token: String,
    },
    RegistrationRequired {
        registration_ticket: String,
    },
}

pub(in crate::modules::auth) struct OtpVerifyConfig {
    jwt_secret: Arc<[u8]>,
    jwt_access_ttl_seconds: u64,
    pub jwt_refresh_ttl_days: u64,
    otp_max_attempts: u32,
}

impl OtpVerifyConfig {
    pub(in crate::modules::auth) fn new(
        jwt_secret: Arc<[u8]>,
        jwt_access_ttl_seconds: u64,
        jwt_refresh_ttl_days: u64,
        otp_max_attempts: u32,
    ) -> Self {
        Self {
            jwt_secret,
            jwt_access_ttl_seconds,
            jwt_refresh_ttl_days,
            otp_max_attempts,
        }
    }
}

/// Entry point wired by the handler.
pub(in crate::modules::auth) async fn verify_otp(
    pool: &PgPool,
    challenge_cache: &dyn AppCacheProvider<String, AuthChallenge>,
    config: &OtpVerifyConfig,
    email: &str,
    code: &str,
) -> Result<VerifyOutcome, AppError> {
    let key = otp_key(email);
    let AuthChallenge::PendingOtp(entry) = challenge_cache
        .get(&key)
        .await
        .ok_or(AppError::OtpInvalid)?
    else {
        return Err(AppError::OtpInvalid);
    };

    // Constant-time-ish comparison isn't the point here — both sides are
    // already SHA-256 digests, so a length/prefix leak reveals nothing about
    // the raw code.
    if crypto::hash_otp_code(code) != entry.code_hash {
        let attempts = entry.attempts + 1;
        if attempts >= config.otp_max_attempts {
            // 3rd (or config-configured Nth) mismatch permanently
            // invalidates the code — deleting the entry, not just marking it
            // exhausted, so a later correct-code retry also fails (AUTH-06).
            challenge_cache.invalidate(&key).await;
        } else {
            challenge_cache
                .insert(
                    key,
                    AuthChallenge::PendingOtp(PendingOtp { attempts, ..entry }),
                )
                .await;
        }
        return Err(AppError::OtpInvalid);
    }

    // Single-use: the entry is gone before any account/session mutation
    // happens, so a replayed second call with the same code can never
    // succeed twice.
    challenge_cache.invalidate(&key).await;

    if entry.is_new {
        let registration_ticket = Uuid::new_v4().to_string();
        challenge_cache
            .insert(
                registration_key(&registration_ticket),
                AuthChallenge::PendingRegistration {
                    email: email.to_string(),
                },
            )
            .await;
        return Ok(VerifyOutcome::RegistrationRequired {
            registration_ticket,
        });
    }

    let (user_id, role) = repository::find_user_by_email(pool, email)
        .await?
        .ok_or(AppError::OtpInvalid)?;

    let access_token = mint(
        user_id,
        role,
        &config.jwt_secret,
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

    Ok(VerifyOutcome::Session {
        access_token,
        raw_refresh_token,
    })
}

/// Fields accepted by `/auth/register`, already unwrapped from the DTO.
pub(in crate::modules::auth) struct RegisterInput<'a> {
    pub registration_ticket: &'a str,
    pub first_name: Option<&'a str>,
    pub last_name: &'a str,
    pub phone: &'a str,
}

/// Entry point wired by the handler. Consumes the ticket minted by
/// [`verify_otp`] and creates the account — the email is taken from the
/// ticket, never from the request body, so it stays the one the OTP proved.
pub(in crate::modules::auth) async fn register(
    pool: &PgPool,
    challenge_cache: &dyn AppCacheProvider<String, AuthChallenge>,
    mailer: &Mailer,
    config: &SessionConfig,
    input: RegisterInput<'_>,
) -> Result<SessionOutcome, AppError> {
    let key = registration_key(input.registration_ticket);
    let AuthChallenge::PendingRegistration { email } = challenge_cache
        .get(&key)
        .await
        .ok_or(AppError::OtpInvalid)?
    else {
        return Err(AppError::OtpInvalid);
    };

    let first_name = optional_name(input.first_name, "first_name")?;
    let last_name = required_name(input.last_name, "last_name")?;
    let phone = required_phone(input.phone)?;

    // Single-use, consumed before any write; a rejected payload above leaves
    // the ticket usable for the retry.
    challenge_cache.invalidate(&key).await;

    let raw_refresh_token = Uuid::new_v4().to_string();
    let refresh_token_hash = crypto::hash_refresh_token(&raw_refresh_token);

    let user_id = repository::create_account(
        pool,
        NewAccount {
            email: &email,
            first_name,
            last_name,
            phone,
            refresh_token_hash: &refresh_token_hash,
            refresh_ttl_days: config.jwt_refresh_ttl_days as i32,
        },
    )
    .await?;

    let display_name = first_name.unwrap_or(last_name);
    notifications::service::send_welcome_email(mailer, &email, display_name).await;

    Ok(SessionOutcome {
        access_token: mint(
            user_id,
            Role::Seeker,
            &config.jwt_secret,
            config.jwt_access_ttl_seconds,
        )?,
        raw_refresh_token,
    })
}
