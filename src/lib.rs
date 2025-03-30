pub mod api;
pub mod blockchain;
pub mod config;
pub mod database;
pub mod error;
pub mod funding;
pub mod models;
pub mod services;

use once_cell::sync::Lazy;
use std::sync::Mutex;
use log::info;

// Global application version
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

// Application name for logging
pub static APP_NAME: &str = "CorneaDeSci Platform";

// Initialize the platform with welcome message
pub fn init() {
    info!("Initializing {} v{}", APP_NAME, VERSION);
    info!("Decentralized Corneal Health Research Platform");
} 