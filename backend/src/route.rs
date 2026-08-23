use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

use crate::api_doc::ApiDoc;
use crate::app_state::AppState;
use crate::config::AppEnv;
use crate::infra::health;
use crate::middleware::logging::request_id;
use crate::modules::{auth, listings};

// ─────────────────────────────────────────────────────────────────────────────
// Sub-routers by role
//
// Each sub-router is built in isolation so that role-scoped middleware
// (auth guards, RBAC checks) can be added as a `.layer()` on the sub-router
// *before* it is merged into the global router — without polluting the other
// role groups.
//
// Layering order reminder (Axum / Tower):
//   router.layer(middleware)  →  middleware wraps the *entire* sub-router.
//   Add auth check as a layer here; add tracing/logging at the global level.
// ─────────────────────────────────────────────────────────────────────────────

/// Routes accessible without authentication, mounted under the global
/// `/api/v1` prefix (see [`merged_router`]) — everything except `/health`,
/// which lives outside it per `TECHNICAL_SPEC_MVP.md §4` ("Hors `/api/v1`").
///
/// Examples: OTP request/verify, public listing browse.
///
/// Built as an [`OpenApiRouter`] so every handler carrying a `#[utoipa::path]`
/// annotation is automatically collected into the OpenAPI schema served at
/// `/api/docs/openapi.json` — no manual schema maintenance required.
fn public_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(listings::handler::list))
        .routes(routes!(listings::handler::get_by_id))
        .routes(routes!(auth::handler::otp_request))
        .routes(routes!(auth::handler::otp_verify))
        .routes(routes!(auth::handler::refresh))
}

/// Routes requiring a valid session (any authenticated user — seeker by default).
///
/// Protected by: `AuthUser` extractor (re-validates `is_active` on every request).
/// Examples: profile read/update, saved searches, logout.
fn seeker_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(auth::handler::logout))
    // TODO EP-02: .routes(routes!(users::me))
    // TODO EP-02: .routes(routes!(users::update_me))
    // TODO EP-02: .routes(routes!(users::request_owner_upgrade))
}

/// Routes restricted to validated owners.
///
/// Protected by: `AuthUser` extractor + `require_role(Role::Owner)` layer.
/// Examples: create/edit/delete own listings, manage media uploads.
fn owner_router() -> Router<AppState> {
    Router::new()
    // TODO EP-03: .route("/api/v1/listings",         post(listings::create))
    // TODO EP-03: .route("/api/v1/listings/:id",     patch(listings::update))
    // TODO EP-03: .route("/api/v1/listings/:id",     delete(listings::delete))
    // TODO EP-04: .route("/api/v1/media",            post(media::upload))
    // Layer added here before merge:
    // .layer(middleware::from_fn_with_state(state, require_owner))
}

/// Routes restricted to platform administrators.
///
/// Protected by: `AuthUser` extractor + `require_role(Role::Admin)` layer.
/// Examples: validate owner upgrade requests, deactivate users, audit logs.
fn admin_router() -> Router<AppState> {
    Router::new()
    // TODO EP-02: .route("/api/v1/admin/owner-requests",      get(admin::list_owner_requests))
    // TODO EP-02: .route("/api/v1/admin/owner-requests/:id",  patch(admin::review_owner_request))
    // TODO EP-02: .route("/api/v1/admin/users/:id/deactivate", patch(admin::deactivate_user))
    // Layer added here before merge:
    // .layer(middleware::from_fn_with_state(state, require_admin))
}

// ─────────────────────────────────────────────────────────────────────────────
// Root router
// ─────────────────────────────────────────────────────────────────────────────

/// Merges all sub-routers and returns both the assembled router and the
/// collected OpenAPI schema.
///
/// Every role-scoped sub-router is nested under the global `/api/v1` prefix
/// (`TECHNICAL_SPEC_MVP.md §4`) — `.nest()` on an [`OpenApiRouter`] prepends
/// the prefix to both the live routes and the collected OpenAPI paths, so
/// handlers keep declaring bare paths (e.g. `/listings`) in `#[utoipa::path]`.
/// `/health` is the sole exception, mounted outside the prefix so it matches
/// the dedicated `location /health` block in the prod nginx config.
///
/// Merge order does not affect routing precedence in Axum (routes are matched
/// by specificity, not insertion order), but keep it consistent for readability:
/// public → seeker → owner → admin.
///
/// Building this graph never touches `AppState` — only `.with_state()` at the
/// call site does — so [`openapi_spec`] can reuse it to produce the schema
/// without a database or a running server.
fn merged_router() -> (Router<AppState>, utoipa::openapi::OpenApi) {
    let api_v1 = OpenApiRouter::new()
        .merge(public_router())
        .merge(seeker_router())
        .merge(OpenApiRouter::from(owner_router()))
        .merge(OpenApiRouter::from(admin_router()));

    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(health::check))
        .routes(routes!(health::check_storage))
        .nest("/api/v1", api_v1)
        .split_for_parts()
}

/// Assembles all sub-routers and applies global middleware layers.
///
/// Global layers (tracing, CORS, request-id, …) are applied here via
/// `.layer()` on the final router so that they run for every request
/// regardless of role.
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

    router
        .layer(axum::middleware::from_fn(request_id))
        // TODO EP-01: .layer(CorsLayer::permissive())   ← tighten in prod
        .with_state(state)
}

/// Returns the OpenAPI schema without booting a database or HTTP server.
///
/// Backs `src/bin/gen_openapi.rs`, which CI uses to detect drift between
/// `docs/openapi.json` and the router's route annotations.
pub fn openapi_spec() -> utoipa::openapi::OpenApi {
    merged_router().1
}
