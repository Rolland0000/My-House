use uuid::Uuid;

use crate::shared::errors::AppError;
use crate::shared::rbac::Role;

/// Identity carried by a valid access token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenClaims {
    pub user_id: Uuid,
    pub role: Role,
}

/// Decoding seam consumed by `AuthUser` — kept independent of any concrete
/// JWT library. `shared::crypto::JwtTokenDecoder` is the MH-35 implementation.
pub trait TokenDecoder: Send + Sync {
    /// A decode failure maps to `AppError::TokenExpired` for an expired
    /// token, `AppError::Unauthorized` for anything else (malformed, bad
    /// signature, disallowed algorithm).
    fn decode(&self, token: &str) -> Result<TokenClaims, AppError>;
}
