//! `Mailer` — single outbound SMTP transport for every later trigger point
//! (OTP codes, owner-request notifications). No business trigger is wired
//! here (MH-28); callers are added in later tickets (cf. TECHNICAL_SPEC_MVP.md §3bis.1).
//!
//! Best-effort policy is a hard constraint, not an implementation detail:
//! [`Mailer::send`] returns `()`, never `Result`, so a caller cannot
//! accidentally propagate an SMTP failure into a request-failing `?`. Send
//! failures are logged via `tracing::error!` and otherwise swallowed.

use lettre::message::Mailbox;
use lettre::transport::smtp::AsyncSmtpTransport;
use lettre::{Address, AsyncTransport};
use lettre::{Message, Tokio1Executor};

use crate::config::AppConfig;

/// Errors that can occur while building the SMTP transport at startup.
///
/// Distinct from a send-time failure: an invalid `SMTP_HOST`/`SMTP_FROM`
/// must fail fast during boot (AppConfig-style), not silently at first send.
#[derive(Debug, thiserror::Error)]
pub enum MailerError {
    #[error("invalid SMTP_FROM address \"{0}\": {1}")]
    InvalidFrom(String, lettre::address::AddressError),
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
        f.debug_struct("Mailer").field("from", &self.from).finish_non_exhaustive()
    }
}

impl Mailer {
    /// Builds the SMTP transport from `SMTP_HOST` / `SMTP_PORT` / `SMTP_FROM`.
    ///
    /// Uses `builder_dangerous` (no TLS, no auth) rather than `relay()`
    /// because direct SMTP with no third-party transactional provider
    /// (SES/Postmark) is the *locked* MVP transport (TECHNICAL_SPEC_MVP.md
    /// §3bis.1 "Transport", §3bis.3 "Hors scope MVP") — not a stopgap. The
    /// config surface has no credentials/TLS variables by design; the local
    /// dev target (Mailhog) speaks plaintext SMTP — see `.env.example`.
    /// Adding auth/TLS is a V2 concern, not a follow-up MVP ticket.
    pub fn new(config: &AppConfig) -> Result<Self, MailerError> {
        let from_address: Address = config
            .smtp_from
            .parse()
            .map_err(|error| MailerError::InvalidFrom(config.smtp_from.clone(), error))?;
        let from = Mailbox::new(None, from_address);

        let transport = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.smtp_host)
            .port(config.smtp_port)
            .build();

        Ok(Self { transport, from })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppEnv, StorageProvider};

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
            smtp_host: "localhost".to_string(),
            smtp_port: 1025,
            smtp_from: "noreply@myhouse.app".to_string(),
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
}
