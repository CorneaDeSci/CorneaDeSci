use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// The smart contract ABI for Corneal Research Registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchRegistryContract<M> {
    contract: ethers::contract::Contract<M>,
    address: ethers::types::Address,
}

/// The smart contract ABI for Cornea Platform Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorneaTokenContract<M> {
    contract: ethers::contract::Contract<M>,
    address: ethers::types::Address,
}

impl<M: Middleware> ResearchRegistryContract<M> {
    pub fn new(address: ethers::types::Address, client: Arc<M>) -> Self {
        // In a real implementation, this would load the ABI from a compiled artifact
        // For this example, we'll create a minimal ABI
        let abi = r#"[
            {
                "inputs": [
                    {
                        "internalType": "string",
                        "name": "researchId",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "metadataURI",
                        "type": "string"
                    }
                ],
                "name": "registerResearch",
                "outputs": [
                    {
                        "internalType": "uint256",
                        "name": "blockchainId",
                        "type": "uint256"
                    }
                ],
                "stateMutability": "nonpayable",
                "type": "function"
            },
            {
                "inputs": [
                    {
                        "internalType": "string",
                        "name": "poiId",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "ipfsHash",
                        "type": "string"
                    }
                ],
                "name": "registerProofOfInvention",
                "outputs": [
                    {
                        "internalType": "bool",
                        "name": "success",
                        "type": "bool"
                    }
                ],
                "stateMutability": "nonpayable",
                "type": "function"
            },
            {
                "inputs": [
                    {
                        "internalType": "uint256",
                        "name": "blockchainId",
                        "type": "uint256"
                    }
                ],
                "name": "getResearchMetadata",
                "outputs": [
                    {
                        "internalType": "string",
                        "name": "metadataURI",
                        "type": "string"
                    },
                    {
                        "internalType": "address",
                        "name": "registeredBy",
                        "type": "address"
                    },
                    {
                        "internalType": "uint256",
                        "name": "timestamp",
                        "type": "uint256"
                    }
                ],
                "stateMutability": "view",
                "type": "function"
            }
        ]"#;
        
        let contract = ethers::contract::Contract::new(address, serde_json::from_str(abi).unwrap(), client);
        
        Self {
            contract,
            address,
        }
    }
    
    pub async fn register_research(&self, research_id: &str, metadata_uri: &str) -> Result<U256, ethers::contract::ContractError<M>> {
        self.contract
            .method::<_, U256>("registerResearch", (research_id, metadata_uri))?
            .send()
            .await?
            .await?
            .transaction_receipt()
            .map(|receipt| receipt.block_number.unwrap_or_default().as_u64().into())
    }
    
    pub async fn register_proof_of_invention(&self, poi_id: &str, ipfs_hash: &str) -> Result<bool, ethers::contract::ContractError<M>> {
        self.contract
            .method::<_, bool>("registerProofOfInvention", (poi_id, ipfs_hash))?
            .send()
            .await?
            .await?
            .transaction_receipt()
            .map(|_| true)
    }
}

impl<M: Middleware> CorneaTokenContract<M> {
    pub fn new(address: ethers::types::Address, client: Arc<M>) -> Self {
        // In a real implementation, this would load the ABI from a compiled artifact
        // For this example, we'll create a minimal ERC20 ABI
        let abi = r#"[
            {
                "inputs": [
                    {
                        "internalType": "address",
                        "name": "account",
                        "type": "address"
                    }
                ],
                "name": "balanceOf",
                "outputs": [
                    {
                        "internalType": "uint256",
                        "name": "",
                        "type": "uint256"
                    }
                ],
                "stateMutability": "view",
                "type": "function"
            },
            {
                "inputs": [
                    {
                        "internalType": "address",
                        "name": "to",
                        "type": "address"
                    },
                    {
                        "internalType": "uint256",
                        "name": "amount",
                        "type": "uint256"
                    }
                ],
                "name": "mint",
                "outputs": [],
                "stateMutability": "nonpayable",
                "type": "function"
            },
            {
                "inputs": [
                    {
                        "internalType": "address",
                        "name": "to",
                        "type": "address"
                    },
                    {
                        "internalType": "uint256",
                        "name": "amount",
                        "type": "uint256"
                    }
                ],
                "name": "transfer",
                "outputs": [
                    {
                        "internalType": "bool",
                        "name": "",
                        "type": "bool"
                    }
                ],
                "stateMutability": "nonpayable",
                "type": "function"
            }
        ]"#;
        
        let contract = ethers::contract::Contract::new(address, serde_json::from_str(abi).unwrap(), client);
        
        Self {
            contract,
            address,
        }
    }
    
    pub async fn balance_of(&self, address: ethers::types::Address) -> Result<U256, ethers::contract::ContractError<M>> {
        self.contract
            .method::<_, U256>("balanceOf", address)?
            .call()
            .await
    }
    
    pub async fn mint(&self, to: ethers::types::Address, amount: U256) -> Result<ethers::providers::TransactionReceipt, ethers::contract::ContractError<M>> {
        self.contract
            .method::<_, ()>("mint", (to, amount))?
            .send()
            .await?
            .await?
    }
    
    pub async fn transfer(&self, to: ethers::types::Address, amount: U256) -> Result<bool, ethers::contract::ContractError<M>> {
        self.contract
            .method::<_, bool>("transfer", (to, amount))?
            .send()
            .await?
            .await?
            .transaction_receipt()
            .map(|_| true)
    }
} 