//! Outgoing mail (SMTP relay) settings.

use std::fmt;

use super::app_config::AppEnv;
use super::config_error::ConfigError;
use super::env_reader::{optional_non_empty, optional_or, require, require_u16};

/// `SMTP_SECURITY` — how the connection to the SMTP relay is encrypted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmtpSecurity {
    /// Plaintext, e.g. a local dev catcher.
    None,
    /// Plaintext connection upgraded with STARTTLS.
    StartTls,
    /// Implicit TLS from the first byte (SMTPS).
    Tls,
}

impl SmtpSecurity {
    fn parse(raw: &str) -> Result<Self, String> {
        match raw {
            "none" => Ok(SmtpSecurity::None),
            "starttls" => Ok(SmtpSecurity::StartTls),
            "tls" => Ok(SmtpSecurity::Tls),
            other => Err(format!(
                "expected \"none\", \"starttls\" or \"tls\", got \"{other}\""
            )),
        }
    }
}

/// `SMTP_USERNAME` / `SMTP_PASSWORD` for an authenticated relay.
#[derive(Clone)]
pub struct SmtpCredentials {
    pub username: String,
    pub password: String,
}

// Hand-written so the password never ends up in logs through `AppConfig`'s `Debug`.
impl fmt::Debug for SmtpCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SmtpCredentials")
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    /// Connection encryption (`SMTP_SECURITY`, defaults to `none`).
    pub security: SmtpSecurity,
    /// Relay login. `None` sends unauthenticated, which only development allows.
    pub credentials: Option<SmtpCredentials>,
    /// Sender address for outgoing mail (e.g. `noreply@myhouse.app`).
    pub from: String,
}

impl SmtpConfig {
    pub(super) fn from_env(app_env: &AppEnv) -> Result<Self, ConfigError> {
        let host = require("SMTP_HOST")?;
        let port = require_u16("SMTP_PORT")?;
        let security =
            SmtpSecurity::parse(&optional_or("SMTP_SECURITY", "none")).map_err(|reason| {
                ConfigError::Invalid {
                    key: "SMTP_SECURITY".to_string(),
                    reason,
                }
            })?;
        let credentials = smtp_credentials(app_env, security)?;
        let from = require("SMTP_FROM")?;
        Ok(Self {
            host,
            port,
            security,
            credentials,
            from,
        })
    }
}

/// Read `SMTP_USERNAME` / `SMTP_PASSWORD`. They must come as a pair. Outside
/// development they are mandatory and need an encrypted connection: lettre
/// sends AUTH over plaintext without complaint, which would leak the password.
fn smtp_credentials(
    app_env: &AppEnv,
    smtp_security: SmtpSecurity,
) -> Result<Option<SmtpCredentials>, ConfigError> {
    let credentials = match (
        optional_non_empty("SMTP_USERNAME"),
        optional_non_empty("SMTP_PASSWORD"),
    ) {
        (Some(username), Some(password)) => SmtpCredentials { username, password },
        (Some(_), None) => return Err(ConfigError::Missing("SMTP_PASSWORD".to_string())),
        (None, Some(_)) => return Err(ConfigError::Missing("SMTP_USERNAME".to_string())),
        (None, None) if app_env.is_dev() => return Ok(None),
        (None, None) => {
            return Err(ConfigError::Missing(
                "SMTP_USERNAME and SMTP_PASSWORD".to_string(),
            ))
        }
    };

    if smtp_security == SmtpSecurity::None && !app_env.is_dev() {
        return Err(ConfigError::Invalid {
            key: "SMTP_SECURITY".to_string(),
            reason: format!(
                "must be \"starttls\" or \"tls\" when SMTP credentials are set (APP_ENV={app_env})"
            ),
        });
    }
    Ok(Some(credentials))
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use crate::config::test_support::{set_valid_env, ENV_LOCK};

    /// Sets both SMTP credentials.
    fn set_smtp_credentials() {
        env::set_var("SMTP_USERNAME", "mailer");
        env::set_var("SMTP_PASSWORD", "s3cret");
    }

    #[test]
    fn loads_host_port_and_sender() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        let smtp = SmtpConfig::from_env(&AppEnv::Development).expect("should load");
        assert_eq!(smtp.host, "localhost");
        assert_eq!(smtp.port, 1025);
        assert_eq!(smtp.from, "noreply@myhouse.app");
    }

    #[test]
    fn rejects_invalid_smtp_port() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("SMTP_PORT", "not-a-port");
        let err = SmtpConfig::from_env(&AppEnv::Development)
            .expect_err("should fail on invalid SMTP_PORT");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "SMTP_PORT"));
    }

    #[test]
    fn dev_without_credentials_sends_plaintext_unauthenticated() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        let smtp = SmtpConfig::from_env(&AppEnv::Development).expect("should load");
        assert_eq!(smtp.security, SmtpSecurity::None);
        assert!(smtp.credentials.is_none());
    }

    #[test]
    fn empty_credentials_count_as_absent() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("SMTP_USERNAME", "");
        env::set_var("SMTP_PASSWORD", "");
        let smtp = SmtpConfig::from_env(&AppEnv::Development).expect("should load");
        assert!(smtp.credentials.is_none());
    }

    #[test]
    fn rejects_username_without_password() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("SMTP_USERNAME", "mailer");
        let err = SmtpConfig::from_env(&AppEnv::Development)
            .expect_err("should fail when only SMTP_USERNAME is set");
        assert!(matches!(err, ConfigError::Missing(key) if key == "SMTP_PASSWORD"));
    }

    #[test]
    fn rejects_password_without_username() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("SMTP_PASSWORD", "s3cret");
        let err = SmtpConfig::from_env(&AppEnv::Development)
            .expect_err("should fail when only SMTP_PASSWORD is set");
        assert!(matches!(err, ConfigError::Missing(key) if key == "SMTP_USERNAME"));
    }

    #[test]
    fn staging_and_production_require_credentials() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("SMTP_SECURITY", "starttls");
        for app_env in [AppEnv::Staging, AppEnv::Production] {
            let err = SmtpConfig::from_env(&app_env).expect_err("should fail without credentials");
            assert!(matches!(err, ConfigError::Missing(key) if key.contains("SMTP_USERNAME")));
        }
    }

    #[test]
    fn production_rejects_credentials_over_plaintext() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        set_smtp_credentials();
        let err = SmtpConfig::from_env(&AppEnv::Production)
            .expect_err("should refuse to send credentials unencrypted");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "SMTP_SECURITY"));
    }

    #[test]
    fn dev_allows_credentials_over_plaintext() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        set_smtp_credentials();
        let smtp = SmtpConfig::from_env(&AppEnv::Development).expect("should load");
        assert!(smtp.credentials.is_some());
    }

    #[test]
    fn production_loads_authenticated_encrypted_relay() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        set_smtp_credentials();
        env::set_var("SMTP_SECURITY", "tls");
        let smtp = SmtpConfig::from_env(&AppEnv::Production).expect("should load");
        assert_eq!(smtp.security, SmtpSecurity::Tls);
        let credentials = smtp.credentials.expect("credentials should be set");
        assert_eq!(credentials.username, "mailer");
        assert_eq!(credentials.password, "s3cret");
    }

    #[test]
    fn parses_every_security_mode() {
        assert_eq!(SmtpSecurity::parse("none"), Ok(SmtpSecurity::None));
        assert_eq!(SmtpSecurity::parse("starttls"), Ok(SmtpSecurity::StartTls));
        assert_eq!(SmtpSecurity::parse("tls"), Ok(SmtpSecurity::Tls));
    }

    #[test]
    fn rejects_unknown_security_mode() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("SMTP_SECURITY", "ssl");
        let err = SmtpConfig::from_env(&AppEnv::Development)
            .expect_err("should fail on an unknown SMTP_SECURITY");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "SMTP_SECURITY"));
    }

    #[test]
    fn credentials_debug_redacts_password() {
        let credentials = SmtpCredentials {
            username: "mailer".to_string(),
            password: "s3cret".to_string(),
        };
        let rendered = format!("{credentials:?}");
        assert!(rendered.contains("mailer"));
        assert!(!rendered.contains("s3cret"));
    }
}
