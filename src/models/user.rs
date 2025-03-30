use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub role: UserRole,
    pub wallet_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Researcher,
    Patient,
    Funder,
    Reviewer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub role: UserRole,
    pub wallet_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub wallet_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub role: UserRole,
    pub wallet_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            full_name: user.full_name,
            bio: user.bio,
            role: user.role,
            wallet_address: user.wallet_address,
            created_at: user.created_at,
        }
    }
} 