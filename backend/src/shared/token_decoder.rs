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
/// JWT library so MH-35 can implement it with no structural change here.
pub trait TokenDecoder: Send + Sync {
    /// Any decode failure (malformed, expired, bad signature) maps to `AppError::Unauthorized`.
    fn decode(&self, token: &str) -> Result<TokenClaims, AppError>;
}

/// Placeholder until MH-35 lands the JWT-backed implementation.
pub(crate) struct UnimplementedTokenDecoder;

impl TokenDecoder for UnimplementedTokenDecoder {
    fn decode(&self, _token: &str) -> Result<TokenClaims, AppError> {
        todo!("TokenDecoder implementation lands in MH-35 (JWT access token issuance & validation)")
    }
}
