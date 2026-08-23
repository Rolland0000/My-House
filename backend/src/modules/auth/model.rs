use uuid::Uuid;

use crate::shared::rbac::Role;

/// Joined `refresh_tokens` + `users.role` lookup backing every branch of
/// `/auth/refresh` — one round trip instead of a second query for role.
#[derive(Debug, Clone)]
pub struct RefreshTokenLookup {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role: Role,
    pub is_revoked: bool,
    pub is_expired: bool,
}
