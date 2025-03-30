use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{User, UserResponse, CreateUserRequest, UpdateUserRequest};

pub struct UserService<'a> {
    pool: &'a PgPool,
}

impl<'a> UserService<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
    
    pub async fn create_user(&self, user_req: CreateUserRequest) -> Result<User, AppError> {
        // Check if user already exists
        let existing_user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1 OR username = $2",
            user_req.email,
            user_req.username
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        if let Some(user) = existing_user {
            if user.email == user_req.email {
                return Err(AppError::Conflict(format!("Email {} is already in use", user_req.email)));
            } else {
                return Err(AppError::Conflict(format!("Username {} is already in use", user_req.username)));
            }
        }
        
        // Hash password
        let password_hash = hash(user_req.password, DEFAULT_COST)
            .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))?;
        
        // Insert new user
        let now = Utc::now();
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, username, password_hash, full_name, bio, role, wallet_address, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
            Uuid::new_v4(),
            user_req.email,
            user_req.username,
            password_hash,
            user_req.full_name,
            user_req.bio,
            user_req.role as _,
            user_req.wallet_address,
            now,
            now
        )
        .fetch_one(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(user)
    }
    
    pub async fn authenticate_user(&self, email: &str, password: &str) -> Result<User, AppError> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            email
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::Auth("Invalid email or password".to_string()))?;
        
        let valid = verify(password, &user.password_hash)
            .map_err(|e| AppError::Internal(format!("Failed to verify password: {}", e)))?;
            
        if !valid {
            return Err(AppError::Auth("Invalid email or password".to_string()));
        }
        
        Ok(user)
    }
    
    pub async fn get_user_by_id(&self, id: Uuid) -> Result<UserResponse, AppError> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", id)))?;
        
        Ok(UserResponse::from(user))
    }
    
    pub async fn update_user(&self, id: Uuid, update_req: UpdateUserRequest) -> Result<UserResponse, AppError> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", id)))?;
        
        // Update user fields
        let email = update_req.email.unwrap_or_else(|| user.email.clone());
        let username = update_req.username.unwrap_or_else(|| user.username.clone());
        let full_name = update_req.full_name.or(user.full_name);
        let bio = update_req.bio.or(user.bio);
        let wallet_address = update_req.wallet_address.or(user.wallet_address);
        let now = Utc::now();
        
        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET email = $1, username = $2, full_name = $3, bio = $4, wallet_address = $5, updated_at = $6
            WHERE id = $7
            RETURNING *
            "#,
            email,
            username,
            full_name,
            bio,
            wallet_address,
            now,
            id
        )
        .fetch_one(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(UserResponse::from(updated_user))
    }
    
    pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(self.pool)
            .await
            .map_err(AppError::Database)?;
            
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("User with ID {} not found", id)));
        }
        
        Ok(())
    }
    
    pub async fn get_users(&self) -> Result<Vec<UserResponse>, AppError> {
        let users = sqlx::query_as!(User, "SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(self.pool)
            .await
            .map_err(AppError::Database)?;
            
        Ok(users.into_iter().map(UserResponse::from).collect())
    }
} 