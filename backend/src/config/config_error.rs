use std::fmt;

/// Errors that can occur while loading the configuration.
#[derive(Debug)]
pub enum ConfigError {
    /// A required variable is absent from the environment.
    Missing(String),
    /// A variable is present but its value cannot be parsed.
    Invalid { key: String, reason: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Missing(key) => {
                write!(f, "Missing required environment variable: {key}")
            }
            ConfigError::Invalid { key, reason } => {
                write!(f, "Invalid value for environment variable {key}: {reason}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_display_names_the_variable() {
        let rendered = ConfigError::Missing("DATABASE_URL".to_string()).to_string();
        assert_eq!(
            rendered,
            "Missing required environment variable: DATABASE_URL"
        );
    }

    #[test]
    fn invalid_display_names_the_variable_and_the_reason() {
        let rendered = ConfigError::Invalid {
            key: "SMTP_PORT".to_string(),
            reason: "not a port".to_string(),
        }
        .to_string();
        assert_eq!(
            rendered,
            "Invalid value for environment variable SMTP_PORT: not a port"
        );
    }
}
