use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Error as SqlxError;
use log::{info, error};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self, SqlxError> {
        info!("Connecting to database at {}", database_url);
        
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
            
        info!("Successfully connected to database");
        
        // Run migrations if needed
        match sqlx::migrate!("./migrations").run(&pool).await {
            Ok(_) => info!("Database migrations applied successfully"),
            Err(e) => error!("Failed to apply database migrations: {}", e),
        }
        
        Ok(Self { pool })
    }
    
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    
    pub async fn ping(&self) -> Result<(), SqlxError> {
        sqlx::query("SELECT 1").execute(self.get_pool()).await?;
        Ok(())
    }
} 