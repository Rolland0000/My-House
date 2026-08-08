use utoipa::OpenApi;

/// Root OpenAPI document metadata.
///
/// Carries only global metadata (info, tags) — paths and schemas are
/// collected automatically from `#[utoipa::path]`-annotated handlers via
/// `OpenApiRouter::with_openapi(ApiDoc::openapi())` + `routes!(...)` in
/// `route.rs`. Do not add `paths(...)` or `components(schemas(...))` here:
/// that would reintroduce the manual book keeping this pipeline exists to
/// avoid, and risks drifting from the actual router.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "MyHouse API",
        description = "Real estate rental matching platform (owners ↔ seekers)."
    ),
    tags(
        (name = "health", description = "Service health check"),
        (name = "listings", description = "Public read-only listings feed"),
    )
)]
pub struct ApiDoc;
