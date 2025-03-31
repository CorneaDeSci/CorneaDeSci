//! API routes module for CorneaDeSci
//! Defines all API endpoints and their handlers

use actix_web::{web, Scope};
use crate::api::{auth, research, users};

/// Configure all API routes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(auth::scope())
            .service(research::scope())
            .service(users::scope())
    );
}

/// Health check endpoint
pub async fn health_check() -> &'static str {
    "OK"
}

/// API version endpoint
pub async fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
} 