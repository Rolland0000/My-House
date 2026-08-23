//! Project-wide home for cryptographic operations: JWT access-token
//! signing/verification ([`jwt`]), SHA-256 hashing for secrets that are
//! only ever compared by their digest — refresh tokens and OTP codes
//! ([`hashing`]) — and CSPRNG-backed OTP code generation ([`otp`]). Future
//! crypto needs extend this module rather than scattering calls elsewhere.
//!
//! HS256 (HMAC-SHA256) is used for JWTs, not an asymmetric algorithm:
//! MyHouse issues and verifies tokens from the same monolith process, so a
//! shared secret (`JWT_SECRET`, length-validated at `AppConfig::from_env`)
//! is the right trade-off — no keypair/PKI overhead for a benefit that only
//! matters once issuance and verification live in separate services.
//!
//! Access tokens are stateless by design: there is no DB-backed revocation
//! list, so a token remains cryptographically valid for its full TTL
//! regardless of later account changes. The `AuthUser` extractor bounds
//! this by rechecking `is_active` on every request (see
//! `shared::extractors::resolve_identity`), keeping a suspended account
//! locked out well within the token's own lifetime even though its
//! signature would still verify.

mod hashing;
mod jwt;
mod otp;

pub use hashing::{hash_otp_code, hash_refresh_token};
pub use jwt::{issue_access_token, verify_access_token, JwtTokenDecoder};
pub use otp::generate_otp_code;
