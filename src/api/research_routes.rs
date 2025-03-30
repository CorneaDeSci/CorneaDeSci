use actix_web::{web, HttpResponse, Responder, get, post, put, delete};
use log::info;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{CreateResearchRequest, UpdateResearchRequest, CreateDatapointRequest, CreatePoIRequest, AddCollaboratorRequest};
use crate::services::research::ResearchService;
use crate::database::Database;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/research")
            .service(create_research)
            .service(get_research)
            .service(get_all_research)
            .service(update_research)
            .service(delete_research)
            .service(add_datapoint)
            .service(get_datapoints)
            .service(submit_proof_of_invention)
            .service(get_proof_of_inventions)
            .service(add_collaborator)
            .service(get_collaborators)
            .service(remove_collaborator)
    );
}

#[post("")]
async fn create_research(
    db: web::Data<Database>,
    auth_user: web::ReqData<Uuid>,
    research_req: web::Json<CreateResearchRequest>,
) -> Result<impl Responder, AppError> {
    let research_service = ResearchService::new(db.get_pool());
    let research = research_service.create_research(*auth_user, research_req.into_inner()).await?;
    
    Ok(HttpResponse::Created().json(research))
}

#[get("/{id}")]
async fn get_research(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let research_service = ResearchService::new(db.get_pool());
    let research = research_service.get_research_by_id(*id).await?;
    
    Ok(HttpResponse::Ok().json(research))
}

#[get("")]
async fn get_all_research(
    db: web::Data<Database>,
    query: web::Query<GetResearchQuery>,
) -> Result<impl Responder, AppError> {
    let research_service = ResearchService::new(db.get_pool());
    let researches = match query.researcher_id {
        Some(id) => research_service.get_research_by_researcher(id).await?,
        None => research_service.get_all_research().await?,
    };
    
    Ok(HttpResponse::Ok().json(researches))
}

#[put("/{id}")]
async fn update_research(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    auth_user: web::ReqData<Uuid>,
    update_req: web::Json<UpdateResearchRequest>,
) -> Result<impl Responder, AppError> {
    let research_service = ResearchService::new(db.get_pool());
    
    // Ensure user is the owner or collaborator
    research_service.ensure_research_access(*id, *auth_user).await?;
    
    let updated_research = research_service.update_research(*id, update_req.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(updated_research))
}

#[delete("/{id}")]
async fn delete_research(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    auth_user: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    let research_service = ResearchService::new(db.get_pool());
    
    // Ensure user is the owner
    research_service.ensure_research_ownership(*id, *auth_user).await?;
    
    research_service.delete_research(*id).await?;
    
    Ok(HttpResponse::NoContent().finish())
}

#[post("/{id}/datapoints")]
async fn add_datapoint(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    auth_user: web::ReqData<Uuid>,
    datapoint_req: web::Json<CreateDatapointRequest>,
) -> Result<impl Responder, AppError> {
    let research_service = ResearchService::new(db.get_pool());
    
    // Ensure user is the owner or collaborator
    research_service.ensure_research_access(*id, *auth_user).await?;
    
    let mut request = datapoint_req.into_inner();
    request.research_id = *id;
    
    let datapoint = research_service.add_datapoint(*auth_user, request).await?;
    
    Ok(HttpResponse::Created().json(datapoint))
}

#[get("/{id}/datapoints")]
async fn get_datapoints(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let research_service = ResearchService::new(db.get_pool());
    let datapoints = research_service.get_datapoints(*id).await?;
    
    Ok(HttpResponse::Ok().json(datapoints))
}

#[post("/{id}/proof-of-invention")]
async fn submit_proof_of_invention(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    auth_user: web::ReqData<Uuid>,
    poi_req: web::Json<CreatePoIRequest>,
) -> Result<impl Responder, AppError> {
    let research_service = ResearchService::new(db.get_pool());
    
    // Ensure user is the owner or collaborator
    research_service.ensure_research_access(*id, *auth_user).await?;
    
    let mut request = poi_req.into_inner();
    request.research_id = *id;
    
    let poi = research_service.submit_proof_of_invention(*auth_user, request).await?;
    
    Ok(HttpResponse::Created().json(poi))
}

#[get("/{id}/proof-of-invention")]
async fn get_proof_of_inventions(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let research_service = ResearchService::new(db.get_pool());
    let pois = research_service.get_proof_of_inventions(*id).await?;
    
    Ok(HttpResponse::Ok().json(pois))
}

#[post("/{id}/collaborators")]
async fn add_collaborator(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    auth_user: web::ReqData<Uuid>,
    collaborator_req: web::Json<AddCollaboratorRequest>,
) -> Result<impl Responder, AppError> {
    let research_service = ResearchService::new(db.get_pool());
    
    // Ensure user is the owner
    research_service.ensure_research_ownership(*id, *auth_user).await?;
    
    let mut request = collaborator_req.into_inner();
    request.research_id = *id;
    
    let collaboration = research_service.add_collaborator(request).await?;
    
    Ok(HttpResponse::Created().json(collaboration))
}

#[get("/{id}/collaborators")]
async fn get_collaborators(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let research_service = ResearchService::new(db.get_pool());
    let collaborators = research_service.get_collaborators(*id).await?;
    
    Ok(HttpResponse::Ok().json(collaborators))
}

#[delete("/{id}/collaborators/{user_id}")]
async fn remove_collaborator(
    db: web::Data<Database>,
    path: web::Path<(Uuid, Uuid)>,
    auth_user: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    let (research_id, user_id) = path.into_inner();
    let research_service = ResearchService::new(db.get_pool());
    
    // Ensure user is the owner
    research_service.ensure_research_ownership(research_id, *auth_user).await?;
    
    research_service.remove_collaborator(research_id, user_id).await?;
    
    Ok(HttpResponse::NoContent().finish())
}

#[derive(serde::Deserialize)]
struct GetResearchQuery {
    researcher_id: Option<Uuid>,
} 