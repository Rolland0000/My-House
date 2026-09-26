use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::app_state::AppState;
use crate::modules::listings::handler;

pub fn router() -> OpenApiRouter<AppState> {
    // TODO EP-03: owner create, update and delete.
    OpenApiRouter::new()
        .routes(routes!(handler::list)) // public
        .routes(routes!(handler::get_by_id)) // public
}
