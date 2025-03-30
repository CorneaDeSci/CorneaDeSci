use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    Research, ResearchResponse, CreateResearchRequest, UpdateResearchRequest,
    ResearchStatus, ResearchType, ResearchParticipation
};

pub struct ResearchService<'a> {
    pool: &'a PgPool,
}

impl<'a> ResearchService<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
    
    pub async fn create_research(&self, user_id: Uuid, req: CreateResearchRequest) -> Result<ResearchResponse, AppError> {
        let now = Utc::now();
        let research = sqlx::query_as!(
            Research,
            r#"
            INSERT INTO researches (
                id, user_id, title, description, methodology, goal, status, 
                research_type, required_participants, current_participants, 
                funding_goal, current_funding, start_date, end_date, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING *
            "#,
            Uuid::new_v4(),
            user_id,
            req.title,
            req.description,
            req.methodology,
            req.goal,
            ResearchStatus::Draft as _,
            req.research_type as _,
            req.required_participants,
            0,
            req.funding_goal,
            0,
            req.start_date,
            req.end_date,
            now,
            now
        )
        .fetch_one(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(ResearchResponse::from(research))
    }
    
    pub async fn get_research_by_id(&self, id: Uuid) -> Result<ResearchResponse, AppError> {
        let research = sqlx::query_as!(
            Research,
            r#"SELECT * FROM researches WHERE id = $1"#,
            id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("Research with ID {} not found", id)))?;
        
        Ok(ResearchResponse::from(research))
    }
    
    pub async fn update_research_status(&self, id: Uuid, user_id: Uuid, status: ResearchStatus) -> Result<ResearchResponse, AppError> {
        // Verify research belongs to user
        let research = sqlx::query_as!(
            Research,
            r#"SELECT * FROM researches WHERE id = $1"#,
            id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("Research with ID {} not found", id)))?;
        
        if research.user_id != user_id {
            return Err(AppError::Forbidden("You don't have permission to update this research".to_string()));
        }
        
        // Update research status
        let now = Utc::now();
        let updated_research = sqlx::query_as!(
            Research,
            r#"
            UPDATE researches
            SET status = $1, updated_at = $2
            WHERE id = $3
            RETURNING *
            "#,
            status as _,
            now,
            id
        )
        .fetch_one(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(ResearchResponse::from(updated_research))
    }
    
    pub async fn update_research(&self, id: Uuid, user_id: Uuid, req: UpdateResearchRequest) -> Result<ResearchResponse, AppError> {
        // Verify research belongs to user
        let research = sqlx::query_as!(
            Research,
            r#"SELECT * FROM researches WHERE id = $1"#,
            id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("Research with ID {} not found", id)))?;
        
        if research.user_id != user_id {
            return Err(AppError::Forbidden("You don't have permission to update this research".to_string()));
        }
        
        // Update research fields
        let title = req.title.unwrap_or_else(|| research.title.clone());
        let description = req.description.unwrap_or_else(|| research.description.clone());
        let methodology = req.methodology.unwrap_or_else(|| research.methodology.clone());
        let goal = req.goal.unwrap_or_else(|| research.goal.clone());
        let research_type = req.research_type.unwrap_or(research.research_type);
        let required_participants = req.required_participants.unwrap_or(research.required_participants);
        let funding_goal = req.funding_goal.unwrap_or(research.funding_goal);
        let start_date = req.start_date.unwrap_or(research.start_date);
        let end_date = req.end_date.unwrap_or(research.end_date);
        
        let now = Utc::now();
        let updated_research = sqlx::query_as!(
            Research,
            r#"
            UPDATE researches
            SET title = $1, description = $2, methodology = $3, goal = $4, 
                research_type = $5, required_participants = $6, funding_goal = $7,
                start_date = $8, end_date = $9, updated_at = $10
            WHERE id = $11
            RETURNING *
            "#,
            title,
            description,
            methodology,
            goal,
            research_type as _,
            required_participants,
            funding_goal,
            start_date,
            end_date,
            now,
            id
        )
        .fetch_one(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(ResearchResponse::from(updated_research))
    }
    
    pub async fn delete_research(&self, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        // Verify research belongs to user
        let research = sqlx::query_as!(
            Research,
            r#"SELECT * FROM researches WHERE id = $1"#,
            id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("Research with ID {} not found", id)))?;
        
        if research.user_id != user_id {
            return Err(AppError::Forbidden("You don't have permission to delete this research".to_string()));
        }
        
        sqlx::query!("DELETE FROM researches WHERE id = $1", id)
            .execute(self.pool)
            .await
            .map_err(AppError::Database)?;
            
        Ok(())
    }
    
    pub async fn get_user_researches(&self, user_id: Uuid) -> Result<Vec<ResearchResponse>, AppError> {
        let researches = sqlx::query_as!(
            Research,
            r#"SELECT * FROM researches WHERE user_id = $1 ORDER BY created_at DESC"#,
            user_id
        )
        .fetch_all(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(researches.into_iter().map(ResearchResponse::from).collect())
    }
    
    pub async fn get_public_researches(&self) -> Result<Vec<ResearchResponse>, AppError> {
        let researches = sqlx::query_as!(
            Research,
            r#"
            SELECT * FROM researches 
            WHERE status != $1 
            ORDER BY created_at DESC
            "#,
            ResearchStatus::Draft as _
        )
        .fetch_all(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(researches.into_iter().map(ResearchResponse::from).collect())
    }
    
    pub async fn join_research(&self, research_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        // Check if research exists and is accepting participants
        let research = sqlx::query_as!(
            Research,
            r#"SELECT * FROM researches WHERE id = $1"#,
            research_id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("Research with ID {} not found", research_id)))?;
        
        if research.status != ResearchStatus::Active {
            return Err(AppError::Conflict("Research is not currently accepting participants".to_string()));
        }
        
        if research.current_participants >= research.required_participants {
            return Err(AppError::Conflict("Research has reached maximum participant capacity".to_string()));
        }
        
        // Check if user is already participating
        let existing = sqlx::query!(
            r#"
            SELECT * FROM research_participations 
            WHERE research_id = $1 AND user_id = $2
            "#,
            research_id,
            user_id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        if existing.is_some() {
            return Err(AppError::Conflict("User is already participating in this research".to_string()));
        }
        
        // Create participation record
        let now = Utc::now();
        sqlx::query!(
            r#"
            INSERT INTO research_participations (
                id, research_id, user_id, joined_at
            )
            VALUES ($1, $2, $3, $4)
            "#,
            Uuid::new_v4(),
            research_id,
            user_id,
            now
        )
        .execute(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        // Increment participant count
        sqlx::query!(
            r#"
            UPDATE researches
            SET current_participants = current_participants + 1
            WHERE id = $1
            "#,
            research_id
        )
        .execute(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(())
    }
    
    pub async fn get_research_participants(&self, research_id: Uuid) -> Result<Vec<ResearchParticipation>, AppError> {
        // Verify research exists
        let research = sqlx::query_as!(
            Research,
            r#"SELECT * FROM researches WHERE id = $1"#,
            research_id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound(format!("Research with ID {} not found", research_id)))?;
        
        // Get participants
        let participants = sqlx::query_as!(
            ResearchParticipation,
            r#"
            SELECT rp.*, u.username, u.full_name 
            FROM research_participations rp
            JOIN users u ON rp.user_id = u.id
            WHERE rp.research_id = $1
            ORDER BY rp.joined_at DESC
            "#,
            research_id
        )
        .fetch_all(self.pool)
        .await
        .map_err(AppError::Database)?;
        
        Ok(participants)
    }
} 