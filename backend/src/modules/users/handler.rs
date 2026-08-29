use axum::extract::State;
use axum::Json;

use crate::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::extractors::AuthUser;

use super::dto::{UpdateMeDto, UserDto, UserResponse};
use super::service;

#[utoipa::path(
    put,
    path = "/users/me",
    tag = "users",
    request_body = UpdateMeDto,
    responses(
        (status = 200, description = "Profile updated", body = UserResponse),
        (status = 400, description = "Missing or oversized first_name, last_name or phone"),
        (status = 401, description = "Missing or invalid access token"),
    )
)]
pub async fn update_me(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<UpdateMeDto>,
) -> Result<Json<UserResponse>, AppError> {
    let row = service::update_me(state.db(), user.user_id, payload).await?;

    Ok(Json(UserResponse {
        data: UserDto::from(row),
    }))
}
