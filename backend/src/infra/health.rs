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

#[cfg(test)]
mod tests {
    use utoipa_axum::routes;

    /// Pins the `#[utoipa::path]` annotation on `check` to what the OpenAPI
    /// generation pipeline is expected to produce: `GET /health` with 200/503
    /// responses backed by the `HealthStatus` schema. Pure metadata
    /// inspection — no router, no state, no DB.
    #[test]
    fn openapi_schema_reflects_health_check() {
        let (schemas, paths, _method_router) = routes!(super::check);

        let operation = paths
            .paths
            .get("/health")
            .and_then(|item| item.get.as_ref())
            .expect("GET /health should be registered in the OpenAPI schema");

        assert!(operation.responses.responses.contains_key("200"));
        assert!(operation.responses.responses.contains_key("503"));
        assert!(schemas.iter().any(|(name, _)| name == "HealthStatus"));
    }
}
