//! Request-ID tracing middleware (MH-30, Epic "(rattrapage) Logging &
//! Request Tracing").
//!
//! Every request is wrapped in a `tracing` span carrying its `request_id`,
//! so every log line emitted while handling it — from this layer down
//! through handler/service/repository — can be correlated back to a single
//! request. No log shipping/aggregation is wired here; stdout only, per the
//! epic's scope boundary.

use std::time::Instant;

use axum::{extract::Request, http::HeaderMap, middleware::Next, response::Response};
use tracing::Instrument;
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";

/// Container-orchestrator probes, hit every few seconds by the Docker
/// healthcheck. Their start/end events are demoted to DEBUG so they don't
/// drown the INFO stream; `RUST_LOG=backend_my_house=debug` brings them back.
const HEALTH_PATHS: [&str; 2] = ["/health", "/health/storage"];

/// Resolves the request id: propagates an incoming `X-Request-Id` header
/// verbatim if present, otherwise generates a new UUID v4.
fn resolve_request_id(headers: &HeaderMap) -> String {
    headers
        .get(REQUEST_ID_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
        .unwrap_or_else(|| Uuid::new_v4().to_string())
}

/// Axum middleware: attaches `request_id` (plus method/path for readability)
/// to a span covering the request's full handling.
///
/// Emits its own start/end events (rather than relying solely on downstream
/// handlers to log something) so every request produces at least one log
/// line carrying its `request_id` — a handler that never calls `tracing::*`
/// itself (e.g. `/health`) would otherwise leave no visible trace.
pub async fn request_id(request: Request, next: Next) -> Response {
    let request_id = resolve_request_id(request.headers());
    let method = request.method().clone();
    let path = request.uri().path().to_owned();

    let span = tracing::info_span!(
        "request",
        request_id = %request_id,
        %method,
        %path,
    );

    let is_health_probe = HEALTH_PATHS.contains(&path.as_str());

    async move {
        if is_health_probe {
            tracing::debug!("request started");
        } else {
            tracing::info!("request started");
        }

        let started = Instant::now();
        let response = next.run(request).await;
        let status = response.status();
        let duration_ms = started.elapsed().as_millis() as u64;

        // `tracing` macros take a static level, so the choice has to be made
        // by branching rather than by computing a `Level` value.
        if is_health_probe {
            tracing::debug!(status = %status, duration_ms, "request completed");
        } else if status.is_server_error() {
            tracing::error!(status = %status, duration_ms, "request completed");
        } else if status.is_client_error() {
            tracing::warn!(status = %status, duration_ms, "request completed");
        } else {
            tracing::info!(status = %status, duration_ms, "request completed");
        }

        response
    }
    .instrument(span)
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn generates_a_valid_uuid_when_header_is_absent() {
        let headers = HeaderMap::new();
        let id = resolve_request_id(&headers);
        assert!(Uuid::parse_str(&id).is_ok());
    }

    #[test]
    fn propagates_incoming_x_request_id_header_verbatim() {
        let mut headers = HeaderMap::new();
        headers.insert(
            REQUEST_ID_HEADER,
            HeaderValue::from_static("client-supplied-id-42"),
        );
        let id = resolve_request_id(&headers);
        assert_eq!(id, "client-supplied-id-42");
    }

    #[test]
    fn two_calls_without_header_produce_distinct_ids() {
        let headers = HeaderMap::new();
        let first = resolve_request_id(&headers);
        let second = resolve_request_id(&headers);
        assert_ne!(first, second);
    }
}
