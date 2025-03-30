use std::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub blockchain_network: String,
    pub contract_address: String,
    pub ipfs_gateway: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a number"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            blockchain_network: env::var("BLOCKCHAIN_NETWORK").unwrap_or_else(|_| "rinkeby".to_string()),
            contract_address: env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set"),
            ipfs_gateway: env::var("IPFS_GATEWAY").unwrap_or_else(|_| "https://ipfs.io/ipfs/".to_string()),
        }
    }
    
    pub fn development() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            database_url: "postgres://postgres:password@localhost/cornea_dev".to_string(),
            jwt_secret: "development_secret_key".to_string(),
            blockchain_network: "localhost".to_string(),
            contract_address: "0x0000000000000000000000000000000000000000".to_string(),
            ipfs_gateway: "http://localhost:8080/ipfs/".to_string(),
        }
    }
} 