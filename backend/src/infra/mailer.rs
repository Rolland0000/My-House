//! `Mailer` — single outbound SMTP transport for every later trigger point
//! (OTP codes, owner-request notifications). No business trigger is wired
//! here (MH-28); callers are added in later tickets (cf. TECHNICAL_SPEC_MVP.md §3bis.1).
//!
//! Best-effort policy is a hard constraint, not an implementation detail:
//! [`Mailer::send`] returns `()`, never `Result`, so a caller cannot
//! accidentally propagate an SMTP failure into a request-failing `?`. Send
//! failures are logged via `tracing::error!` and otherwise swallowed.

use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::transport::smtp::AsyncSmtpTransport;
use lettre::{Address, AsyncTransport};
use lettre::{Message, Tokio1Executor};

use crate::config::{AppConfig, SmtpSecurity};

/// Errors that can occur while building the SMTP transport at startup.
///
/// Distinct from a send-time failure: an invalid `SMTP_HOST`/`SMTP_FROM`
/// must fail fast during boot (AppConfig-style), not silently at first send.
#[derive(Debug, thiserror::Error)]
pub enum MailerError {
    #[error("invalid SMTP_FROM address \"{0}\": {1}")]
    InvalidFrom(String, lettre::address::AddressError),
    #[error("cannot set up TLS for the SMTP connection: {0}")]
    TlsSetup(lettre::transport::smtp::Error),
}

/// Wraps an async SMTP client. Constructed once at boot and shared via
/// `AppState` behind an `Arc`, mirroring `infra::db::connect_db` and
/// `infra::storage::build_storage_provider`.
pub struct Mailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
}

// `AsyncSmtpTransport` has no `Debug` impl (it wraps a live connection pool),
// so this can't be derived. Only `from` is meaningful to display.
impl std::fmt::Debug for Mailer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mailer")
            .field("from", &self.from)
            .finish_non_exhaustive()
    }
}

impl Mailer {
    /// Builds the SMTP transport from the `SMTP_*` variables: plain SMTP to any relay,
    /// authenticated when credentials are set, encrypted according to `SMTP_SECURITY`.
    ///
    /// `builder_dangerous` is used instead of `relay()` / `starttls_relay()`, which
    /// each hard-wire one TLS mode; here both the mode and the port come from config.
    pub fn new(config: &AppConfig) -> Result<Self, MailerError> {
        let from_address: Address = config
            .smtp_from
            .parse()
            .map_err(|error| MailerError::InvalidFrom(config.smtp_from.clone(), error))?;
        let from = Mailbox::new(None, from_address);

        let mut builder =
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.smtp_host)
                .port(config.smtp_port)
                .tls(tls_mode(config.smtp_security, &config.smtp_host)?);
        if let Some(credentials) = credentials(config) {
            builder = builder.credentials(credentials);
        }

        Ok(Self {
            transport: builder.build(),
            from,
        })
    }

    /// Sends an email. Fire-and-forget: an SMTP failure is logged and never
    /// returned, so it can never fail the calling request.
    pub async fn send(&self, to: Address, subject: &str, body: String) {
        let message = Message::builder()
            .from(self.from.clone())
            .to(Mailbox::new(None, to.clone()))
            .subject(subject)
            .body(body);

        let message = match message {
            Ok(message) => message,
            Err(error) => {
                tracing::error!(error = %error, to = %to, "mailer: failed to build message");
                return;
            }
        };

        if let Err(error) = self.transport.send(message).await {
            tracing::error!(error = %error, to = %to, "mailer: SMTP send failed");
        }
    }
}

/// Maps `SMTP_SECURITY` to lettre's TLS mode. The relay's certificate is checked
/// against `host` when connecting, not here.
fn tls_mode(security: SmtpSecurity, host: &str) -> Result<Tls, MailerError> {
    let parameters = || TlsParameters::new(host.to_string()).map_err(MailerError::TlsSetup);
    Ok(match security {
        SmtpSecurity::None => Tls::None,
        SmtpSecurity::StartTls => Tls::Required(parameters()?),
        SmtpSecurity::Tls => Tls::Wrapper(parameters()?),
    })
}

/// Relay login, or `None` to send unauthenticated.
fn credentials(config: &AppConfig) -> Option<Credentials> {
    config.smtp_credentials.as_ref().map(|credentials| {
        Credentials::new(credentials.username.clone(), credentials.password.clone())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppEnv, SmtpCredentials, StorageProvider};

    fn test_config() -> AppConfig {
        AppConfig {
            app_port: 3000,
            app_env: AppEnv::Development,
            database_url: "postgresql://x:x@localhost/x".to_string(),
            jwt_secret: "a-super-secret-key-that-is-at-least-32-bytes-long!".to_string(),
            jwt_access_ttl_seconds: 900,
            jwt_refresh_ttl_days: 30,
            otp_ttl_seconds: 600,
            otp_max_attempts: 3,
            otp_rate_limit_seconds: 60,
            storage_provider: StorageProvider::Local,
            local_storage_path: "/tmp".to_string(),
            public_media_base_url: "http://localhost/media".to_string(),
            cookie_domain: "localhost".to_string(),
            allowed_origins: vec!["http://localhost".to_string()],
            smtp_host: "localhost".to_string(),
            smtp_port: 1025,
            smtp_security: SmtpSecurity::None,
            smtp_credentials: None,
            smtp_from: "noreply@myhouse.app".to_string(),
            admin_notification_email: "admin@myhouse.app".to_string(),
            admin_bootstrap_on_startup: false,
            admin_bootstrap_email: None,
            rate_limit_max_requests: 100,
            rate_limit_window_seconds: 60,
            trusted_proxies: vec![],
        }
    }

    #[test]
    fn builds_successfully_with_valid_config() {
        let config = test_config();
        assert!(Mailer::new(&config).is_ok());
    }

    #[test]
    fn rejects_malformed_smtp_from() {
        let mut config = test_config();
        config.smtp_from = "not-an-email".to_string();
        let err = Mailer::new(&config).expect_err("should fail on malformed SMTP_FROM");
        assert!(matches!(err, MailerError::InvalidFrom(_, _)));
    }

    #[test]
    fn builds_with_credentials_and_each_security_mode() {
        let mut config = test_config();
        config.smtp_credentials = Some(SmtpCredentials {
            username: "mailer".to_string(),
            password: "s3cret".to_string(),
        });
        for security in [
            SmtpSecurity::None,
            SmtpSecurity::StartTls,
            SmtpSecurity::Tls,
        ] {
            config.smtp_security = security;
            assert!(Mailer::new(&config).is_ok(), "{security:?} should build");
        }
    }

    #[test]
    fn security_mode_selects_the_tls_mode() {
        let host = "smtp.example.com";
        assert!(matches!(tls_mode(SmtpSecurity::None, host), Ok(Tls::None)));
        assert!(matches!(
            tls_mode(SmtpSecurity::StartTls, host),
            Ok(Tls::Required(_))
        ));
        assert!(matches!(
            tls_mode(SmtpSecurity::Tls, host),
            Ok(Tls::Wrapper(_))
        ));
    }

    #[test]
    fn sends_unauthenticated_without_credentials() {
        assert!(credentials(&test_config()).is_none());
    }

    #[test]
    fn authenticates_with_configured_credentials() {
        let mut config = test_config();
        config.smtp_credentials = Some(SmtpCredentials {
            username: "mailer".to_string(),
            password: "s3cret".to_string(),
        });
        assert_eq!(
            credentials(&config),
            Some(Credentials::new("mailer".to_string(), "s3cret".to_string()))
        );
    }
}
