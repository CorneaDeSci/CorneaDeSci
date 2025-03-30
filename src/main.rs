use actix_web::{web, App, HttpServer};
use cornea::api::{funding_routes, research_routes, user_routes, blockchain_routes};
use cornea::config::AppConfig;
use cornea::database::Database;
use dotenv::dotenv;
use log::info;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();
    
    // Load configuration
    let config = AppConfig::from_env();
    info!("Starting Cornea platform with configuration: {:?}", config);
    
    // Initialize database connection
    let database = Database::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");
    let db = Arc::new(database);
    
    info!("Starting HTTP server at {}:{}", config.host, config.port);
    
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(user_routes::configure)
            .configure(research_routes::configure)
            .configure(funding_routes::configure)
            .configure(blockchain_routes::configure)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
} 