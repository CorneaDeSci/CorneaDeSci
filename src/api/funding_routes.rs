use actix_web::{web, HttpResponse, Responder, get, post, put};
use log::info;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{CreateFundingRequest, UpdateFundingStatusRequest, CreateMilestoneRequest, UpdateMilestoneRequest};
use crate::services::funding::FundingService;
use crate::database::Database;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/funding")
            .service(create_funding)
            .service(get_funding)
            .service(get_funding_by_research)
            .service(update_funding_status)
            .service(get_funding_statistics)
            .service(create_milestone)
            .service(get_milestones)
            .service(update_milestone)
            .service(get_user_balance)
    );
}

#[post("")]
async fn create_funding(
    db: web::Data<Database>,
    auth_user: web::ReqData<Uuid>,
    funding_req: web::Json<CreateFundingRequest>,
) -> Result<impl Responder, AppError> {
    let funding_service = FundingService::new(db.get_pool());
    let funding = funding_service.create_funding(*auth_user, funding_req.into_inner()).await?;
    
    Ok(HttpResponse::Created().json(funding))
}

#[get("/{id}")]
async fn get_funding(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let funding_service = FundingService::new(db.get_pool());
    let funding = funding_service.get_funding_by_id(*id).await?;
    
    Ok(HttpResponse::Ok().json(funding))
}

#[get("/research/{research_id}")]
async fn get_funding_by_research(
    db: web::Data<Database>,
    research_id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let funding_service = FundingService::new(db.get_pool());
    let fundings = funding_service.get_funding_by_research(*research_id).await?;
    
    Ok(HttpResponse::Ok().json(fundings))
}

#[put("/{id}/status")]
async fn update_funding_status(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    status_req: web::Json<UpdateFundingStatusRequest>,
) -> Result<impl Responder, AppError> {
    let funding_service = FundingService::new(db.get_pool());
    let updated_funding = funding_service.update_funding_status(*id, status_req.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(updated_funding))
}

#[get("/statistics/{research_id}")]
async fn get_funding_statistics(
    db: web::Data<Database>,
    research_id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let funding_service = FundingService::new(db.get_pool());
    let statistics = funding_service.get_funding_statistics(*research_id).await?;
    
    Ok(HttpResponse::Ok().json(statistics))
}

#[post("/milestones")]
async fn create_milestone(
    db: web::Data<Database>,
    auth_user: web::ReqData<Uuid>,
    milestone_req: web::Json<CreateMilestoneRequest>,
) -> Result<impl Responder, AppError> {
    let funding_service = FundingService::new(db.get_pool());
    let milestone = funding_service.create_milestone(*auth_user, milestone_req.into_inner()).await?;
    
    Ok(HttpResponse::Created().json(milestone))
}

#[get("/milestones/{funding_id}")]
async fn get_milestones(
    db: web::Data<Database>,
    funding_id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let funding_service = FundingService::new(db.get_pool());
    let milestones = funding_service.get_milestones(*funding_id).await?;
    
    Ok(HttpResponse::Ok().json(milestones))
}

#[put("/milestones/{id}")]
async fn update_milestone(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    auth_user: web::ReqData<Uuid>,
    milestone_req: web::Json<UpdateMilestoneRequest>,
) -> Result<impl Responder, AppError> {
    let funding_service = FundingService::new(db.get_pool());
    
    // Ensure user has access to update milestone
    funding_service.ensure_milestone_access(*id, *auth_user).await?;
    
    let milestone = funding_service.update_milestone(*id, milestone_req.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(milestone))
}

#[get("/balance")]
async fn get_user_balance(
    db: web::Data<Database>,
    auth_user: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    let funding_service = FundingService::new(db.get_pool());
    let balance = funding_service.get_user_balance(*auth_user).await?;
    
    Ok(HttpResponse::Ok().json(balance))
} 