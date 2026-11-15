//! Application configuration loaded **exclusively from environment variables** (12-factor).
//!
//! Call [`AppConfig::from_env`] once at startup. It reads every required variable,
//! validates types and constraints, then returns a fully-populated `AppConfig` or
//! a descriptive [`ConfigError`] that causes the process to exit immediately.
//!
//! Each concern lives in its own `config_*` file and exposes one struct that
//! [`AppConfig`] aggregates; `app_config` holds the general-purpose settings.
//!
//! Variable catalogue — see `backend/.env.example` for the full list.

mod app_config;
mod config_admin;
mod config_auth;
mod config_error;
mod config_rate_limit;
mod config_smtp;
mod config_storage;
mod env_reader;
#[cfg(test)]
mod test_support;

pub use app_config::{AppConfig, AppEnv};
pub use config_admin::{
    AdminBootstrapPassword, AdminConfig, ADMIN_PASSWORD_MIN_LENGTH, ADMIN_PASSWORD_PLACEHOLDER,
};
pub use config_auth::AuthConfig;
pub use config_error::ConfigError;
pub use config_rate_limit::RateLimitConfig;
pub use config_smtp::{SmtpConfig, SmtpCredentials, SmtpSecurity};
pub use config_storage::{StorageConfig, StorageProvider};
