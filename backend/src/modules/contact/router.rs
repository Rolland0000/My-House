use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::app_state::AppState;
use crate::modules::contact::handler;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(handler::get_owner_contact)) // seeker, owner
}
