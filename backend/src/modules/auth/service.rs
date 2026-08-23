use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

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

/// Repository seam — lets `perform_refresh` be unit tested without a `PgPool`.
#[async_trait]
trait RefreshTokenRepository: Send + Sync {
    async fn find_by_hash(&self, token_hash: &str) -> Result<Option<RefreshTokenLookup>, AppError>;
    /// `None` means `old_id` was already revoked by a concurrent request.
    async fn rotate(
        &self,
        old_id: Uuid,
        user_id: Uuid,
        new_token_hash: &str,
        ttl_days: i32,
    ) -> Result<Option<Uuid>, AppError>;
    async fn revoke_all_for_user(&self, user_id: Uuid) -> Result<(), AppError>;
    async fn revoke(&self, token_hash: &str) -> Result<(), AppError>;
}

struct PgRefreshTokenRepository<'a>(&'a PgPool);

#[async_trait]
impl RefreshTokenRepository for PgRefreshTokenRepository<'_> {
    async fn find_by_hash(&self, token_hash: &str) -> Result<Option<RefreshTokenLookup>, AppError> {
        repository::find_by_hash(self.0, token_hash).await
    }

    async fn rotate(
        &self,
        old_id: Uuid,
        user_id: Uuid,
        new_token_hash: &str,
        ttl_days: i32,
    ) -> Result<Option<Uuid>, AppError> {
        repository::rotate(self.0, old_id, user_id, new_token_hash, ttl_days).await
    }

    async fn revoke_all_for_user(&self, user_id: Uuid) -> Result<(), AppError> {
        repository::revoke_all_for_user(self.0, user_id).await
    }

    async fn revoke(&self, token_hash: &str) -> Result<(), AppError> {
        repository::revoke(self.0, token_hash).await
    }
}

/// Entry point wired by the handler.
pub async fn refresh(
    pool: &PgPool,
    grace_cache: &dyn AppCacheProvider<RefreshTokenId, String>,
    jwt_secret: &[u8],
    access_ttl_seconds: u64,
    refresh_ttl_days: u64,
    raw_token: &str,
) -> Result<RefreshOutcome, AppError> {
    perform_refresh(
        raw_token,
        &PgRefreshTokenRepository(pool),
        grace_cache,
        jwt_secret,
        access_ttl_seconds,
        refresh_ttl_days,
    )
    .await
}

/// Core logic, decoupled from `PgPool` for unit testing.
async fn perform_refresh(
    raw_token: &str,
    repo: &dyn RefreshTokenRepository,
    grace_cache: &dyn AppCacheProvider<RefreshTokenId, String>,
    jwt_secret: &[u8],
    access_ttl_seconds: u64,
    refresh_ttl_days: u64,
) -> Result<RefreshOutcome, AppError> {
    let token_hash = crypto::hash_refresh_token(raw_token);
    let row = repo
        .find_by_hash(&token_hash)
        .await?
        .ok_or(AppError::RefreshTokenInvalid)?;

    if row.is_revoked {
        return handle_revoked(&row, repo, grace_cache, jwt_secret, access_ttl_seconds).await;
    }
    if row.is_expired {
        return Err(AppError::RefreshTokenInvalid);
    }

    let new_raw_token = Uuid::new_v4().to_string();
    let new_hash = crypto::hash_refresh_token(&new_raw_token);
    let row_id = RefreshTokenId::new(row.id);

    match repo
        .rotate(row.id, row.user_id, &new_hash, refresh_ttl_days as i32)
        .await?
    {
        Some(_new_id) => {
            grace_cache.insert(row_id, new_raw_token.clone()).await;
            Ok(RefreshOutcome {
                access_token: mint(row.user_id, row.role, jwt_secret, access_ttl_seconds)?,
                raw_refresh_token: new_raw_token,
            })
        }
        // Lost a concurrent rotation race — someone else already revoked
        // this row between our SELECT and our UPDATE. Re-check: it now sees
        // the winner's committed state and falls into the same reuse path
        // a sequential re-request would hit.
        None => {
            let row = repo
                .find_by_hash(&token_hash)
                .await?
                .ok_or(AppError::RefreshTokenInvalid)?;
            handle_revoked(&row, repo, grace_cache, jwt_secret, access_ttl_seconds).await
        }
    }
}

async fn handle_revoked(
    row: &RefreshTokenLookup,
    repo: &dyn RefreshTokenRepository,
    grace_cache: &dyn AppCacheProvider<RefreshTokenId, String>,
    jwt_secret: &[u8],
    access_ttl_seconds: u64,
) -> Result<RefreshOutcome, AppError> {
    let row_id = RefreshTokenId::new(row.id);
    if let Some(raw_replacement) = grace_cache.get(&row_id).await {
        return Ok(RefreshOutcome {
            access_token: mint(row.user_id, row.role, jwt_secret, access_ttl_seconds)?,
            raw_refresh_token: raw_replacement,
        });
    }
    tracing::warn!(user_id = %row.user_id, "refresh token reuse outside grace window — revoking token family");
    repo.revoke_all_for_user(row.user_id).await?;
    Err(AppError::RefreshTokenInvalid)
}

/// Entry point wired by the handler. Revokes only the single current token
/// — never the whole family, unlike [`handle_revoked`]'s theft response.
pub async fn logout(pool: &PgPool, raw_token: &str) -> Result<(), AppError> {
    perform_logout(raw_token, &PgRefreshTokenRepository(pool)).await
}

async fn perform_logout(
    raw_token: &str,
    repo: &dyn RefreshTokenRepository,
) -> Result<(), AppError> {
    let token_hash = crypto::hash_refresh_token(raw_token);
    repo.revoke(&token_hash).await
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

/// Repository seam — lets `perform_verify_otp` be unit tested without a
/// `PgPool`, mirroring [`RefreshTokenRepository`].
#[async_trait]
trait VerifyOtpRepository: Send + Sync {
    async fn find_user_by_email(&self, email: &str) -> Result<Option<(Uuid, Role)>, AppError>;
    /// `None` means `email` was taken by a concurrent request first.
    async fn create_seeker(&self, email: &str) -> Result<Option<Uuid>, AppError>;
    async fn insert_refresh_token(
        &self,
        user_id: Uuid,
        token_hash: &str,
        ttl_days: i32,
    ) -> Result<(), AppError>;
}

struct PgVerifyOtpRepository<'a>(&'a PgPool);

#[async_trait]
impl VerifyOtpRepository for PgVerifyOtpRepository<'_> {
    async fn find_user_by_email(&self, email: &str) -> Result<Option<(Uuid, Role)>, AppError> {
        repository::find_user_by_email(self.0, email).await
    }

    async fn create_seeker(&self, email: &str) -> Result<Option<Uuid>, AppError> {
        repository::create_seeker(self.0, email).await
    }

    async fn insert_refresh_token(
        &self,
        user_id: Uuid,
        token_hash: &str,
        ttl_days: i32,
    ) -> Result<(), AppError> {
        repository::insert_refresh_token(self.0, user_id, token_hash, ttl_days).await
    }
}

/// Entry point wired by the handler.
#[allow(clippy::too_many_arguments)]
pub async fn verify_otp(
    pool: &PgPool,
    otp_cache: &dyn AppCacheProvider<String, PendingOtp>,
    mailer: &Mailer,
    jwt_secret: &[u8],
    access_ttl_seconds: u64,
    refresh_ttl_days: u64,
    otp_max_attempts: u32,
    email: &str,
    code: &str,
) -> Result<VerifyOutcome, AppError> {
    perform_verify_otp(
        &PgVerifyOtpRepository(pool),
        otp_cache,
        mailer,
        jwt_secret,
        access_ttl_seconds,
        refresh_ttl_days,
        otp_max_attempts,
        email,
        code,
    )
    .await
}

/// Core logic, decoupled from `PgPool` for unit testing.
#[allow(clippy::too_many_arguments)]
async fn perform_verify_otp(
    repo: &dyn VerifyOtpRepository,
    otp_cache: &dyn AppCacheProvider<String, PendingOtp>,
    mailer: &Mailer,
    jwt_secret: &[u8],
    access_ttl_seconds: u64,
    refresh_ttl_days: u64,
    otp_max_attempts: u32,
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
        if attempts >= otp_max_attempts {
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
        let user_id = match repo.create_seeker(email).await? {
            Some(user_id) => {
                notifications::service::send_welcome_email(mailer, email, email).await;
                user_id
            }
            None => {
                repo.find_user_by_email(email)
                    .await?
                    .ok_or(AppError::OtpInvalid)?
                    .0
            }
        };
        (user_id, Role::Seeker, true)
    } else {
        let (user_id, role) = repo
            .find_user_by_email(email)
            .await?
            .ok_or(AppError::OtpInvalid)?;
        (user_id, role, false)
    };

    let access_token = mint(user_id, role, jwt_secret, access_ttl_seconds)?;

    let raw_refresh_token = Uuid::new_v4().to_string();
    let refresh_token_hash = crypto::hash_refresh_token(&raw_refresh_token);
    repo.insert_refresh_token(user_id, &refresh_token_hash, refresh_ttl_days as i32)
        .await?;

    Ok(VerifyOutcome {
        access_token,
        raw_refresh_token,
        is_new_user,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, AppEnv, StorageProvider};
    use crate::infra::cache::{build_otp_cache, build_refresh_replay_cache};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    /// `lookups` is consulted in order across successive `find_by_hash`
    /// calls (the last entry repeats) — lets a test simulate the state
    /// change a concurrent rotation causes between our two lookups.
    struct StubRepository {
        lookups: Vec<Option<RefreshTokenLookup>>,
        rotate_result: Option<Uuid>,
        find_by_hash_calls: AtomicUsize,
        rotate_calls: AtomicUsize,
        revoke_all_calls: AtomicUsize,
        revoke_calls: AtomicUsize,
    }

    impl StubRepository {
        fn single(lookup: Option<RefreshTokenLookup>) -> Self {
            Self {
                lookups: vec![lookup],
                rotate_result: Some(Uuid::new_v4()),
                find_by_hash_calls: AtomicUsize::new(0),
                rotate_calls: AtomicUsize::new(0),
                revoke_all_calls: AtomicUsize::new(0),
                revoke_calls: AtomicUsize::new(0),
            }
        }
    }

    #[async_trait]
    impl RefreshTokenRepository for StubRepository {
        async fn find_by_hash(
            &self,
            _token_hash: &str,
        ) -> Result<Option<RefreshTokenLookup>, AppError> {
            let call = self.find_by_hash_calls.fetch_add(1, Ordering::SeqCst);
            let index = call.min(self.lookups.len() - 1);
            Ok(self.lookups[index].clone())
        }

        async fn rotate(
            &self,
            _old_id: Uuid,
            _user_id: Uuid,
            _new_token_hash: &str,
            _ttl_days: i32,
        ) -> Result<Option<Uuid>, AppError> {
            self.rotate_calls.fetch_add(1, Ordering::SeqCst);
            Ok(self.rotate_result)
        }

        async fn revoke_all_for_user(&self, _user_id: Uuid) -> Result<(), AppError> {
            self.revoke_all_calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        async fn revoke(&self, _token_hash: &str) -> Result<(), AppError> {
            self.revoke_calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    fn row(is_revoked: bool, is_expired: bool) -> RefreshTokenLookup {
        RefreshTokenLookup {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            role: Role::Seeker,
            is_revoked,
            is_expired,
        }
    }

    const SECRET: &[u8] = b"unit-test-secret-at-least-32-bytes-long!!";

    #[tokio::test]
    async fn successful_rotation_returns_access_token_and_caches_grace_replay() {
        let lookup = row(false, false);
        let row_id = RefreshTokenId::new(lookup.id);
        let repo = StubRepository::single(Some(lookup));
        let cache = build_refresh_replay_cache();

        let outcome = perform_refresh("raw-token", &repo, cache.as_ref(), SECRET, 900, 30)
            .await
            .unwrap();

        assert!(crypto::verify_access_token(&outcome.access_token, SECRET).is_ok());
        assert_eq!(repo.rotate_calls.load(Ordering::SeqCst), 1);
        assert_eq!(repo.revoke_all_calls.load(Ordering::SeqCst), 0);
        assert_eq!(
            cache.get(&row_id).await,
            Some(outcome.raw_refresh_token.clone())
        );
    }

    #[tokio::test]
    async fn reuse_after_grace_window_revokes_family_and_returns_unauthorized() {
        let repo = StubRepository::single(Some(row(true, false)));
        let cache = build_refresh_replay_cache(); // empty: no grace entry

        let result = perform_refresh("raw-token", &repo, cache.as_ref(), SECRET, 900, 30).await;

        assert!(matches!(result, Err(AppError::RefreshTokenInvalid)));
        assert_eq!(repo.revoke_all_calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn reuse_within_grace_window_returns_cached_replacement_idempotently() {
        let lookup = row(true, false);
        let row_id = RefreshTokenId::new(lookup.id);
        let repo = StubRepository::single(Some(lookup));
        let cache = build_refresh_replay_cache();
        cache.insert(row_id, "cached-raw-token".to_string()).await;

        let outcome = perform_refresh("raw-token", &repo, cache.as_ref(), SECRET, 900, 30)
            .await
            .unwrap();

        assert_eq!(outcome.raw_refresh_token, "cached-raw-token");
        assert_eq!(repo.rotate_calls.load(Ordering::SeqCst), 0);
        assert_eq!(repo.revoke_all_calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn expired_token_is_rejected_without_family_revocation() {
        let repo = StubRepository::single(Some(row(false, true)));
        let cache = build_refresh_replay_cache();

        let result = perform_refresh("raw-token", &repo, cache.as_ref(), SECRET, 900, 30).await;

        assert!(matches!(result, Err(AppError::RefreshTokenInvalid)));
        assert_eq!(repo.revoke_all_calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn unknown_token_hash_is_rejected() {
        let repo = StubRepository::single(None);
        let cache = build_refresh_replay_cache();

        let result = perform_refresh("raw-token", &repo, cache.as_ref(), SECRET, 900, 30).await;

        assert!(matches!(result, Err(AppError::RefreshTokenInvalid)));
        assert_eq!(repo.rotate_calls.load(Ordering::SeqCst), 0);
        assert_eq!(repo.revoke_all_calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn losing_the_rotate_race_falls_back_to_cached_replacement() {
        let lookup = row(false, false);
        let row_id = RefreshTokenId::new(lookup.id);
        let revoked_lookup = RefreshTokenLookup {
            is_revoked: true,
            ..lookup.clone()
        };
        let repo = StubRepository {
            lookups: vec![Some(lookup), Some(revoked_lookup)],
            rotate_result: None, // simulates a concurrent winner already revoking it
            find_by_hash_calls: AtomicUsize::new(0),
            rotate_calls: AtomicUsize::new(0),
            revoke_all_calls: AtomicUsize::new(0),
            revoke_calls: AtomicUsize::new(0),
        };
        let cache = build_refresh_replay_cache();
        cache.insert(row_id, "winner-raw-token".to_string()).await;

        let outcome = perform_refresh("raw-token", &repo, cache.as_ref(), SECRET, 900, 30)
            .await
            .unwrap();

        assert_eq!(outcome.raw_refresh_token, "winner-raw-token");
        assert_eq!(repo.revoke_all_calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn logout_revokes_only_the_single_current_token() {
        let repo = StubRepository::single(None);

        perform_logout("raw-token", &repo).await.unwrap();

        assert_eq!(repo.revoke_calls.load(Ordering::SeqCst), 1);
        assert_eq!(repo.revoke_all_calls.load(Ordering::SeqCst), 0);
    }

    fn test_mailer() -> Mailer {
        let config = AppConfig {
            app_port: 3000,
            app_env: AppEnv::Development,
            database_url: "postgresql://x:x@localhost/x".to_string(),
            jwt_secret: "unit-test-secret-at-least-32-bytes-long!!".to_string(),
            jwt_access_ttl_seconds: 900,
            jwt_refresh_ttl_days: 30,
            otp_ttl_seconds: 600,
            otp_max_attempts: 3,
            otp_rate_limit_seconds: 60,
            storage_provider: StorageProvider::Local,
            local_storage_path: "/tmp".to_string(),
            public_media_base_url: "http://localhost/media".to_string(),
            cookie_domain: "localhost".to_string(),
            allowed_origins: vec!["http://localhost".to_string()],
            smtp_host: "localhost".to_string(),
            smtp_port: 1025,
            smtp_from: "noreply@myhouse.app".to_string(),
            admin_notification_email: "admin@myhouse.app".to_string(),
            admin_bootstrap_on_startup: false,
            admin_bootstrap_email: None,
        };
        Mailer::new(&config).expect("test mailer config should build")
    }

    async fn otp_cache_with_entry(
        email: &str,
        code_hash: String,
        is_new: bool,
        attempts: u32,
    ) -> Arc<dyn AppCacheProvider<String, PendingOtp>> {
        let cache = build_otp_cache(Duration::from_secs(60));
        cache
            .insert(
                email.to_string(),
                PendingOtp {
                    code_hash,
                    is_new,
                    attempts,
                },
            )
            .await;
        cache
    }

    /// `existing_user` drives `find_user_by_email`; `create_seeker_result`
    /// drives `create_seeker` (`None` simulates a lost unique-violation race).
    struct StubVerifyRepository {
        existing_user: Option<(Uuid, Role)>,
        create_seeker_result: Option<Uuid>,
        find_calls: AtomicUsize,
        create_calls: AtomicUsize,
        insert_refresh_calls: AtomicUsize,
    }

    impl StubVerifyRepository {
        fn known(user_id: Uuid, role: Role) -> Self {
            Self {
                existing_user: Some((user_id, role)),
                create_seeker_result: Some(Uuid::new_v4()),
                find_calls: AtomicUsize::new(0),
                create_calls: AtomicUsize::new(0),
                insert_refresh_calls: AtomicUsize::new(0),
            }
        }

        fn unknown() -> Self {
            Self {
                existing_user: None,
                create_seeker_result: Some(Uuid::new_v4()),
                find_calls: AtomicUsize::new(0),
                create_calls: AtomicUsize::new(0),
                insert_refresh_calls: AtomicUsize::new(0),
            }
        }

        /// `create_seeker` loses the unique-violation race; `winner_id` is
        /// what the concurrent `find_user_by_email` fallback returns.
        fn lost_create_race(winner_id: Uuid, role: Role) -> Self {
            Self {
                existing_user: Some((winner_id, role)),
                create_seeker_result: None,
                find_calls: AtomicUsize::new(0),
                create_calls: AtomicUsize::new(0),
                insert_refresh_calls: AtomicUsize::new(0),
            }
        }
    }

    #[async_trait]
    impl VerifyOtpRepository for StubVerifyRepository {
        async fn find_user_by_email(&self, _email: &str) -> Result<Option<(Uuid, Role)>, AppError> {
            self.find_calls.fetch_add(1, Ordering::SeqCst);
            Ok(self.existing_user)
        }

        async fn create_seeker(&self, _email: &str) -> Result<Option<Uuid>, AppError> {
            self.create_calls.fetch_add(1, Ordering::SeqCst);
            Ok(self.create_seeker_result)
        }

        async fn insert_refresh_token(
            &self,
            _user_id: Uuid,
            _token_hash: &str,
            _ttl_days: i32,
        ) -> Result<(), AppError> {
            self.insert_refresh_calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn correct_code_new_email_creates_account_and_issues_session_with_is_new_user_true() {
        let cache =
            otp_cache_with_entry("new@example.com", crypto::hash_otp_code("123456"), true, 0).await;
        let repo = StubVerifyRepository::unknown();
        let mailer = test_mailer();

        let outcome = perform_verify_otp(
            &repo,
            cache.as_ref(),
            &mailer,
            SECRET,
            900,
            30,
            3,
            "new@example.com",
            "123456",
        )
        .await
        .unwrap();

        assert!(outcome.is_new_user);
        let decoded = crypto::verify_access_token(&outcome.access_token, SECRET).unwrap();
        assert_eq!(decoded.role, Role::Seeker);
        assert_eq!(repo.create_calls.load(Ordering::SeqCst), 1);
        assert_eq!(repo.find_calls.load(Ordering::SeqCst), 0);
        assert_eq!(repo.insert_refresh_calls.load(Ordering::SeqCst), 1);
        assert!(cache.get(&"new@example.com".to_string()).await.is_none());
    }

    #[tokio::test]
    async fn losing_the_create_seeker_race_falls_back_to_the_winners_account() {
        let cache =
            otp_cache_with_entry("race@example.com", crypto::hash_otp_code("123456"), true, 0)
                .await;
        let winner_id = Uuid::new_v4();
        let repo = StubVerifyRepository::lost_create_race(winner_id, Role::Seeker);
        let mailer = test_mailer();

        let outcome = perform_verify_otp(
            &repo,
            cache.as_ref(),
            &mailer,
            SECRET,
            900,
            30,
            3,
            "race@example.com",
            "123456",
        )
        .await
        .unwrap();

        assert!(outcome.is_new_user);
        let decoded = crypto::verify_access_token(&outcome.access_token, SECRET).unwrap();
        assert_eq!(decoded.user_id, winner_id);
        assert_eq!(repo.create_calls.load(Ordering::SeqCst), 1);
        assert_eq!(repo.find_calls.load(Ordering::SeqCst), 1);
        assert_eq!(repo.insert_refresh_calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn correct_code_known_email_mutates_nothing_and_reports_is_new_user_false() {
        let cache = otp_cache_with_entry(
            "known@example.com",
            crypto::hash_otp_code("654321"),
            false,
            0,
        )
        .await;
        let user_id = Uuid::new_v4();
        let repo = StubVerifyRepository::known(user_id, Role::Owner);
        let mailer = test_mailer();

        let outcome = perform_verify_otp(
            &repo,
            cache.as_ref(),
            &mailer,
            SECRET,
            900,
            30,
            3,
            "known@example.com",
            "654321",
        )
        .await
        .unwrap();

        assert!(!outcome.is_new_user);
        let decoded = crypto::verify_access_token(&outcome.access_token, SECRET).unwrap();
        assert_eq!(decoded.user_id, user_id);
        assert_eq!(decoded.role, Role::Owner);
        assert_eq!(repo.create_calls.load(Ordering::SeqCst), 0);
        assert_eq!(repo.find_calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn wrong_code_under_max_attempts_increments_counter_and_keeps_entry_valid() {
        let cache = otp_cache_with_entry(
            "retry@example.com",
            crypto::hash_otp_code("123456"),
            false,
            0,
        )
        .await;
        let repo = StubVerifyRepository::unknown();
        let mailer = test_mailer();

        let result = perform_verify_otp(
            &repo,
            cache.as_ref(),
            &mailer,
            SECRET,
            900,
            30,
            3,
            "retry@example.com",
            "000000",
        )
        .await;

        assert!(matches!(result, Err(AppError::OtpInvalid)));
        let entry = cache
            .get(&"retry@example.com".to_string())
            .await
            .expect("entry remains valid for further attempts");
        assert_eq!(entry.attempts, 1);
    }

    #[tokio::test]
    async fn third_mismatch_deletes_entry_so_a_later_correct_code_also_fails() {
        let cache = otp_cache_with_entry(
            "exhausted@example.com",
            crypto::hash_otp_code("123456"),
            false,
            2,
        )
        .await;
        let repo = StubVerifyRepository::unknown();
        let mailer = test_mailer();

        let third_attempt = perform_verify_otp(
            &repo,
            cache.as_ref(),
            &mailer,
            SECRET,
            900,
            30,
            3,
            "exhausted@example.com",
            "000000",
        )
        .await;

        assert!(matches!(third_attempt, Err(AppError::OtpInvalid)));
        assert!(cache
            .get(&"exhausted@example.com".to_string())
            .await
            .is_none());

        let retry_with_correct_code = perform_verify_otp(
            &repo,
            cache.as_ref(),
            &mailer,
            SECRET,
            900,
            30,
            3,
            "exhausted@example.com",
            "123456",
        )
        .await;

        assert!(matches!(retry_with_correct_code, Err(AppError::OtpInvalid)));
    }

    #[tokio::test]
    async fn missing_otp_entry_is_rejected() {
        let cache = build_otp_cache(Duration::from_secs(60));
        let repo = StubVerifyRepository::unknown();
        let mailer = test_mailer();

        let result = perform_verify_otp(
            &repo,
            cache.as_ref(),
            &mailer,
            SECRET,
            900,
            30,
            3,
            "absent@example.com",
            "123456",
        )
        .await;

        assert!(matches!(result, Err(AppError::OtpInvalid)));
    }
}
