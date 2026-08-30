//! Profile-field validation shared by account creation (`/auth/register`)
//! and profile edition (`PUT /users/me`), so both enforce one set of bounds.

use crate::shared::errors::AppError;

pub const MAX_NAME_LENGTH: usize = 100;
pub const MAX_PHONE_LENGTH: usize = 30;

/// Trims and rejects an empty or over-long mandatory name field.
pub fn required_name<'a>(raw: &'a str, field: &str) -> Result<&'a str, AppError> {
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

/// Same bound as [`required_name`], but absent or blank is `None`, not an error.
pub fn optional_name<'a>(raw: Option<&'a str>, field: &str) -> Result<Option<&'a str>, AppError> {
    match raw.map(str::trim) {
        None | Some("") => Ok(None),
        Some(value) => required_name(value, field).map(Some),
    }
}

/// Trims and rejects an empty or over-long phone number; format is not validated.
pub fn required_phone(raw: &str) -> Result<&str, AppError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(AppError::BadRequest("phone is required.".to_string()));
    }
    if trimmed.chars().count() > MAX_PHONE_LENGTH {
        return Err(AppError::BadRequest(format!(
            "phone must be at most {MAX_PHONE_LENGTH} characters."
        )));
    }
    Ok(trimmed)
}
