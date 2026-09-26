use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::app_state::AppState;
use crate::modules::auth::handler;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(handler::otp_request)) // public
        .routes(routes!(handler::otp_verify)) // public
        .routes(routes!(handler::register)) // public
        .routes(routes!(handler::refresh)) // public
        .routes(routes!(handler::logout)) // authenticated
}
