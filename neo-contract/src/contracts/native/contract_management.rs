// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::env::contract;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use crate::types::Any;
use crate::types::Array;
use crate::prelude::*;

/// ContractManagement native contract for managing smart contracts on Neo N3
/// 
/// This contract is used to deploy, update, and manage smart contracts on the Neo N3 blockchain.
/// 
/// Contract Hash: 0xfffdc93764dbaddd97c48f252a53ea4643faa3fd
#[allow(non_snake_case)]
pub struct ContractManagement;

impl ContractManagement {
    /// Returns the contract hash for the ContractManagement native contract
    pub fn hash() -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            contract::contract_management_contract_hash()
        }
        #[cfg(target_arch = "wasm32")]
        {
            contract::native_contract_management_contract_hash()
        }
    }

    /// Deploys a new contract to the blockchain
    /// 
    /// # Arguments
    /// 
    /// * `nef_file` - The NEF (Neo Executable Format) data as a byte array
    /// * `manifest` - The contract manifest as a string
    /// * `data` - Optional data for the contract initialization
    /// 
    /// # Returns
    /// 
    /// The script hash of the deployed contract
    pub fn deploy(nef_file: &[u8], manifest: &ByteString, data: Option<Any>) -> H160 {
        let method = ByteString::from("deploy");
        let mut args = Array::<Any>::new();
        args.push(Any::from(nef_file));
        args.push(Any::from(manifest.clone()));
        
        if let Some(init_data) = data {
            args.push(init_data);
        } else {
            args.push(Any::new()); // Null value for optional parameter
        }
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or_else(|_| H160::zero())
    }

    /// Updates an existing contract
    /// 
    /// # Arguments
    /// 
    /// * `script_hash` - The script hash of the contract to update
    /// * `nef_file` - The new NEF data
    /// * `manifest` - The new contract manifest
    /// * `data` - Optional data for contract initialization
    pub fn update(script_hash: &H160, nef_file: &[u8], manifest: &ByteString, data: Option<Any>) {
        let method = ByteString::from("update");
        let mut args = Array::<Any>::new();
        args.push(Any::from(script_hash.clone()));
        args.push(Any::from(nef_file));
        args.push(Any::from(manifest.clone()));
        
        if let Some(init_data) = data {
            args.push(init_data);
        } else {
            args.push(Any::new()); // Null value for optional parameter
        }
        
        let _ = Runtime::call_contract(&Self::hash(), &method, &args);
    }

    /// Destroys a contract
    /// 
    /// # Arguments
    /// 
    /// * `script_hash` - The script hash of the contract to destroy
    pub fn destroy(script_hash: &H160) {
        let method = ByteString::from("destroy");
        let mut args = Array::<Any>::new();
        args.push(Any::from(script_hash.clone()));
        
        let _ = Runtime::call_contract(&Self::hash(), &method, &args);
    }

    /// Gets the minimum deployment fee required to deploy a contract
    /// 
    /// # Returns
    /// 
    /// The minimum fee amount in GAS
    #[safe]
    pub fn get_minimum_deployment_fee() -> u64 {
        let method = ByteString::from("getMinimumDeploymentFee");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }

    /// Sets the minimum deployment fee
    /// 
    /// # Arguments
    /// 
    /// * `value` - The new minimum fee amount in GAS
    pub fn set_minimum_deployment_fee(value: u64) {
        let method = ByteString::from("setMinimumDeploymentFee");
        let mut args = Array::<Any>::new();
        args.push(Any::from(value));
        
        let _ = Runtime::call_contract(&Self::hash(), &method, &args);
    }

    /// Gets the contract state by script hash
    /// 
    /// # Arguments
    /// 
    /// * `script_hash` - The script hash of the contract
    /// 
    /// # Returns
    /// 
    /// The contract state as an Any value
    #[safe]
    pub fn get_contract(script_hash: &H160) -> Any {
        let method = ByteString::from("getContract");
        let mut args = Array::<Any>::new();
        args.push(Any::from(script_hash.clone()));
        
        Runtime::call_contract(&Self::hash(), &method, &args)
    }

    /// Checks if a contract exists
    /// 
    /// # Arguments
    /// 
    /// * `script_hash` - The script hash of the contract to check
    /// 
    /// # Returns
    /// 
    /// True if the contract exists, false otherwise
    #[safe]
    pub fn has_contract(script_hash: &H160) -> bool {
        let method = ByteString::from("hasContract");
        let mut args = Array::<Any>::new();
        args.push(Any::from(script_hash.clone()));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(false)
    }

    /// Calculates the contract hash from a sender and nef checksum
    /// 
    /// # Arguments
    /// 
    /// * `sender` - The contract sender's script hash
    /// * `nef_checksum` - The checksum of the NEF file
    /// 
    /// # Returns
    /// 
    /// The contract script hash
    #[safe]
    pub fn get_contract_hash(sender: &H160, nef_checksum: u32) -> H160 {
        let method = ByteString::from("getContractHash");
        let mut args = Array::<Any>::new();
        args.push(Any::from(sender.clone()));
        args.push(Any::from(nef_checksum));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or_else(|_| H160::zero())
    }
}
