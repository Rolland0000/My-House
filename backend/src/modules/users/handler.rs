use axum::extract::State;
use axum::Json;

use crate::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::extractors::{AppJson, AuthUser};

use super::dto::{UpdateMeDto, UserDto, UserResponse};
use super::service;

#[utoipa::path(
    get,
    path = "/users/me",
    tag = "users",
    responses(
        (status = 200, description = "Caller profile", body = UserResponse),
        (status = 401, description = "Missing or invalid access token"),
        (status = 404, description = "User not found"),
    )
)]
pub async fn get_me(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<UserResponse>, AppError> {
    let row = service::get_me(state.db(), user.user_id).await?;

    Ok(Json(UserResponse {
        data: UserDto::from(row),
    }))
}

#[utoipa::path(
    put,
    path = "/users/me",
    tag = "users",
    request_body = UpdateMeDto,
    responses(
        (status = 200, description = "Profile updated", body = UserResponse),
        (status = 400, description = "Missing or oversized first_name, last_name or phone"),
        (status = 401, description = "Missing or invalid access token"),
        (status = 404, description = "User not found"),
    )
)]
pub async fn update_me(
    State(state): State<AppState>,
    user: AuthUser,
    AppJson(payload): AppJson<UpdateMeDto>,
) -> Result<Json<UserResponse>, AppError> {
    let row = service::update_me(state.db(), user.user_id, payload).await?;

    Ok(Json(UserResponse {
        data: UserDto::from(row),
    }))
}
