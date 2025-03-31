//! CorneaDeSci Library
//! This module exports the main functionality of the CorneaDeSci platform

pub mod api;
pub mod blockchain;
pub mod database;
pub mod services;
pub mod utils;

// Re-export commonly used types
pub use database::models::*;
pub use services::auth::*;
pub use services::research::*;

/// Application configuration
pub struct Config {
    /// Database connection string
    pub database_url: String,
    /// JWT secret key
    pub jwt_secret: String,
    /// Blockchain node URL
    pub blockchain_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/corneadesci".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key".to_string()),
            blockchain_url: std::env::var("BLOCKCHAIN_URL")
                .unwrap_or_else(|_| "ws://localhost:9944".to_string()),
        }
    }
} 