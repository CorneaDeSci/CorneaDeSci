//! Main entry point for the CorneaDeSci application
//! This module initializes the application and starts the server

use actix_web::{App, HttpServer};
use log::info;

mod api;
mod blockchain;
mod database;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting CorneaDeSci server...");

    // Initialize database connection
    database::init().await.expect("Failed to initialize database");

    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            .configure(api::config)
            .wrap(actix_cors::Cors::permissive())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 