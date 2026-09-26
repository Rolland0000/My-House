use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::app_state::AppState;
use crate::modules::listings::handler;

pub fn router() -> OpenApiRouter<AppState> {
    // TODO: owner update and delete.
    OpenApiRouter::new()
        .routes(routes!(handler::list, handler::create)) // list is public, create is owner-only
        .routes(routes!(handler::get_by_id)) // public
}
