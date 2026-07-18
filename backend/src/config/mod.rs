//! Application configuration loaded **exclusively from environment variables** (12-factor).
//!
//! Call [`AppConfig::from_env`] once at startup. It reads every required variable,
//! validates types and constraints, then returns a fully-populated `AppConfig` or
//! a descriptive [`ConfigError`] that causes the process to exit immediately.
//!
//! Variable catalogue — see `backend/.env.example` for the full list.

use std::{env, fmt};

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

/// Errors that can occur while loading the configuration.
#[derive(Debug)]
pub enum ConfigError {
    /// A required variable is absent from the environment.
    Missing(String),
    /// A variable is present but its value cannot be parsed.
    Invalid { key: String, reason: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Missing(key) => {
                write!(f, "Missing required environment variable: {key}")
            }
            ConfigError::Invalid { key, reason } => {
                write!(f, "Invalid value for environment variable {key}: {reason}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

// ─────────────────────────────────────────────────────────────────────────────
// Sub-structs (grouped by concern)
// ─────────────────────────────────────────────────────────────────────────────

/// `APP_ENV` — controls behaviour that differs between local and production.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEnv {
    Development,
    Staging,
    Production,
}

impl AppEnv {
    fn parse(raw: &str) -> Result<Self, String> {
        match raw {
            "development" => Ok(AppEnv::Development),
            "staging" => Ok(AppEnv::Staging),
            "production" => Ok(AppEnv::Production),
            other => Err(format!(
                "expected \"development\", \"staging\" or \"production\", got \"{other}\""
            )),
        }
    }
    /// Returns `true` when running in local development mode.
    #[inline]
    pub fn is_dev(&self) -> bool {
        matches!(self, AppEnv::Development)
    }
    /// Read `APP_ENV` directly from the **real** process environment,
    /// bypassing dotenvy.  Call this as the very first step in `main`.
    pub fn from_real_env() -> Result<Self, ConfigError> {
        match env::var("APP_ENV") {
            Err(_) => Ok(AppEnv::Development),
            Ok(raw) => Self::parse(&raw).map_err(|reason| ConfigError::Invalid {
                key: "APP_ENV".to_string(),
                reason,
            }),
        }
    }
}

impl fmt::Display for AppEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppEnv::Development => write!(f, "development"),
            AppEnv::Staging => write!(f, "staging"),
            AppEnv::Production => write!(f, "production"),
        }
    }
}

/// `STORAGE_PROVIDER` — selects the storage backend.
///
/// Only `local` is implemented in the MVP; `s3` is reserved for V2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageProvider {
    Local,
    S3,
}

impl StorageProvider {
    fn parse(raw: &str) -> Result<Self, String> {
        match raw {
            "local" => Ok(StorageProvider::Local),
            "s3" => Ok(StorageProvider::S3),
            other => Err(format!("expected \"local\" or \"s3\", got \"{other}\"")),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// AppConfig
// ─────────────────────────────────────────────────────────────────────────────

/// Full application configuration, populated once at startup from env vars.
#[derive(Debug, Clone)]
pub struct AppConfig {
    // ── Application ─────────────────────────────────────────────────────────
    /// TCP port to listen on (defaults to 3000 when `APP_PORT` is absent).
    pub app_port: u16,
    /// Runtime environment — governs logging verbosity and security settings.
    pub app_env: AppEnv,

    // ── Database ─────────────────────────────────────────────────────────────
    /// Full PostgreSQL DSN: `postgresql://user:pass@host:port/db`.
    pub database_url: String,

    // ── JWT ──────────────────────────────────────────────────────────────────
    /// Raw secret used to sign and verify JWTs (≥ 32 bytes enforced).
    pub jwt_secret: String,
    /// Access-token lifetime in seconds (e.g. 900 = 15 min).
    pub jwt_access_ttl_seconds: u64,
    /// Refresh-token lifetime in days (sliding TTL, e.g. 30).
    pub jwt_refresh_ttl_days: u64,

    // ── OTP ──────────────────────────────────────────────────────────────────
    /// Time-to-live for an OTP code in seconds (e.g. 600 = 10 min).
    pub otp_ttl_seconds: u64,
    /// Maximum verification attempts per OTP before it is invalidated.
    pub otp_max_attempts: u32,
    /// Minimum gap between two OTP requests for the same email (in seconds).
    pub otp_rate_limit_seconds: u64,

    // ── Storage ──────────────────────────────────────────────────────────────
    /// Which storage backend to use.
    pub storage_provider: StorageProvider,
    /// Root path on the filesystem for `local` storage (e.g. `/app/storage`).
    pub local_storage_path: String,
    /// Base URL used to build public media links (e.g. `http://localhost/media`).
    pub public_media_base_url: String,

    // ── Cookies ──────────────────────────────────────────────────────────────
    /// Domain attribute of the `refresh_token` cookie (e.g. `localhost`).
    pub cookie_domain: String,

    // ── Email / SMTP ─────────────────────────────────────────────────────────
    pub smtp_host: String,
    pub smtp_port: u16,
    /// Sender address for outgoing mail (e.g. `noreply@myhouse.app`).
    pub smtp_from: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Loading helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Read a required string variable. Returns `ConfigError::Missing` if absent.
fn require(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::Missing(key.to_string()))
}

/// Read an optional variable, returning `default` if absent.
#[allow(dead_code)] // used by optional vars added in future tickets
fn optional_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Parse a required variable as `T` using a custom parser closure.
fn require_parsed<T, F>(key: &str, parse: F) -> Result<T, ConfigError>
where
    F: Fn(&str) -> Result<T, String>,
{
    let raw = require(key)?;
    parse(&raw).map_err(|reason| ConfigError::Invalid {
        key: key.to_string(),
        reason,
    })
}

/// Parse a required variable as `u16`.
fn require_u16(key: &str) -> Result<u16, ConfigError> {
    require_parsed(key, |v| {
        v.parse::<u16>()
            .map_err(|_| format!("expected a port number (0–65535), got \"{v}\""))
    })
}

/// Parse a required variable as `u32`.
fn require_u32(key: &str) -> Result<u32, ConfigError> {
    require_parsed(key, |v| {
        v.parse::<u32>()
            .map_err(|_| format!("expected an unsigned integer, got \"{v}\""))
    })
}

/// Parse a required variable as `u64`.
fn require_u64(key: &str) -> Result<u64, ConfigError> {
    require_parsed(key, |v| {
        v.parse::<u64>()
            .map_err(|_| format!("expected an unsigned integer, got \"{v}\""))
    })
}

/// Parse an optional variable as `u16`, falling back to `default`.
fn optional_u16_or(key: &str, default: u16) -> Result<u16, ConfigError> {
    match env::var(key) {
        Err(_) => Ok(default),
        Ok(raw) => raw.parse::<u16>().map_err(|_| ConfigError::Invalid {
            key: key.to_string(),
            reason: format!("expected a port number (0–65535), got \"{raw}\""),
        }),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// AppConfig::from_env
// ─────────────────────────────────────────────────────────────────────────────

impl AppConfig {
    /// Load and validate all configuration from the current environment.
    ///
    /// **Must be called after `dotenvy::dotenv_override()`** (or equivalent)
    /// so that `.env` values are already populated in the process environment.
    ///
    /// `app_env` is resolved once by the caller (via [`AppEnv::from_real_env`])
    /// and passed in here rather than being re-parsed from `APP_ENV`, so that
    /// a single value is used consistently instead of two independent reads
    /// that could observe different environment states (e.g. before/after
    /// dotenvy loads a `.env` file).
    ///
    /// # Errors
    /// Returns the **first** validation error encountered. The caller is
    /// expected to print the error and exit; there is no partial configuration.
    pub fn from_env(app_env: AppEnv) -> Result<Self, ConfigError> {
        // ── Application ───────────────────────────────────────────────────────
        let app_port = optional_u16_or("APP_PORT", 3000)?;

        // ── Database ──────────────────────────────────────────────────────────
        let database_url = require("DATABASE_URL")?;

        // ── JWT ───────────────────────────────────────────────────────────────
        let jwt_secret = require("JWT_SECRET")?;
        if jwt_secret.len() < 32 {
            return Err(ConfigError::Invalid {
                key: "JWT_SECRET".to_string(),
                reason: format!(
                    "must be at least 32 bytes (256 bits), got {} byte(s)",
                    jwt_secret.len()
                ),
            });
        }
        let jwt_access_ttl_seconds = require_u64("JWT_ACCESS_TTL_SECONDS")?;
        let jwt_refresh_ttl_days = require_u64("JWT_REFRESH_TTL_DAYS")?;

        // ── OTP ───────────────────────────────────────────────────────────────
        let otp_ttl_seconds = require_u64("OTP_TTL_SECONDS")?;
        let otp_max_attempts = require_u32("OTP_MAX_ATTEMPTS")?;
        let otp_rate_limit_seconds = require_u64("OTP_RATE_LIMIT_SECONDS")?;

        // ── Storage ───────────────────────────────────────────────────────────
        let storage_provider = require_parsed("STORAGE_PROVIDER", StorageProvider::parse)?;
        let local_storage_path = require("LOCAL_STORAGE_PATH")?;
        let public_media_base_url = require("PUBLIC_MEDIA_BASE_URL")?;

        // ── Cookies ───────────────────────────────────────────────────────────
        let cookie_domain = require("COOKIE_DOMAIN")?;

        // ── Email / SMTP ──────────────────────────────────────────────────────
        let smtp_host = require("SMTP_HOST")?;
        let smtp_port = require_u16("SMTP_PORT")?;
        let smtp_from = require("SMTP_FROM")?;

        Ok(AppConfig {
            app_port,
            app_env,
            database_url,
            jwt_secret,
            jwt_access_ttl_seconds,
            jwt_refresh_ttl_days,
            otp_ttl_seconds,
            otp_max_attempts,
            otp_rate_limit_seconds,
            storage_provider,
            local_storage_path,
            public_media_base_url,
            cookie_domain,
            smtp_host,
            smtp_port,
            smtp_from,
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Environment variables are process-global state. Tests that mutate them
    // MUST be serialized; otherwise parallel runners read each other's leftovers.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    /// Populate every required env var with valid values.
    /// Must be called while holding `ENV_LOCK`.
    fn set_valid_env() {
        env::set_var("APP_PORT", "3000");
        env::set_var("APP_ENV", "development");
        env::set_var(
            "DATABASE_URL",
            "postgresql://myhouse:myhouse@localhost:5432/myhouse",
        );
        env::set_var(
            "JWT_SECRET",
            "a-super-secret-key-that-is-at-least-32-bytes-long!",
        );
        env::set_var("JWT_ACCESS_TTL_SECONDS", "900");
        env::set_var("JWT_REFRESH_TTL_DAYS", "30");
        env::set_var("OTP_TTL_SECONDS", "600");
        env::set_var("OTP_MAX_ATTEMPTS", "3");
        env::set_var("OTP_RATE_LIMIT_SECONDS", "60");
        env::set_var("STORAGE_PROVIDER", "local");
        env::set_var("LOCAL_STORAGE_PATH", "/app/storage");
        env::set_var("PUBLIC_MEDIA_BASE_URL", "http://localhost/media");
        env::set_var("COOKIE_DOMAIN", "localhost");
        env::set_var("SMTP_HOST", "localhost");
        env::set_var("SMTP_PORT", "1025");
        env::set_var("SMTP_FROM", "noreply@myhouse.app");
    }

    #[test]
    fn loads_valid_config() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        let cfg = AppConfig::from_env(AppEnv::Development).expect("should load without error");
        assert_eq!(cfg.app_port, 3000);
        assert_eq!(cfg.app_env, AppEnv::Development);
        assert_eq!(cfg.jwt_access_ttl_seconds, 900);
        assert_eq!(cfg.jwt_refresh_ttl_days, 30);
        assert_eq!(cfg.otp_ttl_seconds, 600);
        assert_eq!(cfg.otp_max_attempts, 3);
        assert_eq!(cfg.storage_provider, StorageProvider::Local);
        assert_eq!(cfg.smtp_port, 1025);
    }

    #[test]
    fn rejects_short_jwt_secret() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("JWT_SECRET", "tooshort");
        let err =
            AppConfig::from_env(AppEnv::Development).expect_err("should fail on short secret");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "JWT_SECRET"));
    }

    #[test]
    fn rejects_missing_database_url() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::remove_var("DATABASE_URL");
        let err = AppConfig::from_env(AppEnv::Development)
            .expect_err("should fail on missing DATABASE_URL");
        assert!(matches!(err, ConfigError::Missing(key) if key == "DATABASE_URL"));
    }

    #[test]
    fn rejects_invalid_app_env() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("APP_ENV", "unknown");
        let err = AppEnv::from_real_env().expect_err("should fail on unknown APP_ENV");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "APP_ENV"));
    }

    #[test]
    fn rejects_invalid_smtp_port() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("SMTP_PORT", "not-a-port");
        let err =
            AppConfig::from_env(AppEnv::Development).expect_err("should fail on invalid SMTP_PORT");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "SMTP_PORT"));
    }

    #[test]
    fn app_port_defaults_to_3000_when_absent() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::remove_var("APP_PORT");
        let cfg = AppConfig::from_env(AppEnv::Development).expect("should load with default port");
        assert_eq!(cfg.app_port, 3000);
    }
}
