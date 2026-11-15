//! Shared fixtures for the configuration unit tests.

use std::env;
use std::sync::Mutex;

// Environment variables are process-global state. Tests that mutate them
// MUST hold this lock; otherwise parallel runners read each other's leftovers.
pub(crate) static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Populate every required env var with valid values.
/// Must be called while holding `ENV_LOCK`.
pub(crate) fn set_valid_env() {
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
    env::set_var("COOKIE_DOMAIN", "localhost");
    env::set_var("STORAGE_PROVIDER", "local");
    env::set_var("LOCAL_STORAGE_PATH", "/app/storage");
    env::set_var("PUBLIC_MEDIA_BASE_URL", "http://localhost/media");
    env::set_var("ALLOWED_ORIGINS", "http://localhost,http://localhost:5173");
    env::set_var("SMTP_HOST", "localhost");
    env::set_var("SMTP_PORT", "1025");
    env::remove_var("SMTP_SECURITY");
    env::remove_var("SMTP_USERNAME");
    env::remove_var("SMTP_PASSWORD");
    env::set_var("SMTP_FROM", "noreply@myhouse.app");
    env::set_var("ADMIN_NOTIFICATION_EMAIL", "admin@myhouse.app");
    env::remove_var("ADMIN_BOOTSTRAP_ON_STARTUP");
    env::remove_var("ADMIN_BOOTSTRAP_EMAIL");
    env::remove_var("ADMIN_BOOTSTRAP_PASSWORD");
    env::set_var("RATE_LIMIT_MAX_REQUESTS", "100");
    env::set_var("RATE_LIMIT_WINDOW_SECONDS", "60");
    env::set_var("TRUSTED_PROXIES", "");
    env::set_var("CONTACT_REVEAL_MAX_PER_WINDOW", "20");
    env::set_var("CONTACT_REVEAL_WINDOW_SECONDS", "3600");
}
