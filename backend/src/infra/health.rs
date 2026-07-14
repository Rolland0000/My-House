use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use utoipa::ToSchema;

use crate::app_state::AppState;

/// Health check response body.
#[derive(Serialize, ToSchema)]
pub struct HealthStatus {
    status: &'static str,
}

/// Checks PostgreSQL connectivity via `SELECT 1`.
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Database reachable", body = HealthStatus),
        (status = 503, description = "Database unreachable", body = HealthStatus),
    )
)]
pub async fn check(State(state): State<AppState>) -> (StatusCode, Json<HealthStatus>) {
    match sqlx::query("SELECT 1").execute(state.db()).await {
        Ok(_) => (StatusCode::OK, Json(HealthStatus { status: "ok" })),
        Err(error) => {
            tracing::error!(error = %error, "health check: database unreachable");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(HealthStatus {
                    status: "unavailable",
                }),
            )
        }
    }
}
