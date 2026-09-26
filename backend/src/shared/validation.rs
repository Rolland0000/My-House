//! Field validation shared across modules: profile fields (`/auth/register`,
//! `PUT /users/me`) fail fast with a `400`; multi-field request bodies use
//! [`FieldErrors`] to report every violation at once as a `422`.

use std::ops::RangeInclusive;

use crate::shared::errors::{AppError, FieldError};

pub const MAX_NAME_LENGTH: usize = 100;
pub const MAX_PHONE_LENGTH: usize = 30;
pub const MAX_ADMIN_NOTE_LENGTH: usize = 1000;

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

/// Same bound as [`required_phone`], but absent or blank is `None`, not an error.
pub fn optional_phone(raw: Option<&str>) -> Result<Option<&str>, AppError> {
    match raw.map(str::trim) {
        None | Some("") => Ok(None),
        Some(value) => required_phone(value).map(Some),
    }
}

/// Trims a freeform admin note; absent or blank is `None`, over-length is an error.
pub fn optional_note<'a>(raw: Option<&'a str>, field: &str) -> Result<Option<&'a str>, AppError> {
    match raw.map(str::trim) {
        None | Some("") => Ok(None),
        Some(value) if value.chars().count() > MAX_ADMIN_NOTE_LENGTH => Err(AppError::BadRequest(
            format!("{field} must be at most {MAX_ADMIN_NOTE_LENGTH} characters."),
        )),
        Some(value) => Ok(Some(value)),
    }
}

/// Postgres rejects NUL in text columns, so it is caught here as a field error, not a 500.
const NUL: char = '\0';

/// Trims and collapses every run of Unicode whitespace to a single space.
/// Stored place names go through it; feed filters must too (MH-59) so both compare equal.
pub fn normalize_place_name(raw: &str) -> String {
    raw.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Accumulates rule violations across fields so a request is answered with
/// every offending field at once. Each check returns the accepted value, or
/// `None` after recording the error.
#[derive(Debug, Default)]
pub struct FieldErrors(Vec<FieldError>);

impl FieldErrors {
    pub fn push(&mut self, field: &'static str, message: impl Into<String>) {
        self.0.push(FieldError {
            field,
            message: message.into(),
        });
    }

    /// Trims, then checks the length in characters against `length`.
    pub fn required_text(
        &mut self,
        field: &'static str,
        raw: Option<String>,
        length: RangeInclusive<usize>,
    ) -> Option<String> {
        let Some(raw) = raw else {
            self.push(field, format!("{field} is required."));
            return None;
        };
        let trimmed = raw.trim();
        if !length.contains(&trimmed.chars().count()) {
            self.push(
                field,
                format!(
                    "{field} must be between {} and {} characters.",
                    length.start(),
                    length.end()
                ),
            );
            return None;
        }
        if trimmed.contains(NUL) {
            self.push(field, format!("{field} contains an invalid character."));
            return None;
        }
        Some(trimmed.to_owned())
    }

    /// Normalizes with [`normalize_place_name`], then rejects an empty or
    /// over-long value.
    pub fn required_place_name(
        &mut self,
        field: &'static str,
        raw: Option<String>,
        max_length: usize,
    ) -> Option<String> {
        let Some(raw) = raw else {
            self.push(field, format!("{field} is required."));
            return None;
        };
        let normalized = normalize_place_name(&raw);
        if normalized.is_empty() || normalized.chars().count() > max_length {
            self.push(
                field,
                format!("{field} must be between 1 and {max_length} characters."),
            );
            return None;
        }
        if normalized.contains(NUL) {
            self.push(field, format!("{field} contains an invalid character."));
            return None;
        }
        Some(normalized)
    }

    pub fn required_int(
        &mut self,
        field: &'static str,
        raw: Option<i64>,
        range: RangeInclusive<i64>,
    ) -> Option<i64> {
        let Some(value) = raw else {
            self.push(field, format!("{field} is required."));
            return None;
        };
        self.check_int_range(field, value, range)
    }

    /// Absent is accepted as `None`; a present value must be within `range`.
    pub fn optional_int(
        &mut self,
        field: &'static str,
        raw: Option<i64>,
        range: RangeInclusive<i64>,
    ) -> Option<i64> {
        self.check_int_range(field, raw?, range)
    }

    fn check_int_range(
        &mut self,
        field: &'static str,
        value: i64,
        range: RangeInclusive<i64>,
    ) -> Option<i64> {
        if range.contains(&value) {
            return Some(value);
        }
        self.push(
            field,
            format!(
                "{field} must be between {} and {}.",
                range.start(),
                range.end()
            ),
        );
        None
    }

    /// `Err(AppError::Validation)` when at least one violation was recorded.
    pub fn finish(self) -> Result<(), AppError> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(AppError::Validation(self.0))
        }
    }
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

    #[test]
    fn optional_phone_treats_absent_and_blank_as_cleared_but_still_bounds_a_value() {
        assert_eq!(optional_phone(None).unwrap(), None);
        assert_eq!(optional_phone(Some("  ")).unwrap(), None);
        assert_eq!(
            optional_phone(Some(" +33600000000 ")).unwrap(),
            Some("+33600000000")
        );
        assert!(matches!(
            optional_phone(Some(&"0".repeat(MAX_PHONE_LENGTH + 1))),
            Err(AppError::BadRequest(_))
        ));
    }

    #[test]
    fn optional_note_treats_absent_and_blank_as_cleared() {
        assert_eq!(optional_note(None, "admin_note").unwrap(), None);
        assert_eq!(optional_note(Some("   "), "admin_note").unwrap(), None);
        assert_eq!(
            optional_note(Some(" Blurry ID photo "), "admin_note").unwrap(),
            Some("Blurry ID photo")
        );
    }

    #[test]
    fn optional_note_accepts_the_upper_bound_and_rejects_over_length() {
        let at_bound = repeat(MAX_ADMIN_NOTE_LENGTH);
        assert_eq!(
            optional_note(Some(&at_bound), "admin_note").unwrap(),
            Some(at_bound.as_str())
        );
        assert!(matches!(
            optional_note(Some(&repeat(MAX_ADMIN_NOTE_LENGTH + 1)), "admin_note"),
            Err(AppError::BadRequest(_))
        ));
    }

    #[test]
    fn normalize_place_name_trims_and_collapses_unicode_whitespace() {
        assert_eq!(normalize_place_name("  Dakar "), "Dakar");
        assert_eq!(normalize_place_name("Plateau   Nord"), "Plateau Nord");
        assert_eq!(
            normalize_place_name("Plateau\t\nNord\u{00A0}Est"),
            "Plateau Nord Est"
        );
        assert_eq!(normalize_place_name(" \t\u{00A0} "), "");
    }

    fn violated_fields(errors: FieldErrors) -> Vec<&'static str> {
        match errors.finish() {
            Err(AppError::Validation(entries)) => {
                entries.into_iter().map(|entry| entry.field).collect()
            }
            other => panic!("expected a validation error, got {other:?}"),
        }
    }

    #[test]
    fn required_text_trims_and_checks_both_bounds() {
        let mut errors = FieldErrors::default();
        assert_eq!(
            errors.required_text("title", Some("  Hello  ".into()), 5..=10),
            Some("Hello".to_owned())
        );
        assert!(errors
            .required_text("title", Some("abcd".into()), 5..=10)
            .is_none());
        assert!(errors
            .required_text("title", Some(repeat(11)), 5..=10)
            .is_none());
        assert!(errors
            .required_text("title", Some(repeat(10)), 5..=10)
            .is_some());
        assert!(errors.required_text("title", None, 5..=10).is_none());
        assert_eq!(violated_fields(errors).len(), 3);
    }

    #[test]
    fn required_text_counts_characters_not_bytes() {
        let mut errors = FieldErrors::default();
        assert!(errors
            .required_text("title", Some(repeat(5)), 5..=5)
            .is_some());
        assert!(errors.finish().is_ok());
    }

    #[test]
    fn required_place_name_normalizes_then_bounds() {
        let mut errors = FieldErrors::default();
        assert_eq!(
            errors.required_place_name("city", Some(" Plateau \t Nord ".into()), 100),
            Some("Plateau Nord".to_owned())
        );
        assert!(errors
            .required_place_name("city", Some(repeat(100)), 100)
            .is_some());
        assert!(errors
            .required_place_name("city", Some("   ".into()), 100)
            .is_none());
        assert!(errors
            .required_place_name("city", Some(repeat(101)), 100)
            .is_none());
        assert!(errors.required_place_name("city", None, 100).is_none());
        assert_eq!(violated_fields(errors).len(), 3);
    }

    #[test]
    fn text_fields_reject_the_nul_character() {
        let mut errors = FieldErrors::default();
        assert!(errors
            .required_text("title", Some("abcde\0".into()), 5..=10)
            .is_none());
        assert!(errors
            .required_place_name("city", Some("Da\0kar".into()), 100)
            .is_none());
        assert_eq!(violated_fields(errors), vec!["title", "city"]);
    }

    #[test]
    fn required_int_accepts_the_bounds_and_rejects_one_past_them() {
        let mut errors = FieldErrors::default();
        assert_eq!(errors.required_int("price", Some(1), 1..=10), Some(1));
        assert_eq!(errors.required_int("price", Some(10), 1..=10), Some(10));
        assert!(errors.required_int("price", Some(0), 1..=10).is_none());
        assert!(errors.required_int("price", Some(11), 1..=10).is_none());
        assert!(errors.required_int("price", None, 1..=10).is_none());
        assert_eq!(violated_fields(errors).len(), 3);
    }

    #[test]
    fn optional_int_accepts_absent_and_bounds_a_present_value() {
        let mut errors = FieldErrors::default();
        assert_eq!(errors.optional_int("rooms", None, 0..=100), None);
        assert_eq!(errors.optional_int("rooms", Some(0), 0..=100), Some(0));
        assert_eq!(errors.optional_int("rooms", Some(100), 0..=100), Some(100));
        assert!(errors.finish().is_ok());

        let mut errors = FieldErrors::default();
        assert!(errors.optional_int("rooms", Some(-1), 0..=100).is_none());
        assert!(errors.optional_int("rooms", Some(101), 0..=100).is_none());
        assert_eq!(violated_fields(errors), vec!["rooms", "rooms"]);
    }

    #[test]
    fn finish_reports_one_entry_per_recorded_violation() {
        assert!(FieldErrors::default().finish().is_ok());

        let mut errors = FieldErrors::default();
        errors.push("type", "type is invalid.");
        errors.required_int("price", None, 1..=10);
        errors.required_text("title", None, 5..=10);
        assert_eq!(violated_fields(errors), vec!["type", "price", "title"]);
    }
}
