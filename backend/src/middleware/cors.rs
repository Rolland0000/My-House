//! CORS middleware (MH-38) — cross-origin policy for the API.

use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

use crate::config::AppConfig;

/// Builds the global `CorsLayer` from `AppConfig::allowed_origins`.
///
/// `OPTIONS` is deliberately absent from `allow_methods`: `CorsLayer` answers
/// the preflight request itself and never routes it to a handler.
pub fn build_cors_layer(config: &AppConfig) -> CorsLayer {
    let origins: Vec<HeaderValue> = config
        .allowed_origins
        .iter()
        .map(|origin| {
            HeaderValue::from_str(origin).expect("origin validated by AppConfig::from_env")
        })
        .collect();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
        ])
        .allow_credentials(true)
}
