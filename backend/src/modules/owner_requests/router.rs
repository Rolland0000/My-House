use axum::extract::DefaultBodyLimit;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::app_state::AppState;
use crate::modules::owner_requests::handler;
use crate::shared::file_validation::{MAX_IMAGE_SIZE_BYTES, MULTIPART_OVERHEAD_BYTES};

/// Two 5 MB images (photo modality). The 3 MB PDF modality always fits under it.
const SUBMISSION_BODY_LIMIT_BYTES: usize = MAX_IMAGE_SIZE_BYTES * 2 + MULTIPART_OVERHEAD_BYTES;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(handler::get_owner_request_status)) // authenticated
        .merge(submission_router())
}

/// Kept apart so the raised body limit wraps the submission only.
fn submission_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(handler::submit_owner_request)) // seeker, owner
        .layer(DefaultBodyLimit::max(SUBMISSION_BODY_LIMIT_BYTES))
}
