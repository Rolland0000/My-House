use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::shared::rbac::Role;

use super::model::{AdminUserRow, UserRow};

/// Same obligations as `RegisterDto`: `phone` being required is what keeps an
/// omitted field from silently erasing the stored number.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMeDto {
    pub first_name: Option<String>,
    pub last_name: String,
    pub phone: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserDto {
    pub id: Uuid,
    pub email: String,
    pub role: Role,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub created_at: String,
}

impl From<UserRow> for UserDto {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id,
            email: row.email,
            role: row.role,
            first_name: row.first_name,
            last_name: row.last_name,
            phone: row.phone,
            avatar_url: row.avatar_url,
            is_active: row.is_active,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub data: UserDto,
}

/// Documents the avatar upload body for the OpenAPI schema. Nothing
/// deserializes into it — the handler walks the multipart fields itself.
#[derive(ToSchema)]
pub struct AvatarUploadForm {
    #[schema(value_type = String, format = Binary)]
    pub file: String,
}

/// `suspended` means `is_active = false`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AccountState {
    Active,
    Suspended,
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct AdminUserListQuery {
    pub role: Option<Role>,
    pub state: Option<AccountState>,
    /// Matches part of the email, first name or last name, ignoring case and accents.
    pub q: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// One account in the admin user list. Never carries owner-request identity data.
#[derive(Debug, Serialize, ToSchema)]
pub struct AdminUserDto {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub role: Role,
    pub is_active: bool,
    pub created_at: String,
    /// Listings the user owns, drafts included.
    pub listing_count: i64,
}

impl From<AdminUserRow> for AdminUserDto {
    fn from(row: AdminUserRow) -> Self {
        Self {
            id: row.id,
            email: row.email,
            first_name: row.first_name,
            last_name: row.last_name,
            phone: row.phone,
            role: row.role,
            is_active: row.is_active,
            created_at: row.created_at,
            listing_count: row.listing_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row() -> UserRow {
        UserRow {
            id: Uuid::nil(),
            email: "user@example.com".to_string(),
            role: Role::Seeker,
            first_name: Some("Ada".to_string()),
            last_name: Some("Lovelace".to_string()),
            phone: Some("+33600000000".to_string()),
            avatar_url: None,
            is_active: true,
            created_at: "2026-08-30T10:00:00Z".to_string(),
        }
    }

    #[test]
    fn missing_phone_fails_deserialization() {
        let json = r#"{"first_name":"Ada","last_name":"Lovelace"}"#;
        assert!(serde_json::from_str::<UpdateMeDto>(json).is_err());
    }

    #[test]
    fn null_phone_fails_deserialization() {
        let json = r#"{"last_name":"Lovelace","phone":null}"#;
        assert!(serde_json::from_str::<UpdateMeDto>(json).is_err());
    }

    #[test]
    fn missing_last_name_fails_deserialization() {
        let json = r#"{"phone":"+33600000000"}"#;
        assert!(serde_json::from_str::<UpdateMeDto>(json).is_err());
    }

    #[test]
    fn absent_first_name_is_none() {
        let json = r#"{"last_name":"Lovelace","phone":"+33600000000"}"#;
        let dto: UpdateMeDto = serde_json::from_str(json).unwrap();
        assert!(dto.first_name.is_none());
    }

    /// `email` and `role` are absent from the DTO, so a body carrying them
    /// parses fine and simply has nowhere to write them.
    #[test]
    fn email_and_role_in_body_are_ignored() {
        let json = r#"{"last_name":"Lovelace","phone":"+33600000000",
                       "email":"attacker@example.com","role":"admin"}"#;
        let dto: UpdateMeDto = serde_json::from_str(json).unwrap();
        assert_eq!(dto.last_name, "Lovelace");
        assert_eq!(dto.phone, "+33600000000");
    }

    #[test]
    fn row_maps_to_dto_field_for_field() {
        let dto = UserDto::from(row());
        assert_eq!(dto.id, Uuid::nil());
        assert_eq!(dto.email, "user@example.com");
        assert_eq!(dto.role, Role::Seeker);
        assert_eq!(dto.first_name.as_deref(), Some("Ada"));
        assert_eq!(dto.last_name.as_deref(), Some("Lovelace"));
        assert_eq!(dto.phone.as_deref(), Some("+33600000000"));
        assert_eq!(dto.avatar_url, None);
        assert!(dto.is_active);
        assert_eq!(dto.created_at, "2026-08-30T10:00:00Z");
    }

    #[test]
    fn response_envelope_serializes_the_profile_fields() {
        let json = serde_json::to_value(UserResponse { data: row().into() }).unwrap();
        let data = &json["data"];
        assert_eq!(data["email"], "user@example.com");
        assert_eq!(data["role"], "seeker");
        assert_eq!(data["is_active"], true);
        assert_eq!(data["created_at"], "2026-08-30T10:00:00Z");
    }

    #[test]
    fn account_state_accepts_only_its_two_lowercase_values() {
        let parse = |raw| serde_json::from_value::<AccountState>(serde_json::json!(raw));
        assert_eq!(parse("active").unwrap(), AccountState::Active);
        assert_eq!(parse("suspended").unwrap(), AccountState::Suspended);
        assert!(parse("inactive").is_err());
        assert!(parse("Active").is_err());
    }

    #[test]
    fn admin_user_dto_serializes_exactly_the_list_fields() {
        let dto = AdminUserDto::from(AdminUserRow {
            id: Uuid::nil(),
            email: "owner@example.com".to_string(),
            first_name: None,
            last_name: Some("Mbarga".to_string()),
            phone: Some("+237600000000".to_string()),
            role: Role::Owner,
            is_active: false,
            created_at: "2026-11-01T08:00:00Z".to_string(),
            listing_count: 3,
        });
        let json = serde_json::to_value(dto).unwrap();

        let mut keys: Vec<_> = json.as_object().unwrap().keys().cloned().collect();
        keys.sort();
        assert_eq!(
            keys,
            [
                "created_at",
                "email",
                "first_name",
                "id",
                "is_active",
                "last_name",
                "listing_count",
                "phone",
                "role"
            ]
        );
        assert_eq!(json["role"], "owner");
        assert_eq!(json["listing_count"], 3);
        assert!(json["first_name"].is_null());
    }
}
