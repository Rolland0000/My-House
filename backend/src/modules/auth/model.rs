use uuid::Uuid;

use crate::shared::rbac::Role;

/// Input of `repository::create_account`: profile columns + first refresh token.
pub struct NewAccount<'a> {
    pub email: &'a str,
    pub first_name: Option<&'a str>,
    pub last_name: &'a str,
    pub phone: &'a str,
    pub refresh_token_hash: &'a str,
    pub refresh_ttl_days: i32,
}

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
