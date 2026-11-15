//! General-purpose settings and the root [`AppConfig`] that joins every group.

use std::{env, fmt};

use axum::http::HeaderValue;

use super::config_admin::AdminConfig;
use super::config_auth::AuthConfig;
use super::config_error::ConfigError;
use super::config_rate_limit::RateLimitConfig;
use super::config_smtp::SmtpConfig;
use super::config_storage::StorageConfig;
use super::env_reader::{optional_u16_or, require, require_parsed};

/// Port used when `APP_PORT` is absent.
const DEFAULT_APP_PORT: u16 = 3000;

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

/// Full application configuration, populated once at startup from env vars.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// TCP port to listen on (defaults to 3000 when `APP_PORT` is absent).
    pub app_port: u16,
    /// Runtime environment — governs logging verbosity and security settings.
    pub app_env: AppEnv,
    /// Full PostgreSQL DSN: `postgresql://user:pass@host:port/db`.
    pub database_url: String,
    /// Origins allowed to make cross-origin requests (`ALLOWED_ORIGINS`, comma-separated).
    pub allowed_origins: Vec<String>,
    pub auth: AuthConfig,
    pub storage: StorageConfig,
    pub smtp: SmtpConfig,
    pub admin: AdminConfig,
    pub rate_limit: RateLimitConfig,
}

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
        let app_port = optional_u16_or("APP_PORT", DEFAULT_APP_PORT)?;
        let database_url = require("DATABASE_URL")?;
        let auth = AuthConfig::from_env()?;
        let storage = StorageConfig::from_env()?;
        let allowed_origins = require_origins("ALLOWED_ORIGINS")?;
        let smtp = SmtpConfig::from_env(&app_env)?;
        let admin = AdminConfig::from_env(&app_env)?;
        let rate_limit = RateLimitConfig::from_env()?;

        Ok(AppConfig {
            app_port,
            app_env,
            database_url,
            allowed_origins,
            auth,
            storage,
            smtp,
            admin,
            rate_limit,
        })
    }
}

/// Parse a required comma-separated origin list (e.g. `http://localhost,http://localhost:5173`).
fn require_origins(key: &str) -> Result<Vec<String>, ConfigError> {
    require_parsed(key, |raw| {
        let origins: Vec<String> = raw
            .split(',')
            .map(|origin| origin.trim().to_owned())
            .collect();

        for origin in &origins {
            if origin.is_empty() {
                return Err(
                    "must not contain an empty origin (check for a stray comma)".to_string()
                );
            }
            if !(origin.starts_with("http://") || origin.starts_with("https://")) {
                return Err(format!(
                    "origin \"{origin}\" must start with http:// or https://"
                ));
            }
            if origin.ends_with('/') {
                return Err(format!(
                    "origin \"{origin}\" must not have a trailing slash"
                ));
            }
            if HeaderValue::from_str(origin).is_err() {
                return Err(format!(
                    "origin \"{origin}\" is not a valid HTTP header value"
                ));
            }
        }
        Ok(origins)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::config_smtp::SmtpSecurity;
    use crate::config::config_storage::StorageProvider;
    use crate::config::test_support::{set_valid_env, ENV_LOCK};

    #[test]
    fn loads_valid_config() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        let cfg = AppConfig::from_env(AppEnv::Development).expect("should load without error");
        assert_eq!(cfg.app_port, 3000);
        assert_eq!(cfg.app_env, AppEnv::Development);
        assert_eq!(
            cfg.database_url,
            "postgresql://myhouse:myhouse@localhost:5432/myhouse"
        );
        assert_eq!(cfg.auth.jwt_access_ttl_seconds, 900);
        assert_eq!(cfg.storage.provider, StorageProvider::Local);
        assert_eq!(cfg.smtp.port, 1025);
        assert_eq!(cfg.smtp.security, SmtpSecurity::None);
        assert_eq!(cfg.admin.notification_email, "admin@myhouse.app");
        assert_eq!(cfg.rate_limit.max_requests, 100);
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
    fn propagates_group_errors() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("JWT_SECRET", "tooshort");
        let err = AppConfig::from_env(AppEnv::Development)
            .expect_err("an invalid group must fail the whole load");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "JWT_SECRET"));
    }

    #[test]
    fn passes_the_resolved_app_env_to_the_groups() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        // Valid in development, rejected elsewhere: proves the argument is used,
        // not the APP_ENV variable (still "development" here).
        env::set_var("SMTP_USERNAME", "mailer");
        env::set_var("SMTP_PASSWORD", "s3cret");
        assert!(AppConfig::from_env(AppEnv::Development).is_ok());
        let err = AppConfig::from_env(AppEnv::Production)
            .expect_err("plaintext credentials are refused in production");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "SMTP_SECURITY"));
    }

    #[test]
    fn config_debug_does_not_leak_secrets_wrapped_by_the_groups() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("SMTP_USERNAME", "mailer");
        env::set_var("SMTP_PASSWORD", "smtp-s3cret");
        env::set_var("ADMIN_BOOTSTRAP_ON_STARTUP", "true");
        env::set_var("ADMIN_BOOTSTRAP_EMAIL", "root@myhouse.app");
        env::set_var("ADMIN_BOOTSTRAP_PASSWORD", "dev-only-password");
        let cfg = AppConfig::from_env(AppEnv::Development).expect("should load");
        let rendered = format!("{cfg:?}");
        assert!(!rendered.contains("smtp-s3cret"));
        assert!(!rendered.contains("dev-only-password"));
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
    fn app_env_defaults_to_development_when_absent() {
        let _guard = ENV_LOCK.lock().unwrap();
        env::remove_var("APP_ENV");
        assert_eq!(AppEnv::from_real_env().unwrap(), AppEnv::Development);
    }

    #[test]
    fn app_env_parses_every_environment_and_displays_it_back() {
        for (raw, expected) in [
            ("development", AppEnv::Development),
            ("staging", AppEnv::Staging),
            ("production", AppEnv::Production),
        ] {
            let parsed = AppEnv::parse(raw).expect("known environment");
            assert_eq!(parsed, expected);
            assert_eq!(parsed.to_string(), raw);
        }
    }

    #[test]
    fn only_development_is_dev() {
        assert!(AppEnv::Development.is_dev());
        assert!(!AppEnv::Staging.is_dev());
        assert!(!AppEnv::Production.is_dev());
    }

    #[test]
    fn app_port_defaults_to_3000_when_absent() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::remove_var("APP_PORT");
        let cfg = AppConfig::from_env(AppEnv::Development).expect("should load with default port");
        assert_eq!(cfg.app_port, DEFAULT_APP_PORT);
    }

    #[test]
    fn rejects_invalid_app_port() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("APP_PORT", "70000");
        let err = AppConfig::from_env(AppEnv::Development).expect_err("port is out of range");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "APP_PORT"));
    }

    #[test]
    fn parses_allowed_origins_into_a_list() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        let cfg = AppConfig::from_env(AppEnv::Development).expect("should load without error");
        assert_eq!(
            cfg.allowed_origins,
            vec![
                "http://localhost".to_string(),
                "http://localhost:5173".to_string()
            ]
        );
    }

    #[test]
    fn rejects_allowed_origins_without_scheme() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("ALLOWED_ORIGINS", "localhost:5173");
        let err = AppConfig::from_env(AppEnv::Development)
            .expect_err("should fail on origin missing a scheme");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "ALLOWED_ORIGINS"));
    }

    #[test]
    fn rejects_allowed_origins_with_a_stray_comma() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("ALLOWED_ORIGINS", "http://localhost,,http://localhost:5173");
        let err = AppConfig::from_env(AppEnv::Development)
            .expect_err("should fail on an empty origin entry");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "ALLOWED_ORIGINS"));
    }

    #[test]
    fn rejects_allowed_origins_with_a_trailing_slash() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("ALLOWED_ORIGINS", "http://localhost/");
        let err =
            AppConfig::from_env(AppEnv::Development).expect_err("should fail on a trailing slash");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "ALLOWED_ORIGINS"));
    }
}
