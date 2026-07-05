use axum::Router;

use crate::app_state::AppState;

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
fn public_router() -> Router<AppState> {
    Router::new()
    // TODO EP-02: .route("/api/v1/auth/otp/request", post(auth::request_otp))
    // TODO EP-02: .route("/api/v1/auth/otp/verify", post(auth::verify_otp))
    // TODO EP-03: .route("/api/v1/listings",         get(listings::list_public))
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
pub fn build_router(state: AppState) -> Router {
    let public  = public_router();
    let seeker  = seeker_router();
    let owner   = owner_router();
    let admin   = admin_router();

    Router::new()
        .merge(public)
        .merge(seeker)
        .merge(owner)
        .merge(admin)
        // TODO EP-01: .layer(TraceLayer::new_for_http())
        // TODO EP-01: .layer(CorsLayer::permissive())   ← tighten in prod
        .with_state(state)
}
