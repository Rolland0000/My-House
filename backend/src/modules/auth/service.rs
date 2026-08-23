use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::infra::cache::AppCacheProvider;
use crate::shared::crypto;
use crate::shared::errors::AppError;
use crate::shared::rbac::Role;
use crate::shared::token_decoder::TokenClaims;
use crate::shared::types::RefreshTokenId;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::cache::build_refresh_replay_cache_provider;
    use std::sync::atomic::{AtomicUsize, Ordering};

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
        let cache = build_refresh_replay_cache_provider();

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
        let cache = build_refresh_replay_cache_provider(); // empty: no grace entry

        let result = perform_refresh("raw-token", &repo, cache.as_ref(), SECRET, 900, 30).await;

        assert!(matches!(result, Err(AppError::RefreshTokenInvalid)));
        assert_eq!(repo.revoke_all_calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn reuse_within_grace_window_returns_cached_replacement_idempotently() {
        let lookup = row(true, false);
        let row_id = RefreshTokenId::new(lookup.id);
        let repo = StubRepository::single(Some(lookup));
        let cache = build_refresh_replay_cache_provider();
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
        let cache = build_refresh_replay_cache_provider();

        let result = perform_refresh("raw-token", &repo, cache.as_ref(), SECRET, 900, 30).await;

        assert!(matches!(result, Err(AppError::RefreshTokenInvalid)));
        assert_eq!(repo.revoke_all_calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn unknown_token_hash_is_rejected() {
        let repo = StubRepository::single(None);
        let cache = build_refresh_replay_cache_provider();

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
        let cache = build_refresh_replay_cache_provider();
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
}
