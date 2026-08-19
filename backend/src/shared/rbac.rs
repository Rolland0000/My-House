use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::shared::errors::AppError;

// ─────────────────────────────────────────────────────────────────────────────
// Role
//
// Mirrors the Postgres `user_role` enum. Roles are peers, not a hierarchy:
// `owner` and `admin` do not implicitly satisfy a `seeker`-only guard, and
// vice versa. A route lists every role it accepts explicitly.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Seeker,
    Owner,
    Admin,
}

/// Checks `role` against the set of `allowed` roles, with no hierarchy applied.
pub fn require_role(role: Role, allowed: &[Role]) -> Result<(), AppError> {
    if allowed.contains(&role) {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeker_only_rejects_owner_and_admin() {
        assert!(require_role(Role::Seeker, &[Role::Seeker]).is_ok());
        assert!(require_role(Role::Owner, &[Role::Seeker]).is_err());
        assert!(require_role(Role::Admin, &[Role::Seeker]).is_err());
    }

    #[test]
    fn owner_only_rejects_seeker_and_admin() {
        assert!(require_role(Role::Owner, &[Role::Owner]).is_ok());
        assert!(require_role(Role::Seeker, &[Role::Owner]).is_err());
        assert!(require_role(Role::Admin, &[Role::Owner]).is_err());
    }

    #[test]
    fn admin_only_rejects_seeker_and_owner() {
        assert!(require_role(Role::Admin, &[Role::Admin]).is_ok());
        assert!(require_role(Role::Seeker, &[Role::Admin]).is_err());
        assert!(require_role(Role::Owner, &[Role::Admin]).is_err());
    }

    #[test]
    fn seeker_or_owner_allows_both_and_rejects_admin() {
        let allowed = [Role::Seeker, Role::Owner];
        assert!(require_role(Role::Seeker, &allowed).is_ok());
        assert!(require_role(Role::Owner, &allowed).is_ok());
        assert!(require_role(Role::Admin, &allowed).is_err());
    }

    /// Regression guard: roles are peers, not a numeric hierarchy. An
    /// owner-only guard must reject admin, not treat it as "higher access".
    #[test]
    fn no_implicit_hierarchy_admin_does_not_pass_owner_only_guard() {
        assert!(require_role(Role::Admin, &[Role::Owner]).is_err());
    }

    #[test]
    fn failed_check_maps_to_forbidden() {
        let err = require_role(Role::Seeker, &[Role::Admin]).unwrap_err();
        assert!(matches!(err, AppError::Forbidden));
    }
}
