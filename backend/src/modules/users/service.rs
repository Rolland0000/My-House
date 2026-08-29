use sqlx::PgPool;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::shared::errors::AppError;

use super::dto::UpdateMeDto;
use super::model::UserRow;
use super::repository;

const MAX_NAME_LENGTH: usize = 100;
const MAX_PHONE_LENGTH: usize = 30;

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

pub async fn update_me(
    pool: &PgPool,
    user_id: Uuid,
    payload: UpdateMeDto,
) -> Result<UserRow, AppError> {
    let first_name = required_name(&payload.first_name, "first_name")?;
    let last_name = required_name(&payload.last_name, "last_name")?;
    let phone = match payload.phone.as_deref().map(str::trim) {
        Some("") | None => None,
        Some(value) if value.chars().count() > MAX_PHONE_LENGTH => {
            return Err(AppError::BadRequest(format!(
                "phone must be at most {MAX_PHONE_LENGTH} characters."
            )))
        }
        Some(value) => Some(value),
    };

    repository::update_profile(pool, user_id, first_name, last_name, phone)
        .await?
        .ok_or(AppError::UserNotFound)
}

fn required_name<'a>(raw: &'a str, field: &str) -> Result<&'a str, AppError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(AppError::BadRequest(format!("{field} is required.")));
    }
    if trimmed.chars().count() > MAX_NAME_LENGTH {
        return Err(AppError::BadRequest(format!(
            "{field} must be at most {MAX_NAME_LENGTH} characters."
        )));
    }
    Ok(trimmed)
}
