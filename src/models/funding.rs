use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "funding_status", rename_all = "lowercase")]
pub enum FundingStatus {
    Pending,
    Confirmed,
    Refunded,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "funding_type", rename_all = "lowercase")]
pub enum FundingType {
    Direct,
    Milestone,
    Grant,
    Donation,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Funding {
    pub id: Uuid,
    pub research_id: Uuid,
    pub funder_id: Uuid,
    pub amount: i64,
    pub status: FundingStatus,
    pub funding_type: FundingType,
    pub transaction_hash: Option<String>,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFundingRequest {
    pub research_id: Uuid,
    pub amount: i64,
    pub funding_type: FundingType,
    pub message: Option<String>,
    pub transaction_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFundingStatusRequest {
    pub status: FundingStatus,
    pub transaction_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FundingMilestone {
    pub id: Uuid,
    pub funding_id: Uuid,
    pub title: String,
    pub description: String,
    pub amount: i64,
    pub is_released: bool,
    pub release_conditions: String,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMilestoneRequest {
    pub funding_id: Uuid,
    pub title: String,
    pub description: String,
    pub amount: i64,
    pub release_conditions: String,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMilestoneRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub amount: Option<i64>,
    pub release_conditions: Option<String>,
    pub is_released: Option<bool>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FundingStatistics {
    pub total_funding: i64,
    pub funders_count: i64,
    pub average_funding: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenBalance {
    pub user_id: Uuid,
    pub balance: i64,
    pub last_updated: DateTime<Utc>,
} 