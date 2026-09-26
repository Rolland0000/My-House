use axum::extract::DefaultBodyLimit;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::app_state::AppState;
use crate::modules::media::handler;
use crate::shared::file_validation::SINGLE_IMAGE_BODY_LIMIT_BYTES;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().merge(upload_router())
}

/// Kept apart so the raised body limit wraps the photo upload only.
fn upload_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(handler::upload_media)) // owner
        .layer(DefaultBodyLimit::max(SINGLE_IMAGE_BODY_LIMIT_BYTES))
}
