//! SHA-256 hashing for secrets that are only ever stored and compared by
//! their digest — the raw value itself is never persisted.

use sha2::{Digest, Sha256};

fn sha256_hex(input: &str) -> String {
    let digest = Sha256::digest(input.as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Hashes a raw opaque refresh-token string to its hex-encoded SHA-256
/// digest — the only form ever persisted in `refresh_tokens.token_hash`.
pub fn hash_refresh_token(raw_token: &str) -> String {
    sha256_hex(raw_token)
}

/// Hashes a raw OTP code the same way — the only form ever persisted or
/// cached; the raw code itself is held in memory just long enough to send
/// the delivery email.
pub fn hash_otp_code(raw_code: &str) -> String {
    sha256_hex(raw_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_deterministically_to_a_64_char_hex_digest() {
        let hash = hash_otp_code("123456");
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(hash, hash_otp_code("123456"));
    }
}
