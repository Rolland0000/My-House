use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use sqlx::PgPool;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::infra::cache::AppCacheProvider;
use crate::modules::users::repository::find_is_active;
use crate::shared::errors::AppError;
use crate::shared::rbac::{require_role, Role};
use crate::shared::token_decoder::TokenDecoder;

/// Authenticated caller identity, extracted from the access token on every
/// protected route. `is_active` is rechecked on each request (via a
/// short-TTL cache, not proactively invalidated on suspend — MH-32,
/// ARCHITECTURE.md §8.1) so a suspension takes effect within the cache TTL
/// even against a token still inside its own validity window.
#[derive(Debug, Clone, Copy)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub role: Role,
}

impl AuthUser {
    /// Convenience for handlers that only need a role check, e.g.
    /// `user.require_role(&[Role::Owner, Role::Admin])?`.
    pub fn require_role(&self, allowed: &[Role]) -> Result<(), AppError> {
        require_role(self.role, allowed)
    }
}

/// DB lookup seam for the cache-miss path — lets `resolve_identity` be unit
/// tested without a `PgPool`.
#[async_trait]
trait IsActiveLookup: Send + Sync {
    async fn is_active(&self, user_id: Uuid) -> Result<Option<bool>, AppError>;
}

struct PgIsActiveLookup<'a>(&'a sqlx::PgPool);

#[async_trait]
impl IsActiveLookup for PgIsActiveLookup<'_> {
    async fn is_active(&self, user_id: Uuid) -> Result<Option<bool>, AppError> {
        find_is_active(self.0, user_id).await
    }
}

/// The slice of `AppState` `AuthUser` actually needs — `config`, `mailer`,
/// `storage` stay unreachable from the auth path even though `AppState`
/// carries them. Built per request from cheap `Arc`/`PgPool` clones.
#[derive(Clone)]
struct AuthState {
    db: PgPool,
    cache: Arc<dyn AppCacheProvider<Uuid, bool>>,
    token_decoder: Arc<dyn TokenDecoder>,
}

impl FromRef<AppState> for AuthState {
    fn from_ref(state: &AppState) -> Self {
        Self {
            db: state.db().clone(),
            cache: state.cache().is_active_status(),
            token_decoder: Arc::clone(state.token_decoder()),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AuthState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = bearer_token(parts)?;
        let auth_state = AuthState::from_ref(state);

        resolve_identity(
            token,
            auth_state.token_decoder.as_ref(),
            auth_state.cache.as_ref(),
            &PgIsActiveLookup(&auth_state.db),
        )
        .await
    }
}

fn bearer_token(parts: &Parts) -> Result<&str, AppError> {
    parts
        .headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|token| !token.is_empty())
        .ok_or(AppError::Unauthorized)
}

/// Core extractor logic, decoupled from Axum/`AppState` so it can be unit
/// tested with stub decoder/cache/lookup implementations.
async fn resolve_identity(
    token: &str,
    decoder: &dyn TokenDecoder,
    cache: &dyn AppCacheProvider<Uuid, bool>,
    lookup: &dyn IsActiveLookup,
) -> Result<AuthUser, AppError> {
    let claims = decoder.decode(token)?;

    let is_active = match cache.get(&claims.user_id).await {
        Some(cached) => cached,
        None => {
            let is_active = lookup
                .is_active(claims.user_id)
                .await?
                .ok_or(AppError::Unauthorized)?;
            cache.insert(claims.user_id, is_active).await;
            is_active
        }
    };

    if !is_active {
        return Err(AppError::AccountSuspended);
    }

    Ok(AuthUser {
        user_id: claims.user_id,
        role: claims.role,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::cache::build_cache_provider;
    use crate::shared::token_decoder::TokenClaims;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct StubDecoder(Result<TokenClaims, ()>);

    impl TokenDecoder for StubDecoder {
        fn decode(&self, _token: &str) -> Result<TokenClaims, AppError> {
            self.0.map_err(|_| AppError::Unauthorized)
        }
    }

    struct StubLookup {
        is_active: Option<bool>,
        calls: AtomicUsize,
    }

    #[async_trait]
    impl IsActiveLookup for StubLookup {
        async fn is_active(&self, _user_id: Uuid) -> Result<Option<bool>, AppError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(self.is_active)
        }
    }

    fn claims(role: Role) -> TokenClaims {
        TokenClaims {
            user_id: Uuid::new_v4(),
            role,
        }
    }

    #[tokio::test]
    async fn missing_token_is_unauthorized() {
        let request = axum::http::Request::builder().uri("/").body(()).unwrap();
        let (parts, _) = request.into_parts();
        assert!(matches!(bearer_token(&parts), Err(AppError::Unauthorized)));
    }

    #[tokio::test]
    async fn malformed_token_is_unauthorized() {
        let decoder = StubDecoder(Err(()));
        let cache = build_cache_provider();
        let lookup = StubLookup {
            is_active: Some(true),
            calls: AtomicUsize::new(0),
        };

        let result = resolve_identity("bad-token", &decoder, cache.as_ref(), &lookup).await;
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }

    #[tokio::test]
    async fn active_user_resolves_identity() {
        let decoded = claims(Role::Seeker);
        let decoder = StubDecoder(Ok(decoded));
        let cache = build_cache_provider();
        let lookup = StubLookup {
            is_active: Some(true),
            calls: AtomicUsize::new(0),
        };

        let user = resolve_identity("token", &decoder, cache.as_ref(), &lookup)
            .await
            .unwrap();
        assert_eq!(user.user_id, decoded.user_id);
        assert_eq!(user.role, Role::Seeker);
    }

    #[tokio::test]
    async fn inactive_user_is_rejected_even_with_valid_token() {
        let decoder = StubDecoder(Ok(claims(Role::Owner)));
        let cache = build_cache_provider();
        let lookup = StubLookup {
            is_active: Some(false),
            calls: AtomicUsize::new(0),
        };

        let result = resolve_identity("token", &decoder, cache.as_ref(), &lookup).await;
        assert!(matches!(result, Err(AppError::AccountSuspended)));
    }

    #[tokio::test]
    async fn cache_hit_skips_the_db_lookup() {
        let decoded = claims(Role::Admin);
        let decoder = StubDecoder(Ok(decoded));
        let cache = build_cache_provider();
        cache.insert(decoded.user_id, true).await;
        let lookup = StubLookup {
            is_active: Some(true),
            calls: AtomicUsize::new(0),
        };

        let result = resolve_identity("token", &decoder, cache.as_ref(), &lookup).await;
        assert!(result.is_ok());
        assert_eq!(lookup.calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn cache_miss_falls_through_to_db_and_repopulates_cache() {
        let decoded = claims(Role::Seeker);
        let decoder = StubDecoder(Ok(decoded));
        let cache = build_cache_provider();
        let lookup = StubLookup {
            is_active: Some(true),
            calls: AtomicUsize::new(0),
        };

        resolve_identity("token", &decoder, cache.as_ref(), &lookup)
            .await
            .unwrap();
        assert_eq!(lookup.calls.load(Ordering::SeqCst), 1);
        assert_eq!(cache.get(&decoded.user_id).await, Some(true));
    }
}
