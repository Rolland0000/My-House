//! Admin account settings: bootstrap at startup and notification recipient.

use std::{env, fmt};

use super::app_config::AppEnv;
use super::config_error::ConfigError;
use super::env_reader::{optional_bool_or, optional_non_empty, require};
use crate::shared::validation::normalize_email;

/// Value shipped in `.env.example`; accepted in development only.
pub const ADMIN_PASSWORD_PLACEHOLDER: &str = "change-me-admin-password";

/// Shortest admin password accepted outside development.
pub const ADMIN_PASSWORD_MIN_LENGTH: usize = 12;

/// `ADMIN_BOOTSTRAP_PASSWORD` — initial admin password, kept out of logs.
#[derive(Clone)]
pub struct AdminBootstrapPassword(String);

impl AdminBootstrapPassword {
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for AdminBootstrapPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AdminBootstrapPassword(<redacted>)")
    }
}

#[derive(Debug, Clone)]
pub struct AdminConfig {
    /// Fixed recipient for admin notifications (owner-request received, …).
    /// A config value, not a DB query — the admin role has exactly one holder
    /// at MVP (TECHNICAL_SPEC_MVP.md §3bis.2).
    pub notification_email: String,
    /// If `true`, checks/creates the single admin account at startup.
    pub bootstrap_on_startup: bool,
    /// Email to bootstrap the admin account with. Required when
    /// `bootstrap_on_startup` is `true` — validated in [`AdminConfig::from_env`].
    pub bootstrap_email: Option<String>,
    /// Initial admin password. Required when `bootstrap_on_startup` is
    /// `true`; outside development it must meet [`ADMIN_PASSWORD_MIN_LENGTH`]
    /// and differ from [`ADMIN_PASSWORD_PLACEHOLDER`].
    pub bootstrap_password: Option<AdminBootstrapPassword>,
}

impl AdminConfig {
    pub(super) fn from_env(app_env: &AppEnv) -> Result<Self, ConfigError> {
        let notification_email = require("ADMIN_NOTIFICATION_EMAIL")?;
        let bootstrap_on_startup = optional_bool_or("ADMIN_BOOTSTRAP_ON_STARTUP", false)?;
        // Normalised like every sign-in address: `users.email` is stored in
        // canonical form, and the admin login looks it up the same way.
        let bootstrap_email = match env::var("ADMIN_BOOTSTRAP_EMAIL") {
            Ok(raw) if !raw.trim().is_empty() => {
                Some(normalize_email(&raw).ok_or_else(|| ConfigError::Invalid {
                    key: "ADMIN_BOOTSTRAP_EMAIL".to_string(),
                    reason: "must be a valid ASCII email address".to_string(),
                })?)
            }
            _ => None,
        };
        if bootstrap_on_startup && bootstrap_email.is_none() {
            return Err(ConfigError::Missing("ADMIN_BOOTSTRAP_EMAIL".to_string()));
        }
        let bootstrap_password = admin_bootstrap_password(app_env, bootstrap_on_startup)?;
        Ok(Self {
            notification_email,
            bootstrap_on_startup,
            bootstrap_email,
            bootstrap_password,
        })
    }
}

/// Read `ADMIN_BOOTSTRAP_PASSWORD`. Not trimmed: spaces are legitimate password
/// characters. Only read when the bootstrap is enabled; outside development the
/// strength policy is enforced here so a weak password fails the boot.
fn admin_bootstrap_password(
    app_env: &AppEnv,
    bootstrap_enabled: bool,
) -> Result<Option<AdminBootstrapPassword>, ConfigError> {
    if !bootstrap_enabled {
        return Ok(None);
    }
    let password = optional_non_empty("ADMIN_BOOTSTRAP_PASSWORD")
        .ok_or_else(|| ConfigError::Missing("ADMIN_BOOTSTRAP_PASSWORD".to_string()))?;

    if !app_env.is_dev() {
        let reason = if password.chars().count() < ADMIN_PASSWORD_MIN_LENGTH {
            Some(format!(
                "must be at least {ADMIN_PASSWORD_MIN_LENGTH} characters (APP_ENV={app_env})"
            ))
        } else if password == ADMIN_PASSWORD_PLACEHOLDER {
            Some(format!(
                "must not be the .env.example placeholder (APP_ENV={app_env})"
            ))
        } else {
            None
        };
        if let Some(reason) = reason {
            return Err(ConfigError::Invalid {
                key: "ADMIN_BOOTSTRAP_PASSWORD".to_string(),
                reason,
            });
        }
    }
    Ok(Some(AdminBootstrapPassword(password)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::test_support::{set_valid_env, ENV_LOCK};

    fn enable_admin_bootstrap(password: &str) {
        env::set_var("ADMIN_BOOTSTRAP_ON_STARTUP", "true");
        env::set_var("ADMIN_BOOTSTRAP_EMAIL", "root@myhouse.app");
        env::set_var("ADMIN_BOOTSTRAP_PASSWORD", password);
    }

    fn load(app_env: AppEnv) -> Result<AdminConfig, ConfigError> {
        AdminConfig::from_env(&app_env)
    }

    #[test]
    fn loads_the_notification_email() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        let admin = load(AppEnv::Development).expect("should load");
        assert_eq!(admin.notification_email, "admin@myhouse.app");
    }

    #[test]
    fn rejects_missing_notification_email() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::remove_var("ADMIN_NOTIFICATION_EMAIL");
        let err = load(AppEnv::Development).expect_err("should fail without a recipient");
        assert!(matches!(err, ConfigError::Missing(key) if key == "ADMIN_NOTIFICATION_EMAIL"));
    }

    #[test]
    fn bootstrap_defaults_to_disabled_when_absent() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        let admin = load(AppEnv::Development).expect("should load");
        assert!(!admin.bootstrap_on_startup);
        assert_eq!(admin.bootstrap_email, None);
        assert!(admin.bootstrap_password.is_none());
    }

    #[test]
    fn rejects_bootstrap_enabled_without_email() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("ADMIN_BOOTSTRAP_ON_STARTUP", "true");
        let err = load(AppEnv::Development).expect_err("email is required when enabled");
        assert!(matches!(err, ConfigError::Missing(key) if key == "ADMIN_BOOTSTRAP_EMAIL"));
    }

    #[test]
    fn loads_bootstrap_email_when_enabled() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        enable_admin_bootstrap("dev-only-password");
        let admin = load(AppEnv::Development).expect("should load");
        assert!(admin.bootstrap_on_startup);
        assert_eq!(admin.bootstrap_email.as_deref(), Some("root@myhouse.app"));
    }

    #[test]
    fn normalizes_the_bootstrap_email() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        enable_admin_bootstrap("dev-only-password");
        env::set_var("ADMIN_BOOTSTRAP_EMAIL", "  Root@MyHouse.app  ");
        let admin = load(AppEnv::Development).expect("should load");
        assert_eq!(admin.bootstrap_email.as_deref(), Some("root@myhouse.app"));
    }

    #[test]
    fn rejects_an_invalid_bootstrap_email() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        enable_admin_bootstrap("dev-only-password");
        env::set_var("ADMIN_BOOTSTRAP_EMAIL", "not-an-email");
        let err = load(AppEnv::Development).expect_err("an invalid address must fail startup");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "ADMIN_BOOTSTRAP_EMAIL"));
    }

    #[test]
    fn rejects_bootstrap_email_that_is_only_whitespace() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("ADMIN_BOOTSTRAP_ON_STARTUP", "true");
        env::set_var("ADMIN_BOOTSTRAP_EMAIL", "   ");
        let err = load(AppEnv::Development)
            .expect_err("whitespace-only email should not satisfy the requirement");
        assert!(matches!(err, ConfigError::Missing(key) if key == "ADMIN_BOOTSTRAP_EMAIL"));
    }

    #[test]
    fn rejects_non_boolean_bootstrap_flag() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("ADMIN_BOOTSTRAP_ON_STARTUP", "yes");
        let err = load(AppEnv::Development).expect_err("should fail on a non-boolean flag");
        assert!(
            matches!(err, ConfigError::Invalid { key, .. } if key == "ADMIN_BOOTSTRAP_ON_STARTUP")
        );
    }

    #[test]
    fn rejects_bootstrap_enabled_without_password() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        enable_admin_bootstrap("");
        let err = load(AppEnv::Development).expect_err("bootstrap without a password must fail");
        assert!(matches!(err, ConfigError::Missing(key) if key == "ADMIN_BOOTSTRAP_PASSWORD"));
    }

    #[test]
    fn password_is_ignored_when_bootstrap_is_disabled() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("ADMIN_BOOTSTRAP_PASSWORD", "short");
        let admin = load(AppEnv::Production).expect("a disabled bootstrap skips the policy");
        assert!(admin.bootstrap_password.is_none());
    }

    #[test]
    fn development_accepts_the_placeholder_and_short_passwords() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        enable_admin_bootstrap(ADMIN_PASSWORD_PLACEHOLDER);
        assert!(load(AppEnv::Development).is_ok());
        env::set_var("ADMIN_BOOTSTRAP_PASSWORD", "short");
        assert!(load(AppEnv::Development).is_ok());
    }

    #[test]
    fn production_rejects_short_password() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        enable_admin_bootstrap("elevenchars");
        let err = load(AppEnv::Production).expect_err("11 characters is too short");
        assert!(
            matches!(err, ConfigError::Invalid { key, .. } if key == "ADMIN_BOOTSTRAP_PASSWORD")
        );
    }

    #[test]
    fn staging_rejects_short_password() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        enable_admin_bootstrap("elevenchars");
        assert!(load(AppEnv::Staging).is_err());
    }

    #[test]
    fn production_rejects_the_env_example_placeholder() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        enable_admin_bootstrap(ADMIN_PASSWORD_PLACEHOLDER);
        let err = load(AppEnv::Production).expect_err("placeholder must be rejected");
        assert!(
            matches!(err, ConfigError::Invalid { key, .. } if key == "ADMIN_BOOTSTRAP_PASSWORD")
        );
    }

    #[test]
    fn production_accepts_a_twelve_character_password() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        enable_admin_bootstrap("twelve-chars");
        let admin = load(AppEnv::Production).expect("12 characters is enough");
        assert_eq!(
            admin.bootstrap_password.as_ref().map(|p| p.expose()),
            Some("twelve-chars")
        );
    }

    #[test]
    fn password_length_counts_characters_not_bytes() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        // 6 chars, 12 bytes: must still be rejected as too short.
        enable_admin_bootstrap("ééééée");
        assert!(load(AppEnv::Production).is_err());
    }

    #[test]
    fn password_debug_is_redacted() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        enable_admin_bootstrap("dev-only-password");
        let admin = load(AppEnv::Development).expect("should load");
        assert!(!format!("{admin:?}").contains("dev-only-password"));
    }
}
