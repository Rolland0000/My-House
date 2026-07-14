use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};

use crate::app_state::AppState;

/// Checks PostgreSQL connectivity via `SELECT 1`.
///
/// Backs the Docker healthcheck: 200 when the database is reachable,
/// 503 otherwise.
pub async fn check(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    match sqlx::query("SELECT 1").execute(state.db()).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "status": "ok" }))),
        Err(error) => {
            tracing::error!(error = %error, "health check: database unreachable");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({ "status": "unavailable" })),
            )
        }
    }
}
