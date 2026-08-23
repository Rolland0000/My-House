use sqlx::PgPool;

use crate::config::AppConfig;
use crate::shared::errors::AppError;

use super::repository;

/// Startup-only entry point — called once from `main` before the HTTP server
/// starts accepting requests. There is no HTTP route that reaches this: the
/// admin role is never creatable through the API surface. No-op when
/// disabled or when an admin row already exists — never reconciles, merges,
/// or deletes.
pub async fn bootstrap_admin(pool: &PgPool, config: &AppConfig) -> Result<(), AppError> {
    if !config.admin_bootstrap_on_startup {
        return Ok(());
    }
    // Guaranteed `Some` when the flag is on — `AppConfig::from_env` rejects the
    // combination at startup.
    let Some(email) = config.admin_bootstrap_email.as_deref() else {
        return Ok(());
    };
    if repository::admin_exists(pool).await? {
        return Ok(());
    }
    repository::upsert_admin(pool, email).await?;
    tracing::info!(email, "admin account bootstrapped");
    Ok(())
}
