//! Typed readers over `std::env`, shared by every configuration group.

use std::env;

use super::config_error::ConfigError;

/// Read a required string variable. Returns `ConfigError::Missing` if absent.
pub(super) fn require(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::Missing(key.to_string()))
}

/// Read an optional variable, returning `default` if absent.
pub(super) fn optional_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Read an optional variable, treating an empty value (`KEY=` in `.env`) as absent.
pub(super) fn optional_non_empty(key: &str) -> Option<String> {
    env::var(key).ok().filter(|value| !value.is_empty())
}

/// Parse an optional variable as `bool`, falling back to `default`.
pub(super) fn optional_bool_or(key: &str, default: bool) -> Result<bool, ConfigError> {
    match env::var(key) {
        Err(_) => Ok(default),
        Ok(raw) => match raw.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            other => Err(ConfigError::Invalid {
                key: key.to_string(),
                reason: format!("expected \"true\" or \"false\", got \"{other}\""),
            }),
        },
    }
}

/// Parse a required variable as `T` using a custom parser closure.
pub(super) fn require_parsed<T, F>(key: &str, parse: F) -> Result<T, ConfigError>
where
    F: Fn(&str) -> Result<T, String>,
{
    let raw = require(key)?;
    parse(&raw).map_err(|reason| ConfigError::Invalid {
        key: key.to_string(),
        reason,
    })
}

/// Parse a required variable as `u16`.
pub(super) fn require_u16(key: &str) -> Result<u16, ConfigError> {
    require_parsed(key, |v| {
        v.parse::<u16>()
            .map_err(|_| format!("expected a port number (0–65535), got \"{v}\""))
    })
}

/// Parse a required variable as `u32`.
pub(super) fn require_u32(key: &str) -> Result<u32, ConfigError> {
    require_parsed(key, |v| {
        v.parse::<u32>()
            .map_err(|_| format!("expected an unsigned integer, got \"{v}\""))
    })
}

/// Parse a required variable as `u64`.
pub(super) fn require_u64(key: &str) -> Result<u64, ConfigError> {
    require_parsed(key, |v| {
        v.parse::<u64>()
            .map_err(|_| format!("expected an unsigned integer, got \"{v}\""))
    })
}

/// Parse a required variable as a strictly positive `u32`.
pub(super) fn require_positive_u32(key: &str) -> Result<u32, ConfigError> {
    reject_zero(key, require_u32(key)?)
}

/// Parse a required variable as a strictly positive `u64`.
pub(super) fn require_positive_u64(key: &str) -> Result<u64, ConfigError> {
    reject_zero(key, require_u64(key)?)
}

fn reject_zero<T: PartialEq + Default>(key: &str, value: T) -> Result<T, ConfigError> {
    if value == T::default() {
        return Err(ConfigError::Invalid {
            key: key.to_string(),
            reason: "must be greater than 0".to_string(),
        });
    }
    Ok(value)
}

/// Parse an optional variable as `u16`, falling back to `default`.
pub(super) fn optional_u16_or(key: &str, default: u16) -> Result<u16, ConfigError> {
    match env::var(key) {
        Err(_) => Ok(default),
        Ok(raw) => raw.parse::<u16>().map_err(|_| ConfigError::Invalid {
            key: key.to_string(),
            reason: format!("expected a port number (0–65535), got \"{raw}\""),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::test_support::ENV_LOCK;

    const KEY: &str = "ENV_READER_TEST_VALUE";

    #[test]
    fn require_reports_a_missing_variable() {
        let _guard = ENV_LOCK.lock().unwrap();
        env::remove_var(KEY);
        assert!(matches!(require(KEY), Err(ConfigError::Missing(key)) if key == KEY));
    }

    #[test]
    fn optional_or_falls_back_only_when_absent() {
        let _guard = ENV_LOCK.lock().unwrap();
        env::remove_var(KEY);
        assert_eq!(optional_or(KEY, "fallback"), "fallback");
        env::set_var(KEY, "");
        assert_eq!(optional_or(KEY, "fallback"), "");
    }

    #[test]
    fn optional_non_empty_treats_an_empty_value_as_absent() {
        let _guard = ENV_LOCK.lock().unwrap();
        env::set_var(KEY, "");
        assert_eq!(optional_non_empty(KEY), None);
        env::set_var(KEY, "value");
        assert_eq!(optional_non_empty(KEY).as_deref(), Some("value"));
    }

    #[test]
    fn optional_bool_or_accepts_only_true_and_false() {
        let _guard = ENV_LOCK.lock().unwrap();
        env::remove_var(KEY);
        assert!(optional_bool_or(KEY, true).unwrap());
        env::set_var(KEY, "false");
        assert!(!optional_bool_or(KEY, true).unwrap());
        env::set_var(KEY, "yes");
        assert!(matches!(
            optional_bool_or(KEY, false),
            Err(ConfigError::Invalid { key, .. }) if key == KEY
        ));
    }

    #[test]
    fn integer_readers_reject_non_numeric_and_overflowing_values() {
        let _guard = ENV_LOCK.lock().unwrap();
        env::set_var(KEY, "abc");
        assert!(require_u16(KEY).is_err());
        assert!(require_u32(KEY).is_err());
        assert!(require_u64(KEY).is_err());
        env::set_var(KEY, "65536");
        assert!(require_u16(KEY).is_err());
        assert_eq!(require_u32(KEY).unwrap(), 65536);
    }

    #[test]
    fn positive_readers_reject_zero() {
        let _guard = ENV_LOCK.lock().unwrap();
        env::set_var(KEY, "0");
        assert!(matches!(
            require_positive_u32(KEY),
            Err(ConfigError::Invalid { key, .. }) if key == KEY
        ));
        assert!(require_positive_u64(KEY).is_err());
        env::set_var(KEY, "7");
        assert_eq!(require_positive_u32(KEY).unwrap(), 7);
        assert_eq!(require_positive_u64(KEY).unwrap(), 7);
    }

    #[test]
    fn optional_u16_or_uses_the_default_when_absent() {
        let _guard = ENV_LOCK.lock().unwrap();
        env::remove_var(KEY);
        assert_eq!(optional_u16_or(KEY, 3000).unwrap(), 3000);
        env::set_var(KEY, "8080");
        assert_eq!(optional_u16_or(KEY, 3000).unwrap(), 8080);
        env::set_var(KEY, "nope");
        assert!(optional_u16_or(KEY, 3000).is_err());
    }
}
