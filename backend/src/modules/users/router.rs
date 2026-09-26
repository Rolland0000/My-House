use axum::extract::DefaultBodyLimit;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::app_state::AppState;
use crate::modules::users::handler;
use crate::shared::file_validation::SINGLE_IMAGE_BODY_LIMIT_BYTES;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        // One `routes!` call per path: each call emits a single OpenAPI path
        // item, so splitting these three methods would drop two from the schema.
        .routes(routes!(
            handler::get_me,
            handler::update_me,
            handler::delete_me
        )) // authenticated
        .merge(avatar_router())
}

/// Kept apart so the raised body limit wraps the avatar upload only.
fn avatar_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(handler::upload_avatar)) // authenticated
        .layer(DefaultBodyLimit::max(SINGLE_IMAGE_BODY_LIMIT_BYTES))
}
