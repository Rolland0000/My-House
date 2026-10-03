use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::app_state::AppState;
use crate::modules::listings::handler;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(handler::list, handler::create)) // list is public, create is owner-only
        // get is public, update and delete are owner-only
        .routes(routes!(
            handler::get_by_id,
            handler::update,
            handler::delete
        ))
        .routes(routes!(handler::update_status)) // owner
        // Mounted under /users/me, but kept here because it reads listings.
        .routes(routes!(handler::list_mine)) // owner
}
