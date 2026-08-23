//! CSPRNG-backed one-time-code generation.

use rand::rngs::OsRng;
use rand::Rng;

/// Generates a 6-digit code using the OS's cryptographically secure RNG —
/// never a general-purpose PRNG. Zero-padded, so every value in
/// `000000..=999999` is equally likely and the result is always exactly 6
/// characters.
pub fn generate_otp_code() -> String {
    let code: u32 = OsRng.gen_range(0..1_000_000);
    format!("{code:06}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_code_is_always_six_digits() {
        for _ in 0..200 {
            let code = generate_otp_code();
            assert_eq!(code.len(), 6);
            assert!(code.chars().all(|c| c.is_ascii_digit()));
        }
    }
}
