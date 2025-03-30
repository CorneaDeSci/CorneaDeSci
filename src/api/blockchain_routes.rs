use actix_web::{web, HttpResponse, Responder, get, post};
use log::info;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::services::blockchain::BlockchainService;
use crate::database::Database;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/blockchain")
            .service(register_research)
            .service(verify_proof_of_invention)
            .service(verify_transaction)
            .service(get_token_balance)
            .service(mint_tokens)
    );
}

#[post("/research/{id}")]
async fn register_research(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
    auth_user: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    let blockchain_service = BlockchainService::new(db.get_pool());
    let result = blockchain_service.register_research_on_blockchain(*id, *auth_user).await?;
    
    Ok(HttpResponse::Ok().json(result))
}

#[post("/verify-poi/{id}")]
async fn verify_proof_of_invention(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let blockchain_service = BlockchainService::new(db.get_pool());
    let result = blockchain_service.verify_proof_of_invention(*id).await?;
    
    Ok(HttpResponse::Ok().json(result))
}

#[post("/verify-transaction")]
async fn verify_transaction(
    db: web::Data<Database>,
    tx_req: web::Json<VerifyTransactionRequest>,
) -> Result<impl Responder, AppError> {
    let blockchain_service = BlockchainService::new(db.get_pool());
    let result = blockchain_service.verify_transaction(&tx_req.transaction_hash).await?;
    
    Ok(HttpResponse::Ok().json(result))
}

#[get("/token-balance/{address}")]
async fn get_token_balance(
    db: web::Data<Database>,
    address: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let blockchain_service = BlockchainService::new(db.get_pool());
    let balance = blockchain_service.get_token_balance(&address).await?;
    
    Ok(HttpResponse::Ok().json(TokenBalanceResponse { address: address.into_inner(), balance }))
}

#[post("/mint-tokens")]
async fn mint_tokens(
    db: web::Data<Database>,
    auth_user: web::ReqData<Uuid>,
    mint_req: web::Json<MintTokensRequest>,
) -> Result<impl Responder, AppError> {
    let blockchain_service = BlockchainService::new(db.get_pool());
    let result = blockchain_service.mint_tokens(*auth_user, &mint_req.recipient_address, mint_req.amount).await?;
    
    Ok(HttpResponse::Ok().json(result))
}

#[derive(Debug, Serialize, Deserialize)]
struct VerifyTransactionRequest {
    transaction_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenBalanceResponse {
    address: String,
    balance: u128,
}

#[derive(Debug, Serialize, Deserialize)]
struct MintTokensRequest {
    recipient_address: String,
    amount: u128,
} 