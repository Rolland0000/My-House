use std::time::Duration;

use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

use crate::api_doc::ApiDoc;
use crate::app_state::AppState;
use crate::config::AppEnv;
use crate::infra::health;
use crate::middleware::cors::build_cors_layer;
use crate::middleware::logging::request_id;
use crate::middleware::rate_limit::{rate_limit, RateLimitState};
use crate::modules::{admin, auth, listings, media, owner_requests, users};

/// Merges the module routers under `/api/v1` and returns the router with the
/// collected OpenAPI schema.
///
/// Each module owns its routes, public and protected. Role checks run inline
/// in the handlers, and each route in a module router is tagged with its
/// access level. `/health` stays outside the prefix to match the dedicated
/// `location /health` block in the prod nginx config.
///
/// Building this graph never touches `AppState` (only `.with_state()` does),
/// so [`openapi_spec`] can reuse it without a database or a running server.
fn merged_router() -> (Router<AppState>, utoipa::openapi::OpenApi) {
    let api_v1 = OpenApiRouter::new()
        .merge(auth::router::router())
        .merge(listings::router::router())
        .merge(users::router::router())
        .merge(owner_requests::router::router())
        .merge(media::router::router())
        .merge(admin::router::router());

    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(health::check))
        .routes(routes!(health::check_storage))
        .nest("/api/v1", api_v1)
        .split_for_parts()
}

/// Assembles all sub-routers and applies global middleware layers.
///
/// Global layers (tracing, CORS, request-id, rate-limit, …) are applied
/// here via `.layer()` on the final router so that they run for every
/// request regardless of role.
///
/// The Swagger UI (which also serves the raw schema at
/// `/api/docs/openapi.json`) is mounted everywhere except production — the
/// API surface must not be discoverable by anyone probing a public
/// deployment.
pub fn build_router(state: AppState) -> Router {
    let (router, openapi) = merged_router();

    let router = if state.config().app_env == AppEnv::Production {
        router
    } else {
        router.merge(SwaggerUi::new("/api/docs").url("/api/docs/openapi.json", openapi))
    };

    let cors = build_cors_layer(state.config());
    let rate_limit_state = RateLimitState::new(
        state.config().rate_limit_max_requests,
        Duration::from_secs(state.config().rate_limit_window_seconds),
        state.config().trusted_proxies.clone(),
        state.cache().ip_rate_limit(),
    );

    router
        .layer(axum::middleware::from_fn_with_state(
            rate_limit_state,
            rate_limit,
        ))
        .layer(axum::middleware::from_fn(request_id))
        .layer(cors)
        .with_state(state)
}

/// Returns the OpenAPI schema without booting a database or HTTP server.
///
/// Backs `src/bin/gen_openapi.rs`, which CI uses to detect drift between
/// `docs/openapi.json` and the router's route annotations.
pub fn openapi_spec() -> utoipa::openapi::OpenApi {
    merged_router().1
}
