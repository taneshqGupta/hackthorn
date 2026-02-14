use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    #[serde(rename = "student")]
    Student,
    #[serde(rename = "faculty")]
    Faculty,
    #[serde(rename = "authority")]
    Authority,
    #[serde(rename = "admin")]
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "suspended")]
    Suspended,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub google_id: String,
    pub role: UserRole,
    pub status: UserStatus,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: Option<String>,
    pub roll_number: Option<String>,
    pub batch_year: Option<i32>,
    pub program: Option<String>,
    pub department: Option<String>,
    pub employee_id: Option<String>,
    pub designation: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: Option<String>,
    pub department: Option<String>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            role: user.role,
            first_name: user.first_name,
            last_name: user.last_name,
            profile_picture: user.profile_picture,
            department: user.department,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: Option<String>,
}
