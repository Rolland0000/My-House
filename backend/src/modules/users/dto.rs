use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::shared::rbac::Role;

use super::model::UserRow;

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
}
