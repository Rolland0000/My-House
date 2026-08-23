//! Rendering entry point for the five MVP email templates
//! (TECHNICAL_SPEC_MVP.md §3bis.2). No business trigger is wired here — auth
//! (EP-05) and users/owner_requests (EP-07) call [`render`] once they exist.
//!
//! Rendering failure (e.g. a template referencing a missing field) is a
//! compile-time impossibility with Askama for the fields that exist, but a
//! render call can still fail at runtime (`fmt::Write` errors); such a
//! failure surfaces as [`AppError::TemplateRender`], consistent with the
//! best-effort SMTP policy from MH-28 — the caller decides whether to log
//! and continue or fail the request, `render` itself never sends anything.

use askama::Template;
use lettre::Address;

use crate::infra::mailer::Mailer;
use crate::shared::errors::AppError;

/// Context for `otp.html` — OTP code delivery.
#[derive(Template)]
#[template(path = "otp.html")]
struct OtpTemplate<'a> {
    otp_code: &'a str,
    ttl_minutes: u64,
}

/// Context for `welcome.html` — sent after first successful OTP verification.
#[derive(Template)]
#[template(path = "welcome.html")]
struct WelcomeTemplate<'a> {
    user_name: &'a str,
}

/// Context for `owner_request_received.html` — sent to the fixed admin
/// recipient (`ADMIN_NOTIFICATION_EMAIL`), never resolved via a `users` query.
#[derive(Template)]
#[template(path = "owner_request_received.html")]
struct OwnerRequestReceivedTemplate<'a> {
    requester_name: &'a str,
    requester_email: &'a str,
}

/// Context for `owner_request_approved.html`.
#[derive(Template)]
#[template(path = "owner_request_approved.html")]
struct OwnerRequestApprovedTemplate<'a> {
    user_name: &'a str,
}

/// Context for `owner_request_rejected.html`.
#[derive(Template)]
#[template(path = "owner_request_rejected.html")]
struct OwnerRequestRejectedTemplate<'a> {
    user_name: &'a str,
    reason: &'a str,
}

/// One of the five MVP notification templates, carrying the context data it
/// needs to render. This is the "template identifier + context variables"
/// entry point required by MH-29 — matching on this enum, not a raw string
/// identifier, so an unknown template name is a compile error, not a
/// runtime `AppError`.
pub enum NotificationTemplate<'a> {
    Otp {
        otp_code: &'a str,
        ttl_minutes: u64,
    },
    Welcome {
        user_name: &'a str,
    },
    OwnerRequestReceived {
        requester_name: &'a str,
        requester_email: &'a str,
    },
    OwnerRequestApproved {
        user_name: &'a str,
    },
    OwnerRequestRejected {
        user_name: &'a str,
        reason: &'a str,
    },
}

/// Renders `template` to an HTML string.
///
/// Never sends an email — callers pass the result to `Mailer::send`
/// (`infra::mailer`) themselves, so a rendering failure and a send failure
/// stay independently observable in logs.
pub fn render(template: NotificationTemplate<'_>) -> Result<String, AppError> {
    let rendered = match template {
        NotificationTemplate::Otp {
            otp_code,
            ttl_minutes,
        } => OtpTemplate {
            otp_code,
            ttl_minutes,
        }
        .render(),
        NotificationTemplate::Welcome { user_name } => WelcomeTemplate { user_name }.render(),
        NotificationTemplate::OwnerRequestReceived {
            requester_name,
            requester_email,
        } => OwnerRequestReceivedTemplate {
            requester_name,
            requester_email,
        }
        .render(),
        NotificationTemplate::OwnerRequestApproved { user_name } => {
            OwnerRequestApprovedTemplate { user_name }.render()
        }
        NotificationTemplate::OwnerRequestRejected { user_name, reason } => {
            OwnerRequestRejectedTemplate { user_name, reason }.render()
        }
    };

    rendered.map_err(|error| {
        tracing::error!(error = %error, "notifications: template render failed");
        AppError::TemplateRender(error.to_string())
    })
}

/// Renders and sends the OTP login-code email. Best-effort like every
/// `Mailer::send` call: a malformed `to` address or a render failure is
/// logged and swallowed rather than propagated, so it can never fail the
/// request that triggered it.
pub async fn send_otp_email(mailer: &Mailer, to: &str, otp_code: &str, ttl_minutes: u64) {
    let body = match render(NotificationTemplate::Otp {
        otp_code,
        ttl_minutes,
    }) {
        Ok(body) => body,
        Err(error) => {
            tracing::error!(error = %error, to, "notifications: failed to render OTP email");
            return;
        }
    };

    let address: Address = match to.parse() {
        Ok(address) => address,
        Err(error) => {
            tracing::error!(error = %error, to, "notifications: invalid OTP recipient address");
            return;
        }
    };

    mailer.send(address, "Your MyHouse login code", body).await;
}

/// Renders and sends the welcome email, triggered once by a brand-new
/// account's first successful OTP verification. Best-effort like
/// [`send_otp_email`]: a render or delivery failure is logged and
/// swallowed, never propagated to the caller.
pub async fn send_welcome_email(mailer: &Mailer, to: &str, user_name: &str) {
    let body = match render(NotificationTemplate::Welcome { user_name }) {
        Ok(body) => body,
        Err(error) => {
            tracing::error!(error = %error, to, "notifications: failed to render welcome email");
            return;
        }
    };

    let address: Address = match to.parse() {
        Ok(address) => address,
        Err(error) => {
            tracing::error!(error = %error, to, "notifications: invalid welcome recipient address");
            return;
        }
    };

    mailer.send(address, "Bienvenue sur MyHouse", body).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_otp_template_with_sample_context() {
        let html = render(NotificationTemplate::Otp {
            otp_code: "123456",
            ttl_minutes: 10,
        })
        .expect("otp template should render");

        assert!(html.contains("123456"));
        assert!(html.contains("10"));
    }

    #[test]
    fn renders_welcome_template_with_sample_context() {
        let html = render(NotificationTemplate::Welcome {
            user_name: "Awa Diop",
        })
        .expect("welcome template should render");

        assert!(html.contains("Awa Diop"));
    }

    #[test]
    fn renders_owner_request_received_template_with_sample_context() {
        let html = render(NotificationTemplate::OwnerRequestReceived {
            requester_name: "Kouadio Yao",
            requester_email: "kouadio@example.com",
        })
        .expect("owner_request_received template should render");

        assert!(html.contains("Kouadio Yao"));
        assert!(html.contains("kouadio@example.com"));
    }

    #[test]
    fn renders_owner_request_approved_template_with_sample_context() {
        let html = render(NotificationTemplate::OwnerRequestApproved {
            user_name: "Fatou Ndiaye",
        })
        .expect("owner_request_approved template should render");

        assert!(html.contains("Fatou Ndiaye"));
    }

    #[test]
    fn renders_owner_request_rejected_template_with_sample_context() {
        let html = render(NotificationTemplate::OwnerRequestRejected {
            user_name: "Moussa Traore",
            reason: "Documents illisibles",
        })
        .expect("owner_request_rejected template should render");

        assert!(html.contains("Moussa Traore"));
        assert!(html.contains("Documents illisibles"));
    }
}
