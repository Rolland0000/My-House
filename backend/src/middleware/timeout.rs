//! Time bounds on a request once its headers are read (the header read
//! timeout itself is set on the connection, in `app_server`).

use std::time::Duration;

use axum::{extract::Request, middleware::Next, response::IntoResponse, response::Response};

use crate::shared::errors::AppError;

/// Longest silence allowed between two chunks of a request body. Reset on
/// every chunk, so a slow but steady mobile upload is never cut.
pub const REQUEST_BODY_IDLE_TIMEOUT: Duration = Duration::from_secs(30);

/// Total time allowed for one request, body upload included: a 10 MB owner
/// request still goes through at about 140 kbit/s, while a client trickling
/// bytes just under the idle timeout cannot hold a connection indefinitely.
const REQUEST_DEADLINE: Duration = Duration::from_secs(10 * 60);

/// Answers `408 REQUEST_TIMEOUT` in the standard error envelope when a
/// request outlives [`REQUEST_DEADLINE`].
pub async fn request_deadline(request: Request, next: Next) -> Response {
    match tokio::time::timeout(REQUEST_DEADLINE, next.run(request)).await {
        Ok(response) => response,
        Err(_) => AppError::RequestTimeout.into_response(),
    }
}
