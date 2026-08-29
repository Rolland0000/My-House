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
            &auth_state.db,
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

/// Core extractor logic, decoupled from Axum/`AppState`.
async fn resolve_identity(
    token: &str,
    decoder: &dyn TokenDecoder,
    cache: &dyn AppCacheProvider<Uuid, bool>,
    db: &PgPool,
) -> Result<AuthUser, AppError> {
    let claims = decoder.decode(token)?;

    let is_active = match cache.get(&claims.user_id).await {
        Some(cached) => cached,
        None => {
            let is_active = find_is_active(db, claims.user_id)
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

    #[tokio::test]
    async fn missing_token_is_unauthorized() {
        let request = axum::http::Request::builder().uri("/").body(()).unwrap();
        let (parts, _) = request.into_parts();
        assert!(matches!(bearer_token(&parts), Err(AppError::Unauthorized)));
    }
}
