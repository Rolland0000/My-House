use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::app_state::AppState;
use crate::modules::admin::handler;

pub fn router() -> OpenApiRouter<AppState> {
    // TODO EP-12: .routes(routes!(handler::deactivate_user))
    OpenApiRouter::new()
        .routes(routes!(handler::list_owner_requests)) // admin
        .routes(routes!(handler::get_owner_request)) // admin
        .routes(routes!(handler::get_owner_request_document)) // admin
        .routes(routes!(handler::review_owner_request)) // admin
}
