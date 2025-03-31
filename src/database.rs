//! Database module for CorneaDeSci
//! Handles database connections, models, and migrations

use sqlx::postgres::PgPool;
use crate::error::ServiceError;

pub mod models;

/// Initialize database connection pool
pub async fn init() -> Result<PgPool, ServiceError> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/corneadesci".to_string());

    PgPool::connect(&database_url)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))
}

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), ServiceError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))
}

/// Health check for database connection
pub async fn health_check(pool: &PgPool) -> Result<(), ServiceError> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
    Ok(())
} 