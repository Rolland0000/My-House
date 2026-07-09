use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

// ─────────────────────────────────────────────────────────────────────────────
// Wire format
//
// All error responses share the same envelope:
//   { "error": { "code": "SNAKE_UPPER_CASE", "message": "...", "status": 4xx } }
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
    status: u16,
}

#[derive(Debug, Serialize)]
struct ErrorEnvelope {
    error: ErrorBody,
}

// ─────────────────────────────────────────────────────────────────────────────
// AppError
//
// Variants are grouped by HTTP status family.  Every variant carries enough
// context to produce a self-contained error response; no extra mapping step is
// needed in handlers — just propagate with `?`.
//
// Adding a new variant:
//   1. Add the variant with its #[error(…)] message below.
//   2. Add the matching arm in `AppError::status_and_code`.
//   3. That's it — `IntoResponse` is derived automatically.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum AppError {
    // ── 400 Bad Request ──────────────────────────────────────────────────────
    #[error("Invalid request body: {0}")]
    BadRequest(String),

    #[error("Invalid query parameter: {0}")]
    InvalidQueryParam(String),

    // ── 401 Unauthorized ─────────────────────────────────────────────────────
    #[error("Invalid or expired OTP code.")]
    OtpInvalid,

    #[error("Missing or invalid access token.")]
    Unauthorized,

    #[error("Access token expired.")]
    TokenExpired,

    // ── 403 Forbidden ────────────────────────────────────────────────────────
    #[error("Account is inactive.")]
    AccountInactive,

    #[error("Insufficient permissions to access this resource.")]
    Forbidden,

    // ── 404 Not Found ────────────────────────────────────────────────────────
    #[error("User not found.")]
    UserNotFound,

    #[error("Listing not found.")]
    ListingNotFound,

    #[error("Media not found.")]
    MediaNotFound,

    #[error("Owner request not found.")]
    OwnerRequestNotFound,

    // ── 409 Conflict ─────────────────────────────────────────────────────────
    #[error("An owner request is already pending.")]
    OwnerRequestAlreadyPending,

    // ── 422 Unprocessable Entity ─────────────────────────────────────────────
    #[error("Invalid document (unsupported format or size).")]
    InvalidDocument,

    #[error("Invalid file (unsupported format or size).")]
    InvalidFile,

    #[error("Photo quota exceeded (maximum 5 per listing).")]
    MediaQuotaExceeded,

    #[error("Cannot remove cover photo without designating another one.")]
    CoverPhotoRequired,

    // ── 429 Too Many Requests ─────────────────────────────────────────────────
    #[error("Too many OTP requests. Please wait before trying again.")]
    OtpRateLimited,

    // ── 500 Internal Server Error ─────────────────────────────────────────────
    #[error("Internal server error.")]
    Internal,

    #[error("Database error: {0}")]
    Database(String),

    #[error("Storage error: {0}")]
    Storage(String),
}

impl AppError {
    /// Maps each variant to its HTTP status code and machine-readable error code.
    ///
    /// The `code` string is what consumers receive in the JSON `error.code` field.
    /// Keep it SCREAMING_SNAKE_CASE and stable — frontend code may switch on it.
    fn status_and_code(&self) -> (StatusCode, &'static str) {
        match self {
            // 400
            Self::BadRequest(_)       => (StatusCode::BAD_REQUEST,             "BAD_REQUEST"),
            Self::InvalidQueryParam(_)=> (StatusCode::BAD_REQUEST,             "INVALID_QUERY_PARAM"),
            // 401
            Self::OtpInvalid          => (StatusCode::UNAUTHORIZED,            "OTP_INVALID"),
            Self::Unauthorized        => (StatusCode::UNAUTHORIZED,            "UNAUTHORIZED"),
            Self::TokenExpired        => (StatusCode::UNAUTHORIZED,            "TOKEN_EXPIRED"),
            // 403
            Self::AccountInactive     => (StatusCode::FORBIDDEN,               "ACCOUNT_INACTIVE"),
            Self::Forbidden           => (StatusCode::FORBIDDEN,               "FORBIDDEN"),
            // 404
            Self::UserNotFound        => (StatusCode::NOT_FOUND,               "USER_NOT_FOUND"),
            Self::ListingNotFound     => (StatusCode::NOT_FOUND,               "LISTING_NOT_FOUND"),
            Self::MediaNotFound       => (StatusCode::NOT_FOUND,               "MEDIA_NOT_FOUND"),
            Self::OwnerRequestNotFound=> (StatusCode::NOT_FOUND,               "OWNER_REQUEST_NOT_FOUND"),
            // 409
            Self::OwnerRequestAlreadyPending => (StatusCode::CONFLICT,         "OWNER_REQUEST_ALREADY_PENDING"),
            // 422
            Self::InvalidDocument     => (StatusCode::UNPROCESSABLE_ENTITY,    "INVALID_DOCUMENT"),
            Self::InvalidFile         => (StatusCode::UNPROCESSABLE_ENTITY,    "INVALID_FILE"),
            Self::MediaQuotaExceeded  => (StatusCode::UNPROCESSABLE_ENTITY,    "MEDIA_QUOTA_EXCEEDED"),
            Self::CoverPhotoRequired  => (StatusCode::UNPROCESSABLE_ENTITY,    "COVER_PHOTO_REQUIRED"),
            // 429
            Self::OtpRateLimited      => (StatusCode::TOO_MANY_REQUESTS,       "OTP_RATE_LIMITED"),
            // 500
            Self::Internal            => (StatusCode::INTERNAL_SERVER_ERROR,   "INTERNAL_SERVER_ERROR"),
            Self::Database(_)         => (StatusCode::INTERNAL_SERVER_ERROR,   "DATABASE_ERROR"),
            Self::Storage(_)          => (StatusCode::INTERNAL_SERVER_ERROR,   "STORAGE_ERROR"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = self.status_and_code();

        // Log server-side errors with context before the message is consumed.
        if status.is_server_error() {
            tracing::error!(error = %self, status = status.as_u16(), "server error");
        }

        let body = ErrorEnvelope {
            error: ErrorBody {
                code,
                message: self.to_string(),
                status: status.as_u16(),
            },
        };

        (status, Json(body)).into_response()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Convenience conversions
//
// These allow using `?` from handlers/services that return `sqlx::Error` or
// `std::io::Error` without scattering `.map_err` everywhere.
// ─────────────────────────────────────────────────────────────────────────────

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        tracing::error!(error = %error, "I/O error mapped to AppError::Storage");
        Self::Storage(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    // Helper: deserialize the error envelope from a Response body.
    async fn parse_envelope(response: Response) -> serde_json::Value {
        let body_bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("failed to read response body");
        serde_json::from_slice(&body_bytes).expect("response is not valid JSON")
    }

    #[tokio::test]
    async fn test_listing_not_found_produces_correct_envelope() {
        let response = AppError::ListingNotFound.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let json = parse_envelope(response).await;
        assert_eq!(json["error"]["code"],   "LISTING_NOT_FOUND");
        assert_eq!(json["error"]["status"], 404);
    }

    #[tokio::test]
    async fn test_otp_rate_limited_produces_429() {
        let response = AppError::OtpRateLimited.into_response();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);

        let json = parse_envelope(response).await;
        assert_eq!(json["error"]["code"], "OTP_RATE_LIMITED");
    }

    #[tokio::test]
    async fn test_bad_request_carries_detail() {
        let detail = "email manquant".to_owned();
        let response = AppError::BadRequest(detail.clone()).into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let json = parse_envelope(response).await;
        assert!(json["error"]["message"]
            .as_str()
            .unwrap_or("")
            .contains(&detail));
    }

    #[tokio::test]
    async fn test_internal_error_is_500() {
        let response = AppError::Internal.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let json = parse_envelope(response).await;
        assert_eq!(json["error"]["code"], "INTERNAL_SERVER_ERROR");
    }
}
