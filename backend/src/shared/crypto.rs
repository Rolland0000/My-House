//! Project-wide home for cryptographic operations. Scoped to JWT access-token
//! signing and verification for MH-35; a future ticket (refresh-token
//! hashing) extends this module rather than scattering crypto calls
//! elsewhere.
//!
//! HS256 (HMAC-SHA256) is used, not an asymmetric algorithm: MyHouse issues
//! and verifies tokens from the same monolith process, so a shared secret
//! (`JWT_SECRET`, length-validated at `AppConfig::from_env`) is the right
//! trade-off — no keypair/PKI overhead for a benefit that only matters once
//! issuance and verification live in separate services.
//!
//! Access tokens are stateless by design: there is no DB-backed revocation
//! list, so a token remains cryptographically valid for its full TTL
//! regardless of later account changes. This is an accepted MVP trade-off,
//! bounded by two things: a short `JWT_ACCESS_TTL_SECONDS` TTL, and MH-32's
//! `AuthUser` extractor rechecking `is_active` on every request (see
//! `shared::extractors::resolve_identity`) — a suspended account is locked
//! out well within the token's own lifetime, even though its signature
//! would still verify.

use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::AppError;
use crate::shared::rbac::Role;
use crate::shared::token_decoder::{TokenClaims, TokenDecoder};

/// JWT wire payload — internal to this module. Callers only ever see
/// `TokenClaims` (no `iat`/`exp`); nothing outside `crypto.rs` needs the raw
/// timestamps.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    role: Role,
    iat: u64,
    exp: u64,
}

fn now_unix() -> Result<u64, AppError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .map_err(|_| AppError::Internal)
}

/// Issues a signed HS256 access token for `claims`, expiring exactly
/// `ttl_seconds` after issuance.
pub fn issue_access_token(
    claims: TokenClaims,
    secret: &[u8],
    ttl_seconds: u64,
) -> Result<String, AppError> {
    let iat = now_unix()?;
    let exp = iat.checked_add(ttl_seconds).ok_or(AppError::Internal)?;

    let wire_claims = Claims {
        sub: claims.user_id,
        role: claims.role,
        iat,
        exp,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &wire_claims,
        &EncodingKey::from_secret(secret),
    )
    .map_err(|_| AppError::Internal)
}

/// Verifies `token`'s HS256 signature and expiration against `secret`. The
/// accepted algorithm is pinned to HS256 — a token asserting any other
/// algorithm is rejected before its signature is even checked.
pub fn verify_access_token(token: &str, secret: &[u8]) -> Result<TokenClaims, AppError> {
    let validation = Validation::new(Algorithm::HS256);

    let data = decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)
        .map_err(map_jwt_error)?;

    Ok(TokenClaims {
        user_id: data.claims.sub,
        role: data.claims.role,
    })
}

/// Only `ExpiredSignature` gets its own `AppError` variant (per MH-35's
/// acceptance criteria); every other failure — bad signature, disallowed
/// algorithm, malformed token — is a generic `Unauthorized`. The wildcard
/// also keeps this forward-compatible with `ErrorKind` being
/// `#[non_exhaustive]`.
fn map_jwt_error(error: jsonwebtoken::errors::Error) -> AppError {
    use jsonwebtoken::errors::ErrorKind;

    match error.kind() {
        ErrorKind::ExpiredSignature => AppError::TokenExpired,
        _ => AppError::Unauthorized,
    }
}

/// `TokenDecoder` backed by [`verify_access_token`]. Built once from
/// `config.jwt_secret` in `AppState::new` and shared behind an `Arc`.
pub struct JwtTokenDecoder {
    secret: Vec<u8>,
}

impl JwtTokenDecoder {
    pub fn new(secret: impl Into<Vec<u8>>) -> Self {
        Self {
            secret: secret.into(),
        }
    }
}

impl TokenDecoder for JwtTokenDecoder {
    fn decode(&self, token: &str) -> Result<TokenClaims, AppError> {
        verify_access_token(token, &self.secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &[u8] = b"unit-test-secret-at-least-32-bytes-long!!";

    fn claims(role: Role) -> TokenClaims {
        TokenClaims {
            user_id: Uuid::new_v4(),
            role,
        }
    }

    /// Builds a token directly from hand-picked `iat`/`exp`, bypassing
    /// `issue_access_token`'s "now" clock — lets tests construct an
    /// already-expired token without sleeping.
    fn encode_with_exp(claims: TokenClaims, exp: u64, alg: Algorithm) -> String {
        let wire = Claims {
            sub: claims.user_id,
            role: claims.role,
            iat: exp.saturating_sub(900),
            exp,
        };
        encode(&Header::new(alg), &wire, &EncodingKey::from_secret(SECRET)).unwrap()
    }

    #[test]
    fn valid_token_round_trips_through_issue_and_verify() {
        let original = claims(Role::Seeker);
        let token = issue_access_token(original, SECRET, 900).unwrap();

        let decoded = verify_access_token(&token, SECRET).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn expired_token_is_rejected_with_token_expired() {
        let now = now_unix().unwrap();
        let token = encode_with_exp(
            claims(Role::Owner),
            now.saturating_sub(3600),
            Algorithm::HS256,
        );

        let result = verify_access_token(&token, SECRET);
        assert!(matches!(result, Err(AppError::TokenExpired)));
    }

    #[test]
    fn tampered_signature_is_rejected_with_unauthorized() {
        let token = issue_access_token(claims(Role::Admin), SECRET, 900).unwrap();
        let split_at = token.rfind('.').expect("JWT has a signature segment");
        let tampered = format!(
            "{}.{}",
            &token[..split_at],
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
        );

        let result = verify_access_token(&tampered, SECRET);
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }

    /// HS384 accepts the same raw secret bytes as HS256 (both HMAC-family),
    /// so this needs no second key. `jsonwebtoken::Algorithm` has no `none`
    /// variant at all, so a literal `alg: none` token can't even be built
    /// this way — the `Validation::algorithms` pin this test exercises is
    /// exactly what would also reject it, one step later than header parsing.
    #[test]
    fn wrong_algorithm_token_is_rejected_with_unauthorized() {
        let now = now_unix().unwrap();
        let token = encode_with_exp(claims(Role::Seeker), now + 900, Algorithm::HS384);

        let result = verify_access_token(&token, SECRET);
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }

    #[test]
    fn malformed_token_string_is_rejected() {
        let result = verify_access_token("not-a-jwt", SECRET);
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }

    #[test]
    fn issued_token_expires_exactly_ttl_seconds_after_issuance() {
        let token = issue_access_token(claims(Role::Owner), SECRET, 900).unwrap();

        let validation = Validation::new(Algorithm::HS256);
        let data =
            decode::<Claims>(&token, &DecodingKey::from_secret(SECRET), &validation).unwrap();
        assert_eq!(data.claims.exp - data.claims.iat, 900);
    }

    #[test]
    fn jwt_token_decoder_adapter_delegates_correctly() {
        let decoder: &dyn TokenDecoder = &JwtTokenDecoder::new(SECRET);
        let original = claims(Role::Admin);
        let token = issue_access_token(original, SECRET, 900).unwrap();

        assert_eq!(decoder.decode(&token).unwrap(), original);

        let now = now_unix().unwrap();
        let expired = encode_with_exp(
            claims(Role::Seeker),
            now.saturating_sub(3600),
            Algorithm::HS256,
        );
        assert!(matches!(
            decoder.decode(&expired),
            Err(AppError::TokenExpired)
        ));
    }
}
