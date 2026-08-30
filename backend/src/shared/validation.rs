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

#[cfg(test)]
mod tests {
    use super::*;

    fn repeat(count: usize) -> String {
        "é".repeat(count)
    }

    #[test]
    fn required_name_trims_and_accepts_the_upper_bound() {
        assert_eq!(required_name("  Ada  ", "first_name").unwrap(), "Ada");
        let at_bound = repeat(MAX_NAME_LENGTH);
        assert_eq!(required_name(&at_bound, "last_name").unwrap(), at_bound);
    }

    #[test]
    fn required_name_rejects_blank_and_over_length() {
        assert!(matches!(
            required_name("   ", "last_name"),
            Err(AppError::BadRequest(_))
        ));
        assert!(matches!(
            required_name(&repeat(MAX_NAME_LENGTH + 1), "last_name"),
            Err(AppError::BadRequest(_))
        ));
    }

    #[test]
    fn optional_name_treats_absent_and_blank_as_cleared() {
        assert_eq!(optional_name(None, "first_name").unwrap(), None);
        assert_eq!(optional_name(Some("  "), "first_name").unwrap(), None);
        assert_eq!(
            optional_name(Some(" Ada "), "first_name").unwrap(),
            Some("Ada")
        );
    }

    #[test]
    fn optional_name_still_enforces_the_length_bound() {
        assert!(matches!(
            optional_name(Some(&repeat(MAX_NAME_LENGTH + 1)), "first_name"),
            Err(AppError::BadRequest(_))
        ));
    }

    #[test]
    fn required_phone_trims_and_accepts_the_upper_bound() {
        assert_eq!(required_phone(" +33600000000 ").unwrap(), "+33600000000");
        let at_bound = "0".repeat(MAX_PHONE_LENGTH);
        assert_eq!(required_phone(&at_bound).unwrap(), at_bound);
    }

    #[test]
    fn required_phone_rejects_blank_and_over_length() {
        assert!(matches!(required_phone("  "), Err(AppError::BadRequest(_))));
        assert!(matches!(
            required_phone(&"0".repeat(MAX_PHONE_LENGTH + 1)),
            Err(AppError::BadRequest(_))
        ));
    }
}
