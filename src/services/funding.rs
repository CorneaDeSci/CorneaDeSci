use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    Funding, FundingResponse, CreateFundingRequest,
    FundingStatus, FundingTransaction
};

pub struct FundingService<'a> {
    pool: &'a PgPool,
}

impl<'a> FundingService<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
    
    pub async fn fund_research(&self, user_id: Uuid, research_id: Uuid, req: CreateFundingRequest) -> Result<FundingResponse, AppError> {
        // Check if research exists
        let research = sqlx::query!(
            "SELECT * FROM researches WHERE id = $1",
            research_id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("Research with ID {} not found", research_id)))?;
        
        // Create funding record
        let now = Utc::now();
        let tx_hash = req.transaction_hash.clone();
        
        let funding = sqlx::query_as!(
            Funding,
            r#"
            INSERT INTO fundings (
                id, user_id, research_id, amount, transaction_hash, 
                status, message, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
            Uuid::new_v4(),
            user_id,
            research_id,
            req.amount,
            req.transaction_hash,
            FundingStatus::Pending as _,
            req.message,
            now,
            now
        )
        .fetch_one(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        // Register transaction for verification
        sqlx::query!(
            r#"
            INSERT INTO funding_transactions (
                id, funding_id, transaction_hash, status, verified_at, created_at
            )
            VALUES ($1, $2, $3, $4, NULL, $5)
            "#,
            Uuid::new_v4(),
            funding.id,
            tx_hash,
            "PENDING",
            now
        )
        .execute(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(FundingResponse::from(funding))
    }
    
    pub async fn get_research_fundings(&self, research_id: Uuid) -> Result<Vec<FundingResponse>, AppError> {
        // Check if research exists
        let research = sqlx::query!(
            "SELECT * FROM researches WHERE id = $1",
            research_id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("Research with ID {} not found", research_id)))?;
        
        // Get fundings
        let fundings = sqlx::query_as!(
            Funding,
            r#"
            SELECT * FROM fundings 
            WHERE research_id = $1 AND status = $2
            ORDER BY created_at DESC
            "#,
            research_id,
            FundingStatus::Completed as _
        )
        .fetch_all(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(fundings.into_iter().map(FundingResponse::from).collect())
    }
    
    pub async fn get_user_fundings(&self, user_id: Uuid) -> Result<Vec<FundingResponse>, AppError> {
        let fundings = sqlx::query_as!(
            Funding,
            r#"
            SELECT f.* FROM fundings f
            JOIN researches r ON f.research_id = r.id
            WHERE f.user_id = $1
            ORDER BY f.created_at DESC
            "#,
            user_id
        )
        .fetch_all(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(fundings.into_iter().map(FundingResponse::from).collect())
    }
    
    pub async fn verify_transaction(&self, tx_hash: &str) -> Result<(), AppError> {
        // Get transaction
        let tx = sqlx::query_as!(
            FundingTransaction,
            r#"
            SELECT * FROM funding_transactions
            WHERE transaction_hash = $1 AND status = 'PENDING'
            "#,
            tx_hash
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("Transaction with hash {} not found or already verified", tx_hash)))?;
        
        // Update transaction status
        let now = Utc::now();
        sqlx::query!(
            r#"
            UPDATE funding_transactions
            SET status = 'VERIFIED', verified_at = $1
            WHERE id = $2
            "#,
            now,
            tx.id
        )
        .execute(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        // Update funding status
        sqlx::query!(
            r#"
            UPDATE fundings
            SET status = $1, updated_at = $2
            WHERE id = $3
            "#,
            FundingStatus::Completed as _,
            now,
            tx.funding_id
        )
        .execute(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        // Update research funding
        let funding = sqlx::query_as!(
            Funding,
            "SELECT * FROM fundings WHERE id = $1",
            tx.funding_id
        )
        .fetch_one(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        sqlx::query!(
            r#"
            UPDATE researches
            SET current_funding = current_funding + $1
            WHERE id = $2
            "#,
            funding.amount,
            funding.research_id
        )
        .execute(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(())
    }
    
    pub async fn get_funding_by_id(&self, id: Uuid) -> Result<FundingResponse, AppError> {
        let funding = sqlx::query_as!(
            Funding,
            "SELECT * FROM fundings WHERE id = $1",
            id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("Funding with ID {} not found", id)))?;
        
        Ok(FundingResponse::from(funding))
    }
    
    pub async fn get_pending_transactions(&self) -> Result<Vec<FundingTransaction>, AppError> {
        let transactions = sqlx::query_as!(
            FundingTransaction,
            r#"
            SELECT * FROM funding_transactions
            WHERE status = 'PENDING'
            ORDER BY created_at ASC
            "#,
        )
        .fetch_all(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(transactions)
    }
    
    pub async fn get_total_funding_stats(&self) -> Result<(i64, i64), AppError> {
        let stats = sqlx::query!(
            r#"
            SELECT 
                COUNT(*) as total_fundings,
                COALESCE(SUM(amount), 0) as total_amount
            FROM fundings
            WHERE status = $1
            "#,
            FundingStatus::Completed as _
        )
        .fetch_one(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok((stats.total_fundings, stats.total_amount.unwrap_or(0)))
    }
    
    pub async fn get_research_funding_stats(&self, research_id: Uuid) -> Result<(i64, i64, i32), AppError> {
        // Verify research exists
        let research = sqlx::query!(
            "SELECT * FROM researches WHERE id = $1",
            research_id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("Research with ID {} not found", research_id)))?;
        
        let stats = sqlx::query!(
            r#"
            SELECT 
                COUNT(*) as total_fundings,
                COALESCE(SUM(amount), 0) as total_amount,
                COUNT(DISTINCT user_id) as unique_backers
            FROM fundings
            WHERE research_id = $1 AND status = $2
            "#,
            research_id,
            FundingStatus::Completed as _
        )
        .fetch_one(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok((
            stats.total_fundings,
            stats.total_amount.unwrap_or(0),
            stats.unique_backers
        ))
    }
} 