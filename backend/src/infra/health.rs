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

/// Storage backend response body.
#[derive(Serialize, ToSchema)]
pub struct StorageStatus {
    status: &'static str,
}

/// Throwaway smoke check confirming `Arc<dyn StorageProvider>` (MH-27) is reachable
/// from a handler via `State<AppState>` extraction. No module has real uploads yet
/// (first real caller is EP-08/Media) — this only proves the plumbing compiles and
/// resolves through the extractor, mirroring [`check`] above.
#[utoipa::path(
    get,
    path = "/health/storage",
    tag = "health",
    responses(
        (status = 200, description = "Storage provider reachable via AppState", body = StorageStatus),
    )
)]
pub async fn check_storage(State(state): State<AppState>) -> Json<StorageStatus> {
    // Touching the trait object through State<AppState> is the point of this
    // check — no operation is actually performed on it yet.
    let _storage = state.storage();
    Json(StorageStatus { status: "ok" })
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

    /// Same pin for the storage smoke check (MH-27): confirms `GET
    /// /health/storage` is registered and documented, i.e. that `check_storage`
    /// — which extracts `State<AppState>` and calls `.storage()` — compiles
    /// and is wired into the router's route table.
    #[test]
    fn openapi_schema_reflects_storage_smoke_check() {
        let (schemas, paths, _method_router) = routes!(super::check_storage);

        let operation = paths
            .paths
            .get("/health/storage")
            .and_then(|item| item.get.as_ref())
            .expect("GET /health/storage should be registered in the OpenAPI schema");

        assert!(operation.responses.responses.contains_key("200"));
        assert!(schemas.iter().any(|(name, _)| name == "StorageStatus"));
    }
}
