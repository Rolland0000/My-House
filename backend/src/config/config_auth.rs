//! Authentication settings: JWT, OTP and the refresh-token cookie.

use super::config_error::ConfigError;
use super::env_reader::{require, require_u32, require_u64};

/// Shortest `JWT_SECRET` accepted, in bytes (256 bits).
const JWT_SECRET_MIN_BYTES: usize = 32;

#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// Raw secret used to sign and verify JWTs (≥ 32 bytes enforced).
    pub jwt_secret: String,
    /// Access-token lifetime in seconds (e.g. 900 = 15 min).
    pub jwt_access_ttl_seconds: u64,
    /// Refresh-token lifetime in days (sliding TTL, e.g. 30).
    pub jwt_refresh_ttl_days: u64,
    /// Time-to-live for an OTP code in seconds (e.g. 600 = 10 min).
    pub otp_ttl_seconds: u64,
    /// Maximum verification attempts per OTP before it is invalidated.
    pub otp_max_attempts: u32,
    /// Minimum gap between two OTP requests for the same email (in seconds).
    pub otp_rate_limit_seconds: u64,
    /// Domain attribute of the `refresh_token` cookie (e.g. `localhost`).
    pub cookie_domain: String,
}

impl AuthConfig {
    pub(super) fn from_env() -> Result<Self, ConfigError> {
        let jwt_secret = require("JWT_SECRET")?;
        if jwt_secret.len() < JWT_SECRET_MIN_BYTES {
            return Err(ConfigError::Invalid {
                key: "JWT_SECRET".to_string(),
                reason: format!(
                    "must be at least {JWT_SECRET_MIN_BYTES} bytes (256 bits), got {} byte(s)",
                    jwt_secret.len()
                ),
            });
        }
        Ok(Self {
            jwt_secret,
            jwt_access_ttl_seconds: require_u64("JWT_ACCESS_TTL_SECONDS")?,
            jwt_refresh_ttl_days: require_u64("JWT_REFRESH_TTL_DAYS")?,
            otp_ttl_seconds: require_u64("OTP_TTL_SECONDS")?,
            otp_max_attempts: require_u32("OTP_MAX_ATTEMPTS")?,
            otp_rate_limit_seconds: require_u64("OTP_RATE_LIMIT_SECONDS")?,
            cookie_domain: require("COOKIE_DOMAIN")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use crate::config::test_support::{set_valid_env, ENV_LOCK};

    #[test]
    fn loads_every_auth_setting() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        let auth = AuthConfig::from_env().expect("should load without error");
        assert_eq!(auth.jwt_access_ttl_seconds, 900);
        assert_eq!(auth.jwt_refresh_ttl_days, 30);
        assert_eq!(auth.otp_ttl_seconds, 600);
        assert_eq!(auth.otp_max_attempts, 3);
        assert_eq!(auth.otp_rate_limit_seconds, 60);
        assert_eq!(auth.cookie_domain, "localhost");
    }

    #[test]
    fn rejects_short_jwt_secret() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("JWT_SECRET", "tooshort");
        let err = AuthConfig::from_env().expect_err("should fail on short secret");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "JWT_SECRET"));
    }

    #[test]
    fn accepts_a_secret_of_exactly_the_minimum_length() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("JWT_SECRET", "a".repeat(JWT_SECRET_MIN_BYTES));
        assert!(AuthConfig::from_env().is_ok());
    }

    #[test]
    fn rejects_missing_jwt_secret() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::remove_var("JWT_SECRET");
        let err = AuthConfig::from_env().expect_err("should fail on missing secret");
        assert!(matches!(err, ConfigError::Missing(key) if key == "JWT_SECRET"));
    }

    #[test]
    fn rejects_non_numeric_otp_max_attempts() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("OTP_MAX_ATTEMPTS", "three");
        let err = AuthConfig::from_env().expect_err("should fail on a non-numeric value");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "OTP_MAX_ATTEMPTS"));
    }

    #[test]
    fn rejects_missing_cookie_domain() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::remove_var("COOKIE_DOMAIN");
        let err = AuthConfig::from_env().expect_err("should fail on missing cookie domain");
        assert!(matches!(err, ConfigError::Missing(key) if key == "COOKIE_DOMAIN"));
    }
}
