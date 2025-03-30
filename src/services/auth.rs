use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{User, UserResponse, AuthResponse};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn generate_token(&self, user_id: Uuid) -> Result<String, AppError> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::days(7)).timestamp() as usize;
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp,
            iat,
        };
        
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "development_secret".to_string());
            
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| AppError::Auth(format!("Failed to generate token: {}", e)))
    }
    
    pub fn create_auth_response(&self, user: User, token: String) -> AuthResponse {
        AuthResponse {
            token,
            user: UserResponse::from(user),
        }
    }
} 