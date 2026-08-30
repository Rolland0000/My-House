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
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub data: UserDto,
}
