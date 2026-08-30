use sqlx::PgPool;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::shared::errors::AppError;
use crate::shared::validation::{optional_name, required_name, required_phone};

use super::dto::UpdateMeDto;
use super::model::UserRow;
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
    if repository::upsert_admin(pool, email).await? {
        tracing::info!(email, "admin account bootstrapped");
    } else {
        tracing::warn!(email, "existing account promoted to admin by bootstrap");
    }
    Ok(())
}

pub async fn get_me(pool: &PgPool, user_id: Uuid) -> Result<UserRow, AppError> {
    repository::find_by_id(pool, user_id)
        .await?
        .ok_or(AppError::UserNotFound)
}

pub async fn update_me(
    pool: &PgPool,
    user_id: Uuid,
    payload: UpdateMeDto,
) -> Result<UserRow, AppError> {
    let first_name = optional_name(payload.first_name.as_deref(), "first_name")?;
    let last_name = required_name(&payload.last_name, "last_name")?;
    let phone = required_phone(&payload.phone)?;

    repository::update_profile(pool, user_id, first_name, last_name, phone)
        .await?
        .ok_or(AppError::UserNotFound)
}
