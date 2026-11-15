use std::sync::Arc;
use std::time::{Duration, Instant};

use sqlx::PgPool;
use uuid::Uuid;

use crate::infra::cache::AppCacheProvider;
use crate::infra::mailer::Mailer;
use crate::modules::notifications;
use crate::shared::crypto;
use crate::shared::errors::{AppError, FieldError};
use crate::shared::rbac::Role;
use crate::shared::token_decoder::TokenClaims;
use crate::shared::types::{
    otp_key, registration_key, AuthChallenge, OtpAttemptWindow, PendingOtp, RefreshTokenId,
};
use crate::shared::validation::{normalize_email, optional_name, required_name, required_phone};

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
    let row = find_active_row(pool, &token_hash).await?;

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
            purge_stale_tokens(pool, row.user_id).await;
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
            let row = find_active_row(pool, &token_hash).await?;
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

/// Days an expired refresh token is kept before being purged.
const REFRESH_TOKEN_RETENTION_DAYS: i32 = 30;

/// Opportunistic cleanup on each rotation. Best-effort: a failure leaves
/// stale rows for the next rotation and never fails the refresh.
async fn purge_stale_tokens(pool: &PgPool, user_id: Uuid) {
    if let Err(error) =
        repository::purge_stale_tokens(pool, user_id, REFRESH_TOKEN_RETENTION_DAYS).await
    {
        tracing::warn!(%user_id, error = %error, "refresh: purge of expired tokens failed");
    }
}

/// Refuses a suspended account before the revoked and expired checks: all its
/// tokens get the same answer, and the grace window reissues nothing.
async fn find_active_row(pool: &PgPool, token_hash: &str) -> Result<RefreshTokenLookup, AppError> {
    let row = repository::find_by_hash(pool, token_hash)
        .await?
        .ok_or(AppError::RefreshTokenInvalid)?;

    if !row.is_active {
        return Err(AppError::AccountSuspended);
    }
    Ok(row)
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
        // A logout or a family revocation after the rotation ends the grace:
        // replaying the old token must not hand out a fresh access token.
        if row.replacement_revoked {
            return Err(AppError::RefreshTokenInvalid);
        }
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

/// Longest wait imposed between two codes for one email, however many
/// attempts failed. Bounds how long a third party can delay a sign-in.
const OTP_MAX_REQUEST_COOLDOWN: Duration = Duration::from_secs(5 * 60);

/// Minimum gap between two codes: the base cooldown, doubled for every full
/// `max_attempts` attempts already spent, capped at [`OTP_MAX_REQUEST_COOLDOWN`].
fn request_cooldown(base_seconds: u64, attempts: u32, max_attempts: u32) -> Duration {
    let exhausted_codes = attempts / max_attempts.max(1);
    let factor = 1u64.checked_shl(exhausted_codes).unwrap_or(u64::MAX);
    let base = Duration::from_secs(base_seconds);
    Duration::from_secs(base_seconds.saturating_mul(factor))
        .min(OTP_MAX_REQUEST_COOLDOWN)
        .max(base)
}

/// Whole seconds, rounded up and at least 1, for a `Retry-After` header.
fn retry_after_seconds(remaining: Duration) -> u64 {
    (remaining.as_secs() + u64::from(remaining.subsec_nanos() > 0)).max(1)
}

pub struct OtpRequestConfig {
    pub otp_ttl_seconds: u64,
    pub otp_rate_limit_seconds: u64,
    pub otp_max_attempts: u32,
}

/// Entry point wired by the handler. Never reveals to the caller whether
/// `email` is already registered — every branch returns the same `Ok(())`.
pub async fn otp_request(
    pool: &PgPool,
    challenge_cache: &dyn AppCacheProvider<String, AuthChallenge>,
    rate_limit_cache: &dyn AppCacheProvider<String, ()>,
    attempt_cache: &dyn AppCacheProvider<String, Arc<OtpAttemptWindow>>,
    mailer: &Mailer,
    config: &OtpRequestConfig,
    email: &str,
) -> Result<(), AppError> {
    let email = normalize_email(email).ok_or_else(|| {
        AppError::Validation(vec![FieldError {
            field: "email",
            message: "email is not a valid address.".to_string(),
        }])
    })?;
    let email = email.as_str();

    if rate_limit_cache.get(&email.to_string()).await.is_some() {
        return Err(AppError::OtpRateLimited {
            retry_after_seconds: config.otp_rate_limit_seconds,
        });
    }
    let attempt_window = attempt_cache
        .get_or_insert(email.to_string(), Arc::default())
        .await;
    // Failed attempts survive a new code, so each spent code makes the next
    // one wait longer: guessing stays bounded without ever locking the account.
    let now = Instant::now();
    let attempts_at_issue = attempt_window.attempts();
    if let Some(last_issued_at) = attempt_window.last_code_issued_at() {
        let cooldown = request_cooldown(
            config.otp_rate_limit_seconds,
            attempts_at_issue,
            config.otp_max_attempts,
        );
        let remaining = (last_issued_at + cooldown).saturating_duration_since(now);
        if !remaining.is_zero() {
            tracing::warn!(
                attempts = attempts_at_issue,
                retry_after_seconds = remaining.as_secs(),
                "otp: code withheld after repeated failed attempts"
            );
            return Err(AppError::OtpRateLimited {
                retry_after_seconds: retry_after_seconds(remaining),
            });
        }
    }

    let existing = repository::find_user_by_email(pool, email).await?;
    // Admins sign in with a password only. Answering like any other request
    // keeps the response from revealing which email belongs to an admin.
    if matches!(existing, Some((_, Role::Admin, _))) {
        return Ok(());
    }
    let is_new = existing.is_none();
    let raw_code = crypto::generate_otp_code();
    let code_hash = crypto::hash_otp_code(&raw_code);

    attempt_window.record_code_issued(now);
    challenge_cache
        .insert(
            otp_key(email),
            AuthChallenge::PendingOtp(PendingOtp {
                code_hash,
                is_new,
                expires_at: now + Duration::from_secs(config.otp_ttl_seconds),
                attempt_window,
                attempts_at_issue,
            }),
        )
        .await;
    rate_limit_cache.insert(email.to_string(), ()).await;

    notifications::service::send_otp_email(mailer, email, &raw_code, config.otp_ttl_seconds / 60)
        .await;

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
    // No OTP can have been issued for an address `otp_request` would refuse.
    let email = normalize_email(email).ok_or(AppError::OtpInvalid)?;
    let email = email.as_str();
    let key = otp_key(email);
    let AuthChallenge::PendingOtp(entry) = challenge_cache
        .get(&key)
        .await
        .ok_or(AppError::OtpInvalid)?
    else {
        return Err(AppError::OtpInvalid);
    };

    if Instant::now() >= entry.expires_at {
        challenge_cache.invalidate(&key).await;
        return Err(AppError::OtpInvalid);
    }

    // Counted before the comparison: concurrent guesses each take their own
    // slot, so no code is ever compared more than `otp_max_attempts` times.
    let used = entry
        .attempt_window
        .reserve_attempt()
        .saturating_sub(entry.attempts_at_issue);
    if used > config.otp_max_attempts {
        challenge_cache.invalidate(&key).await;
        return Err(AppError::OtpInvalid);
    }

    // Timing is not the defence here: both sides are SHA-256 digests, and the
    // attempt budget above is what bounds guessing.
    if crypto::hash_otp_code(code) != entry.code_hash {
        if used == config.otp_max_attempts {
            // The last allowed mismatch deletes the code, so a later
            // correct-code retry also fails (AUTH-06).
            challenge_cache.invalidate(&key).await;
        }
        return Err(AppError::OtpInvalid);
    }

    // Single-use: the entry is gone before any account/session mutation
    // happens, so a replayed second call with the same code can never
    // succeed twice.
    challenge_cache.invalidate(&key).await;
    entry.attempt_window.reset();

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

    let (user_id, role, is_active) = repository::find_user_by_email(pool, email)
        .await?
        .ok_or(AppError::OtpInvalid)?;

    // Defence in depth: `otp_request` never issues a code for an admin.
    if role == Role::Admin {
        return Err(AppError::OtpInvalid);
    }

    // Only here, once the code is proven: refusing earlier would reveal the
    // suspension to anyone who knows the email.
    if !is_active {
        return Err(AppError::AccountSuspended);
    }

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

/// Wrong passwords tolerated before the admin account locks.
const ADMIN_LOGIN_MAX_ATTEMPTS: i32 = 5;
/// How long the admin account stays locked once the limit is reached.
const ADMIN_LOGIN_LOCK_MINUTES: i32 = 15;
const ADMIN_PASSWORD_MAX_LENGTH: usize = 1024;

/// Entry point wired by the handler. Unknown email, non-admin account, account
/// without a password and wrong password all answer
/// [`AppError::InvalidCredentials`]; an unknown email still pays the hashing
/// cost so timing doesn't tell them apart.
pub(in crate::modules::auth) async fn admin_login(
    pool: &PgPool,
    config: &SessionConfig,
    email: &str,
    password: &str,
) -> Result<SessionOutcome, AppError> {
    let Some(email) = normalize_email(email) else {
        return Err(AppError::InvalidCredentials);
    };
    if password.is_empty() || password.len() > ADMIN_PASSWORD_MAX_LENGTH {
        return Err(AppError::InvalidCredentials);
    }

    let admin = match repository::find_admin_for_login(pool, &email).await? {
        Some(admin) if admin.password_hash.is_some() => admin,
        _ => {
            run_blocking(|password| crypto::verify_against_dummy(&password), password).await?;
            return Err(AppError::InvalidCredentials);
        }
    };
    let stored_hash = admin.password_hash.unwrap_or_default();

    // Refused before the password is checked: a locked account must not act as
    // a guessing oracle, and the counter must not grow while it is locked.
    if let Some(remaining) = admin.lock_remaining_seconds {
        return Err(AppError::AdminLoginLocked {
            retry_after_seconds: remaining.max(1) as u64,
        });
    }
    if admin.lock_expired {
        repository::reset_login_failures(pool, admin.id).await?;
    }

    let password_matches = run_blocking(
        move |password| crypto::verify_password(&password, &stored_hash),
        password,
    )
    .await?;
    if !password_matches {
        repository::record_failed_login(
            pool,
            admin.id,
            ADMIN_LOGIN_MAX_ATTEMPTS,
            ADMIN_LOGIN_LOCK_MINUTES,
        )
        .await?;
        return Err(AppError::InvalidCredentials);
    }

    // Only for a proven password: refusing earlier would reveal the suspension.
    if !admin.is_active {
        return Err(AppError::AccountSuspended);
    }

    repository::reset_login_failures(pool, admin.id).await?;

    let raw_refresh_token = Uuid::new_v4().to_string();
    let refresh_token_hash = crypto::hash_refresh_token(&raw_refresh_token);
    repository::insert_refresh_token(
        pool,
        admin.id,
        &refresh_token_hash,
        config.jwt_refresh_ttl_days as i32,
    )
    .await?;

    Ok(SessionOutcome {
        access_token: mint(
            admin.id,
            Role::Admin,
            &config.jwt_secret,
            config.jwt_access_ttl_seconds,
        )?,
        raw_refresh_token,
    })
}

/// Argon2 is CPU-bound; keeping it off the runtime workers.
async fn run_blocking<T, F>(work: F, password: &str) -> Result<T, AppError>
where
    T: Send + 'static,
    F: FnOnce(String) -> T + Send + 'static,
{
    let password = password.to_owned();
    tokio::task::spawn_blocking(move || work(password))
        .await
        .map_err(|error| {
            tracing::error!(%error, "password task failed");
            AppError::Internal
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::cache::build_auth_challenge_cache;

    const MAX_ATTEMPTS: u32 = 3;
    const EMAIL: &str = "ada@example.cm";
    const CODE: &str = "123456";

    fn verify_config() -> OtpVerifyConfig {
        OtpVerifyConfig::new(Arc::from(&[7u8; 32][..]), 900, 30, MAX_ATTEMPTS)
    }

    /// Only the wrong-code and expiry paths run: none of them reaches the pool.
    fn unused_pool() -> PgPool {
        PgPool::connect_lazy("postgres://localhost/unused").unwrap()
    }

    async fn issue(
        cache: &dyn AppCacheProvider<String, AuthChallenge>,
        window: &Arc<OtpAttemptWindow>,
        expires_at: Instant,
    ) {
        let pending = PendingOtp {
            code_hash: crypto::hash_otp_code(CODE),
            is_new: true,
            expires_at,
            attempt_window: Arc::clone(window),
            attempts_at_issue: window.attempts(),
        };
        cache
            .insert(otp_key(EMAIL), AuthChallenge::PendingOtp(pending))
            .await;
    }

    #[test]
    fn request_cooldown_doubles_per_spent_code_and_caps() {
        let secs = |attempts| request_cooldown(60, attempts, MAX_ATTEMPTS).as_secs();
        assert_eq!(secs(0), 60);
        assert_eq!(secs(2), 60);
        assert_eq!(secs(3), 120);
        assert_eq!(secs(6), 240);
        assert_eq!(secs(9), OTP_MAX_REQUEST_COOLDOWN.as_secs());
        assert_eq!(secs(u32::MAX), OTP_MAX_REQUEST_COOLDOWN.as_secs());
    }

    #[tokio::test]
    async fn the_right_code_fails_once_the_budget_is_spent() {
        let cache = build_auth_challenge_cache(Duration::from_secs(600));
        let window = Arc::new(OtpAttemptWindow::default());
        issue(
            cache.as_ref(),
            &window,
            Instant::now() + Duration::from_secs(600),
        )
        .await;
        let (pool, config) = (unused_pool(), verify_config());

        for _ in 0..MAX_ATTEMPTS {
            let wrong = verify_otp(&pool, cache.as_ref(), &config, EMAIL, "000000").await;
            assert!(matches!(wrong, Err(AppError::OtpInvalid)));
        }
        let right = verify_otp(&pool, cache.as_ref(), &config, EMAIL, CODE).await;
        assert!(matches!(right, Err(AppError::OtpInvalid)));
    }

    #[tokio::test]
    async fn a_new_code_does_not_reset_the_attempt_count() {
        let cache = build_auth_challenge_cache(Duration::from_secs(600));
        let window = Arc::new(OtpAttemptWindow::default());
        let (pool, config) = (unused_pool(), verify_config());
        let later = Instant::now() + Duration::from_secs(600);

        issue(cache.as_ref(), &window, later).await;
        for _ in 0..MAX_ATTEMPTS {
            let _ = verify_otp(&pool, cache.as_ref(), &config, EMAIL, "000000").await;
        }
        issue(cache.as_ref(), &window, later).await;
        let _ = verify_otp(&pool, cache.as_ref(), &config, EMAIL, "000000").await;

        assert_eq!(window.attempts(), MAX_ATTEMPTS + 1);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn concurrent_guesses_never_exceed_the_budget() {
        let cache = build_auth_challenge_cache(Duration::from_secs(600));
        let window = Arc::new(OtpAttemptWindow::default());
        issue(
            cache.as_ref(),
            &window,
            Instant::now() + Duration::from_secs(600),
        )
        .await;
        let pool = unused_pool();

        let mut guesses = tokio::task::JoinSet::new();
        for _ in 0..20 {
            let (cache, pool) = (Arc::clone(&cache), pool.clone());
            guesses.spawn(async move {
                let _ = verify_otp(&pool, cache.as_ref(), &verify_config(), EMAIL, "000000").await;
            });
        }
        guesses.join_all().await;

        let right = verify_otp(&pool, cache.as_ref(), &verify_config(), EMAIL, CODE).await;
        assert!(matches!(right, Err(AppError::OtpInvalid)));
    }

    #[tokio::test]
    async fn an_expired_code_is_refused_even_when_right() {
        let cache = build_auth_challenge_cache(Duration::from_secs(600));
        let window = Arc::new(OtpAttemptWindow::default());
        issue(cache.as_ref(), &window, Instant::now()).await;

        let result = verify_otp(
            &unused_pool(),
            cache.as_ref(),
            &verify_config(),
            EMAIL,
            CODE,
        )
        .await;
        assert!(matches!(result, Err(AppError::OtpInvalid)));
        assert!(cache.get(&otp_key(EMAIL)).await.is_none());
    }
}
