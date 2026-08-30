use uuid::Uuid;

use crate::shared::rbac::Role;

pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub role: Role,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
}
