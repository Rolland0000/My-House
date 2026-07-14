use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

use crate::api_doc::ApiDoc;
use crate::app_state::AppState;
use crate::config::AppEnv;
use crate::infra::health;

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

/// Routes accessible without authentication.
///
/// Examples: OTP request/verify, public listing browse, health check.
///
/// Built as an [`OpenApiRouter`] so every handler carrying a `#[utoipa::path]`
/// annotation is automatically collected into the OpenAPI schema served at
/// `/api/docs/openapi.json` — no manual schema maintenance required.
fn public_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(health::check))
    // TODO EP-02: .routes(routes!(auth::request_otp))
    // TODO EP-02: .routes(routes!(auth::verify_otp))
    // TODO EP-03: .routes(routes!(listings::list_public))
}

/// Routes requiring a valid session (any authenticated user — seeker by default).
///
/// Protected by: `AuthUser` extractor (re-validates `is_active` on every request).
/// Examples: profile read/update, saved searches.
fn seeker_router() -> Router<AppState> {
    Router::new()
    // TODO EP-02: .route("/api/v1/users/me",         get(users::me))
    // TODO EP-02: .route("/api/v1/users/me",         patch(users::update_me))
    // TODO EP-02: .route("/api/v1/users/owner-request", post(users::request_owner_upgrade))
    // Layer added here before merge:
    // .layer(middleware::from_fn_with_state(state, require_auth))
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

/// Assembles all sub-routers and applies global middleware layers.
///
/// Merge order does not affect routing precedence in Axum (routes are matched
/// by specificity, not insertion order), but keep it consistent for readability:
/// public → seeker → owner → admin.
///
/// Global layers (tracing, CORS, request-id, …) are applied here via
/// `.layer()` on the final router so that they run for every request
/// regardless of role.
///
/// The merged [`OpenApiRouter`] is split into the plain Axum router and the
/// generated [`utoipa::openapi::OpenApi`] schema. The Swagger UI (which also
/// serves the raw schema at `/api/docs/openapi.json`) is mounted everywhere
/// except production — the API surface must not be discoverable by anyone
/// probing a public deployment.
pub fn build_router(state: AppState) -> Router {
    let public = public_router();
    let seeker = OpenApiRouter::from(seeker_router());
    let owner = OpenApiRouter::from(owner_router());
    let admin = OpenApiRouter::from(admin_router());

    let (router, openapi) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(public)
        .merge(seeker)
        .merge(owner)
        .merge(admin)
        .split_for_parts();

    let router = if state.config().app_env == AppEnv::Production {
        router
    } else {
        router.merge(SwaggerUi::new("/api/docs").url("/api/docs/openapi.json", openapi))
    };

    router
        // TODO EP-01: .layer(TraceLayer::new_for_http())
        // TODO EP-01: .layer(CorsLayer::permissive())   ← tighten in prod
        .with_state(state)
}
