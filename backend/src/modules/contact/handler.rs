use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::extractors::AuthUser;
use crate::shared::rbac::Role;

use super::dto::ContactResponse;
use super::service;

const CONTACT_ROLES: &[Role] = &[Role::Seeker, Role::Owner];

/// Phone numbers of a listing's owner, for a signed-in seeker or owner.
///
/// The listing must be publicly visible, available or not, and its owner
/// active. Otherwise the 404 is identical to the one for an unknown id. Every
/// call counts toward a per-user cap, 404s included.
#[utoipa::path(
    get,
    path = "/listings/{id}/contact",
    tag = "contact",
    params(("id" = Uuid, Path, description = "Listing id")),
    responses(
        (status = 200, description = "Owner phone numbers", body = ContactResponse),
        (status = 401, description = "Missing, malformed, invalid or expired access token, or suspended account"),
        (status = 403, description = "Caller is an admin"),
        (status = 404, description = "Listing not found, not publicly visible, or its owner is inactive"),
        (status = 429, description = "Too many reveals by the caller within the window"),
    )
)]
pub async fn get_owner_contact(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ContactResponse>, AppError> {
    user.require_role(CONTACT_ROLES)?;

    let data = service::reveal_owner_contact(
        state.db(),
        state.cache().contact_reveal_rate_limit().as_ref(),
        state.config().contact_reveal_max_per_window,
        user.user_id,
        id,
    )
    .await?;
    Ok(Json(ContactResponse { data }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::rbac::require_role;

    #[test]
    fn seekers_and_owners_pass_and_admins_are_forbidden() {
        assert!(require_role(Role::Seeker, CONTACT_ROLES).is_ok());
        assert!(require_role(Role::Owner, CONTACT_ROLES).is_ok());
        assert!(matches!(
            require_role(Role::Admin, CONTACT_ROLES),
            Err(AppError::Forbidden)
        ));
    }
}
