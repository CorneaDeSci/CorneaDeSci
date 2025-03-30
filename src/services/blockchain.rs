use std::sync::Arc;

use ethers::{
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Transaction, TransactionReceipt, H256, U256},
};
use tokio::sync::Mutex;

use crate::config::BlockchainConfig;
use crate::error::AppError;
use crate::models::{TransactionRequest, TransactionResponse};

pub struct BlockchainService {
    provider: Arc<Provider<Http>>,
    wallet: Arc<Mutex<LocalWallet>>,
    contract_address: Address,
}

impl BlockchainService {
    pub fn new(config: &BlockchainConfig) -> Result<Self, AppError> {
        let provider = Provider::<Http>::try_from(&config.rpc_url)
            .map_err(|e| AppError::Blockchain(format!("Failed to connect to provider: {}", e)))?;
        
        let wallet = config.private_key
            .parse::<LocalWallet>()
            .map_err(|e| AppError::Blockchain(format!("Invalid private key: {}", e)))?;
            
        let contract_address = config.contract_address
            .parse::<Address>()
            .map_err(|e| AppError::Blockchain(format!("Invalid contract address: {}", e)))?;
            
        Ok(Self {
            provider: Arc::new(provider),
            wallet: Arc::new(Mutex::new(wallet)),
            contract_address,
        })
    }
    
    pub async fn verify_transaction(&self, tx_hash: &str) -> Result<TransactionReceipt, AppError> {
        let hash = tx_hash
            .parse::<H256>()
            .map_err(|e| AppError::Blockchain(format!("Invalid transaction hash: {}", e)))?;
            
        let receipt = self.provider
            .get_transaction_receipt(hash)
            .await
            .map_err(|e| AppError::Blockchain(format!("Failed to get transaction receipt: {}", e)))?
            .ok_or_else(|| AppError::Blockchain("Transaction not found or not confirmed".to_string()))?;
            
        Ok(receipt)
    }
    
    pub async fn get_transaction(&self, tx_hash: &str) -> Result<Transaction, AppError> {
        let hash = tx_hash
            .parse::<H256>()
            .map_err(|e| AppError::Blockchain(format!("Invalid transaction hash: {}", e)))?;
            
        let tx = self.provider
            .get_transaction(hash)
            .await
            .map_err(|e| AppError::Blockchain(format!("Failed to get transaction: {}", e)))?
            .ok_or_else(|| AppError::Blockchain("Transaction not found".to_string()))?;
            
        Ok(tx)
    }
    
    pub async fn send_transaction(&self, req: TransactionRequest) -> Result<TransactionResponse, AppError> {
        let wallet = self.wallet.lock().await;
        let from_address = wallet.address();
        
        let to_address = req.to
            .parse::<Address>()
            .map_err(|e| AppError::Blockchain(format!("Invalid destination address: {}", e)))?;
            
        let value = U256::from(req.amount);
        
        // Get current gas price
        let gas_price = self.provider
            .get_gas_price()
            .await
            .map_err(|e| AppError::Blockchain(format!("Failed to get gas price: {}", e)))?;
            
        // Build the transaction
        let nonce = self.provider
            .get_transaction_count(from_address, None)
            .await
            .map_err(|e| AppError::Blockchain(format!("Failed to get nonce: {}", e)))?;
            
        let tx = ethers::types::TransactionRequest::new()
            .from(from_address)
            .to(to_address)
            .value(value)
            .gas_price(gas_price)
            .nonce(nonce)
            .data(req.data.unwrap_or_default());
            
        // Estimate gas
        let gas = self.provider
            .estimate_gas(&tx, None)
            .await
            .map_err(|e| AppError::Blockchain(format!("Failed to estimate gas: {}", e)))?;
            
        let tx = tx.gas(gas);
        
        // Sign and send transaction
        let tx = wallet.sign_transaction(&tx.into())
            .await
            .map_err(|e| AppError::Blockchain(format!("Failed to sign transaction: {}", e)))?;
            
        let pending_tx = self.provider
            .send_raw_transaction(tx)
            .await
            .map_err(|e| AppError::Blockchain(format!("Failed to send transaction: {}", e)))?;
            
        let tx_hash = pending_tx.tx_hash();
            
        Ok(TransactionResponse {
            transaction_hash: format!("{:?}", tx_hash),
            from: format!("{:?}", from_address),
            to: req.to,
            amount: req.amount,
            gas_used: gas.as_u64(),
            gas_price: gas_price.as_u64(),
        })
    }
    
    pub async fn get_balance(&self, address: &str) -> Result<U256, AppError> {
        let address = address
            .parse::<Address>()
            .map_err(|e| AppError::Blockchain(format!("Invalid address: {}", e)))?;
            
        let balance = self.provider
            .get_balance(address, None)
            .await
            .map_err(|e| AppError::Blockchain(format!("Failed to get balance: {}", e)))?;
            
        Ok(balance)
    }
    
    pub async fn get_contract_balance(&self) -> Result<U256, AppError> {
        let balance = self.provider
            .get_balance(self.contract_address, None)
            .await
            .map_err(|e| AppError::Blockchain(format!("Failed to get contract balance: {}", e)))?;
            
        Ok(balance)
    }
    
    pub async fn call_contract_function(&self, function: &str, args: Vec<String>) -> Result<String, AppError> {
        // In a real implementation, this would use ethers-rs contract abstraction
        // For simplicity, we're just returning a placeholder
        Ok("0x0000000000000000000000000000000000000000000000000000000000000001".to_string())
    }
} 