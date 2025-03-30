use actix_web::{web, HttpResponse, Responder, get, post, put, delete};
use log::info;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{CreateUserRequest, UpdateUserRequest, LoginRequest};
use crate::services::auth::AuthService;
use crate::services::user::UserService;
use crate::database::Database;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .service(register)
            .service(login)
            .service(get_current_user)
            .service(get_user_by_id)
            .service(update_user)
            .service(delete_user)
            .service(get_users)
    );
}

#[post("/register")]
async fn register(
    db: web::Data<Database>,
    user_req: web::Json<CreateUserRequest>,
) -> Result<impl Responder, AppError> {
    let user_service = UserService::new(db.get_pool());
    let auth_service = AuthService::new();
    
    let user = user_service.create_user(user_req.into_inner()).await?;
    let token = auth_service.generate_token(user.id)?;
    
    Ok(HttpResponse::Created().json(auth_service.create_auth_response(user, token)))
}

#[post("/login")]
async fn login(
    db: web::Data<Database>,
    login_req: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let user_service = UserService::new(db.get_pool());
    let auth_service = AuthService::new();
    
    let user = user_service.authenticate_user(&login_req.email, &login_req.password).await?;
    let token = auth_service.generate_token(user.id)?;
    
    Ok(HttpResponse::Ok().json(auth_service.create_auth_response(user, token)))
}

#[get("/me")]
async fn get_current_user(
    db: web::Data<Database>,
    auth_user: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    let user_service = UserService::new(db.get_pool());
    let user = user_service.get_user_by_id(*auth_user).await?;
    
    Ok(HttpResponse::Ok().json(user))
}

#[get("/{id}")]
async fn get_user_by_id(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let user_service = UserService::new(db.get_pool());
    let user = user_service.get_user_by_id(*id).await?;
    
    Ok(HttpResponse::Ok().json(user))
}

#[put("/{id}")]
async fn update_user(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    auth_user: web::ReqData<Uuid>,
    update_req: web::Json<UpdateUserRequest>,
) -> Result<impl Responder, AppError> {
    if *id != *auth_user {
        return Err(AppError::Auth("Cannot update another user's profile".to_string()));
    }
    
    let user_service = UserService::new(db.get_pool());
    let updated_user = user_service.update_user(*id, update_req.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(updated_user))
}

#[delete("/{id}")]
async fn delete_user(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    auth_user: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    if *id != *auth_user {
        return Err(AppError::Auth("Cannot delete another user's profile".to_string()));
    }
    
    let user_service = UserService::new(db.get_pool());
    user_service.delete_user(*id).await?;
    
    Ok(HttpResponse::NoContent().finish())
}

#[get("")]
async fn get_users(
    db: web::Data<Database>,
) -> Result<impl Responder, AppError> {
    let user_service = UserService::new(db.get_pool());
    let users = user_service.get_users().await?;
    
    Ok(HttpResponse::Ok().json(users))
} 