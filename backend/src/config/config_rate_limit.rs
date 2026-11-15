//! Request throttling settings: generic per-IP cap and contact-reveal cap.

use std::net::IpAddr;

use ipnet::IpNet;

use super::config_error::ConfigError;
use super::env_reader::{require_parsed, require_positive_u32, require_positive_u64};

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Generic per-IP request cap within `window_seconds` (MH-39).
    pub max_requests: u32,
    /// Fixed window length in seconds for `max_requests`.
    pub window_seconds: u64,
    /// Reverse proxies whose `X-Forwarded-For` is trusted, as IPs or CIDRs.
    /// Empty means no proxy is trusted and the header is never read.
    pub trusted_proxies: Vec<IpNet>,
    /// Distinct listings a user can reveal the owner contact of within `contact_reveal_window_seconds` (MH-71).
    pub contact_reveal_max_per_window: u32,
    /// Fixed window length in seconds for `contact_reveal_max_per_window`.
    pub contact_reveal_window_seconds: u64,
}

impl RateLimitConfig {
    pub(super) fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            max_requests: require_positive_u32("RATE_LIMIT_MAX_REQUESTS")?,
            window_seconds: require_positive_u64("RATE_LIMIT_WINDOW_SECONDS")?,
            trusted_proxies: require_trusted_proxies("TRUSTED_PROXIES")?,
            contact_reveal_max_per_window: require_positive_u32("CONTACT_REVEAL_MAX_PER_WINDOW")?,
            contact_reveal_window_seconds: require_positive_u64("CONTACT_REVEAL_WINDOW_SECONDS")?,
        })
    }
}

/// Parse a required comma-separated list of trusted proxy IPs or CIDRs
/// (e.g. `172.16.0.0/12,10.0.0.5`). An empty value trusts no proxy.
fn require_trusted_proxies(key: &str) -> Result<Vec<IpNet>, ConfigError> {
    require_parsed(key, |raw| {
        raw.split(',')
            .map(str::trim)
            .filter(|entry| !entry.is_empty())
            .map(|entry| {
                entry
                    .parse::<IpNet>()
                    .or_else(|_| entry.parse::<IpAddr>().map(IpNet::from))
                    .map_err(|_| format!("\"{entry}\" is not a valid IP address or CIDR"))
            })
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use crate::config::test_support::{set_valid_env, ENV_LOCK};

    #[test]
    fn loads_every_rate_limit_setting() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        let rate_limit = RateLimitConfig::from_env().expect("should load");
        assert_eq!(rate_limit.max_requests, 100);
        assert_eq!(rate_limit.window_seconds, 60);
        assert_eq!(rate_limit.contact_reveal_max_per_window, 20);
        assert_eq!(rate_limit.contact_reveal_window_seconds, 3600);
    }

    #[test]
    fn rejects_zero_max_requests() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("RATE_LIMIT_MAX_REQUESTS", "0");
        let err = RateLimitConfig::from_env().expect_err("should fail on a zero request cap");
        assert!(
            matches!(err, ConfigError::Invalid { key, .. } if key == "RATE_LIMIT_MAX_REQUESTS")
        );
    }

    #[test]
    fn rejects_zero_window_seconds() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("RATE_LIMIT_WINDOW_SECONDS", "0");
        let err = RateLimitConfig::from_env().expect_err("should fail on a zero window");
        assert!(
            matches!(err, ConfigError::Invalid { key, .. } if key == "RATE_LIMIT_WINDOW_SECONDS")
        );
    }

    #[test]
    fn rejects_zero_contact_reveal_max_per_window() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("CONTACT_REVEAL_MAX_PER_WINDOW", "0");
        let err = RateLimitConfig::from_env().expect_err("should fail on a zero reveal cap");
        assert!(
            matches!(err, ConfigError::Invalid { key, .. } if key == "CONTACT_REVEAL_MAX_PER_WINDOW")
        );
    }

    #[test]
    fn rejects_zero_contact_reveal_window_seconds() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("CONTACT_REVEAL_WINDOW_SECONDS", "0");
        let err = RateLimitConfig::from_env().expect_err("should fail on a zero window");
        assert!(
            matches!(err, ConfigError::Invalid { key, .. } if key == "CONTACT_REVEAL_WINDOW_SECONDS")
        );
    }

    #[test]
    fn parses_trusted_proxies_as_ips_and_cidrs() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("TRUSTED_PROXIES", "172.16.0.0/12, 10.0.0.5");
        let rate_limit = RateLimitConfig::from_env().expect("should load");
        assert_eq!(rate_limit.trusted_proxies.len(), 2);
        assert!(rate_limit.trusted_proxies[1].contains(&"10.0.0.5".parse::<IpAddr>().unwrap()));
    }

    #[test]
    fn empty_trusted_proxies_trusts_nothing() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        let rate_limit = RateLimitConfig::from_env().expect("should load");
        assert!(rate_limit.trusted_proxies.is_empty());
    }

    #[test]
    fn rejects_malformed_trusted_proxies() {
        let _guard = ENV_LOCK.lock().unwrap();
        set_valid_env();
        env::set_var("TRUSTED_PROXIES", "not-an-ip");
        let err = RateLimitConfig::from_env().expect_err("should fail on a malformed entry");
        assert!(matches!(err, ConfigError::Invalid { key, .. } if key == "TRUSTED_PROXIES"));
    }
}
