use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use log::{info, error};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: TxHash,
    pub from: Address,
    pub to: Option<Address>,
    pub value: U256,
    pub gas_price: U256,
    pub gas: U256,
    pub data: Bytes,
    pub nonce: U256,
    pub block_number: Option<U64>,
    pub status: Option<bool>,
}

pub async fn get_transaction_receipt(
    provider: &Provider<Http>,
    tx_hash: TxHash,
) -> Result<Option<TransactionReceipt>, AppError> {
    match provider.get_transaction_receipt(tx_hash).await {
        Ok(receipt) => Ok(receipt),
        Err(e) => {
            error!("Failed to get transaction receipt: {}", e);
            Err(AppError::Blockchain(format!("Failed to get transaction receipt: {}", e)))
        }
    }
}

pub async fn get_transaction(
    provider: &Provider<Http>,
    tx_hash: TxHash,
) -> Result<Option<Transaction>, AppError> {
    match provider.get_transaction(tx_hash).await {
        Ok(Some(tx)) => {
            let receipt = get_transaction_receipt(provider, tx_hash).await?;
            
            Ok(Some(Transaction {
                hash: tx.hash,
                from: tx.from,
                to: tx.to,
                value: tx.value,
                gas_price: tx.gas_price.unwrap_or_default(),
                gas: tx.gas,
                data: tx.input.clone(),
                nonce: tx.nonce,
                block_number: tx.block_number,
                status: receipt.and_then(|r| r.status).map(|s| s.as_u64() == 1),
            }))
        }
        Ok(None) => Ok(None),
        Err(e) => {
            error!("Failed to get transaction: {}", e);
            Err(AppError::Blockchain(format!("Failed to get transaction: {}", e)))
        }
    }
}

pub async fn send_transaction<T: Serialize + Send + Sync>(
    provider: &Provider<Http>,
    contract_address: Address,
    function_signature: &str,
    params: T,
    value: Option<U256>,
    wallet: &SignerMiddleware<Provider<Http>, LocalWallet>,
) -> Result<TxHash, AppError> {
    let data = encode_function_data(function_signature, params)?;
    
    let tx = TransactionRequest::new()
        .to(contract_address)
        .data(data)
        .value(value.unwrap_or_default());
    
    match wallet.send_transaction(tx, None).await {
        Ok(pending_tx) => {
            info!("Transaction sent: {}", pending_tx.tx_hash());
            Ok(pending_tx.tx_hash())
        }
        Err(e) => {
            error!("Failed to send transaction: {}", e);
            Err(AppError::Blockchain(format!("Failed to send transaction: {}", e)))
        }
    }
}

fn encode_function_data<T: Serialize>(
    function_signature: &str,
    params: T,
) -> Result<Bytes, AppError> {
    // This is a simplified implementation
    // In a real application, you would use the ethabi crate to properly encode function data
    let params_json = serde_json::to_string(&params)
        .map_err(|e| AppError::Blockchain(format!("Failed to serialize parameters: {}", e)))?;
    
    // Create a mock function selector (first 4 bytes of the keccak256 hash of the function signature)
    let selector = format!("{:08x}", keccak256(function_signature.as_bytes())[0..4].to_vec());
    
    // Combine selector with param data
    let data = format!("0x{}{}", selector, hex::encode(params_json.as_bytes()));
    
    Ok(Bytes::from(hex::decode(&data[2..]).unwrap_or_default()))
}

// Helper function to compute keccak256 hash
fn keccak256(data: &[u8]) -> [u8; 32] {
    use tiny_keccak::{Hasher, Keccak};
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(data);
    hasher.finalize(&mut output);
    output
} 