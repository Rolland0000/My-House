//! Argon2id password hashing for admin credentials. Unlike the SHA-256 digests
//! in [`super::hashing`], a password is low-entropy, so the hash is salted and
//! deliberately expensive. Both operations are CPU-bound: call them through
//! `spawn_blocking` from async code.

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;

use crate::shared::errors::AppError;

/// Hashes `password` with a fresh random salt; the result is a PHC string
/// (`$argon2id$v=19$m=…,t=…,p=…$salt$hash`) that embeds its own parameters.
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| {
            tracing::error!(%error, "password hashing failed");
            AppError::Internal
        })
}

/// `false` for a wrong password or an unparseable stored hash.
pub fn verify_password(password: &str, stored_hash: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(stored_hash) else {
        tracing::error!("stored password hash is not a valid PHC string");
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

/// Verifies against a throwaway hash so an unknown account costs the same time
/// as a known one, keeping response timing from revealing which emails exist.
pub fn verify_against_dummy(password: &str) {
    let _ = verify_password(password, DUMMY_PASSWORD_HASH);
}

// Argon2id hash (default parameters) of a random string nobody knows.
const DUMMY_PASSWORD_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHRzb21lc2FsdA$yXPMNSJ0QEcpFB4yqkvWzR5U5ZX1I5PSjHQf0yqQ0nA";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_a_salted_argon2id_phc_string() {
        let first = hash_password("correct horse battery").unwrap();
        let second = hash_password("correct horse battery").unwrap();
        assert!(first.starts_with("$argon2id$"));
        assert_ne!(first, second);
    }

    #[test]
    fn verifies_only_the_original_password() {
        let hash = hash_password("correct horse battery").unwrap();
        assert!(verify_password("correct horse battery", &hash));
        assert!(!verify_password("correct horse batterz", &hash));
    }

    #[test]
    fn malformed_stored_hash_never_verifies() {
        assert!(!verify_password("anything", "not-a-phc-string"));
    }

    #[test]
    fn dummy_hash_is_parseable() {
        assert!(PasswordHash::new(DUMMY_PASSWORD_HASH).is_ok());
    }
}
