use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Blockchain interaction error: {0}")]
    Blockchain(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Auth(_) => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "unauthorized".to_string(),
                    message: self.to_string(),
                })
            }
            AppError::NotFound(_) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: "not_found".to_string(),
                    message: self.to_string(),
                })
            }
            AppError::Database(_) | AppError::Internal(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "internal_server_error".to_string(),
                    message: self.to_string(),
                })
            }
            AppError::Validation(_) | AppError::BadRequest(_) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "bad_request".to_string(),
                    message: self.to_string(),
                })
            }
            AppError::Conflict(_) => {
                HttpResponse::Conflict().json(ErrorResponse {
                    error: "conflict".to_string(),
                    message: self.to_string(),
                })
            }
            AppError::Blockchain(_) => {
                HttpResponse::FailedDependency().json(ErrorResponse {
                    error: "blockchain_error".to_string(),
                    message: self.to_string(),
                })
            }
        }
    }
} 