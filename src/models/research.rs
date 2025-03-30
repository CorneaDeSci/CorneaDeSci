use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::user::UserRole;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "research_status", rename_all = "lowercase")]
pub enum ResearchStatus {
    Draft,
    UnderReview,
    Approved,
    Rejected,
    InProgress,
    Completed,
    Archived,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Research {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub detailed_proposal: String,
    pub researcher_id: Uuid,
    pub status: ResearchStatus,
    pub funding_target: i64,
    pub current_funding: i64,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub blockchain_id: Option<String>,
    pub ipfs_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResearchRequest {
    pub title: String,
    pub description: String,
    pub detailed_proposal: String,
    pub funding_target: i64,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResearchRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub detailed_proposal: Option<String>,
    pub funding_target: Option<i64>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: Option<ResearchStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResearchDatapoint {
    pub id: Uuid,
    pub research_id: Uuid,
    pub title: String,
    pub description: String,
    pub data_type: String,
    pub ipfs_hash: String,
    pub timestamp: DateTime<Utc>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub blockchain_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDatapointRequest {
    pub research_id: Uuid,
    pub title: String,
    pub description: String,
    pub data_type: String,
    pub ipfs_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofOfInvention {
    pub id: Uuid,
    pub research_id: Uuid,
    pub title: String,
    pub description: String,
    pub ipfs_hash: String,
    pub blockchain_tx: String,
    pub timestamp: DateTime<Utc>,
    pub inventor_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePoIRequest {
    pub research_id: Uuid,
    pub title: String,
    pub description: String,
    pub ipfs_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collaboration {
    pub id: Uuid,
    pub research_id: Uuid,
    pub user_id: Uuid,
    pub role: UserRole,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCollaboratorRequest {
    pub research_id: Uuid,
    pub user_id: Uuid,
    pub role: UserRole,
    pub permissions: Vec<String>,
} 