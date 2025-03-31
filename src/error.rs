//! Error handling module for CorneaDeSci
//! Defines custom error types and error handling utilities

use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use serde::Serialize;

#[derive(Debug, Display, Serialize)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Forbidden")]
    Forbidden,

    #[display(fmt = "Not Found: {}", _0)]
    NotFound(String),

    #[display(fmt = "Database Error: {}", _0)]
    DatabaseError(String),

    #[display(fmt = "Blockchain Error: {}", _0)]
    BlockchainError(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(message)
            }
            ServiceError::Unauthorized => {
                HttpResponse::Unauthorized().json("Unauthorized")
            }
            ServiceError::Forbidden => {
                HttpResponse::Forbidden().json("Forbidden")
            }
            ServiceError::NotFound(ref message) => {
                HttpResponse::NotFound().json(message)
            }
            ServiceError::DatabaseError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            ServiceError::BlockchainError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
        }
    }
}

// Helper function to create a bad request error
pub fn bad_request<T: Into<String>>(message: T) -> ServiceError {
    ServiceError::BadRequest(message.into())
}

// Helper function to create a not found error
pub fn not_found<T: Into<String>>(message: T) -> ServiceError {
    ServiceError::NotFound(message.into())
} 