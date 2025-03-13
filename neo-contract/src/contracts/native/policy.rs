// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::env::contract;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use crate::types::Any;
use crate::types::Array;
use crate::prelude::*;

/// Policy native contract for Neo N3
/// 
/// This contract manages blockchain policies such as system fees, 
/// execution fee rates, and blockchain settings.
/// 
/// Contract Hash: 0xcc5e4edd9f5f8dba8bb65734541df7a1c081c67b
#[allow(non_snake_case)]
pub struct Policy;

impl Policy {
    /// Returns the contract hash for the Policy native contract
    pub fn hash() -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            contract::policy_contract_hash()
        }
        #[cfg(target_arch = "wasm32")]
        {
            contract::native_policy_contract_hash()
        }
    }

    /// Gets the fee per byte for transactions
    /// 
    /// # Returns
    /// 
    /// The fee per byte in GAS
    #[safe]
    pub fn get_fee_per_byte() -> i64 {
        let method = ByteString::from("getFeePerByte");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }

    /// Sets the fee per byte for transactions
    /// 
    /// # Arguments
    /// 
    /// * `fee` - The new fee per byte in GAS
    pub fn set_fee_per_byte(fee: i64) {
        let method = ByteString::from("setFeePerByte");
        let mut args = Array::<Any>::new();
        args.push(Any::from(fee));
        
        let _ = Runtime::call_contract(&Self::hash(), &method, &args);
    }

    /// Gets the exec fee factor for smart contracts
    /// 
    /// # Returns
    /// 
    /// The execution fee factor
    #[safe]
    pub fn get_exec_fee_factor() -> i32 {
        let method = ByteString::from("getExecFeeFactor");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }

    /// Sets the exec fee factor for smart contracts
    /// 
    /// # Arguments
    /// 
    /// * `factor` - The new execution fee factor
    pub fn set_exec_fee_factor(factor: i32) {
        let method = ByteString::from("setExecFeeFactor");
        let mut args = Array::<Any>::new();
        args.push(Any::from(factor));
        
        let _ = Runtime::call_contract(&Self::hash(), &method, &args);
    }

    /// Gets the storage price per byte
    /// 
    /// # Returns
    /// 
    /// The storage price per byte in GAS
    #[safe]
    pub fn get_storage_price() -> i32 {
        let method = ByteString::from("getStoragePrice");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }

    /// Sets the storage price per byte
    /// 
    /// # Arguments
    /// 
    /// * `price` - The new storage price per byte in GAS
    pub fn set_storage_price(price: i32) {
        let method = ByteString::from("setStoragePrice");
        let mut args = Array::<Any>::new();
        args.push(Any::from(price));
        
        let _ = Runtime::call_contract(&Self::hash(), &method, &args);
    }

    /// Gets the network fee per block
    /// 
    /// # Returns
    /// 
    /// The network fee per block
    #[safe]
    pub fn get_network_fee_per_block() -> i64 {
        let method = ByteString::from("getNetworkFeePerBlock");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }

    /// Gets the block account
    /// 
    /// # Returns
    /// 
    /// The block account as H160
    #[safe]
    pub fn get_block_account() -> H160 {
        let method = ByteString::from("getBlockAccount");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or_else(|_| H160::zero())
    }

    /// Checks if a contract is blocked
    /// 
    /// # Arguments
    /// 
    /// * `hash` - The hash of the contract to check
    /// 
    /// # Returns
    /// 
    /// True if the contract is blocked, false otherwise
    #[safe]
    pub fn is_blocked(hash: &H160) -> bool {
        let method = ByteString::from("isBlocked");
        let mut args = Array::<Any>::new();
        args.push(Any::from(hash.clone()));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(false)
    }

    /// Blocks a contract
    /// 
    /// # Arguments
    /// 
    /// * `hash` - The hash of the contract to block
    pub fn block_account(hash: &H160) {
        let method = ByteString::from("blockAccount");
        let mut args = Array::<Any>::new();
        args.push(Any::from(hash.clone()));
        
        let _ = Runtime::call_contract(&Self::hash(), &method, &args);
    }

    /// Unblocks a contract
    /// 
    /// # Arguments
    /// 
    /// * `hash` - The hash of the contract to unblock
    pub fn unblock_account(hash: &H160) {
        let method = ByteString::from("unblockAccount");
        let mut args = Array::<Any>::new();
        args.push(Any::from(hash.clone()));
        
        let _ = Runtime::call_contract(&Self::hash(), &method, &args);
    }
}
