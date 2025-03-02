// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};
use crate::builtin::{H160, ByteString, Array, Any, Int256};
use crate::Runtime;
use neo_contract_proc_macros::safe;

/// Policy native contract for system policy management
pub struct Policy;

impl Policy {
    /// Get the Policy contract hash
    #[inline(always)]
    #[rustfmt::skip]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_policy_contract_hash() }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0xcc5e4edd9f5f8dba8bb65734541df7a1c081c67b").unwrap_or_else(H160::zero)
    }
    
    /// Get the fee per byte
    pub fn get_fee_per_byte() -> Int256 {
        let method = ByteString::from("getFeePerByte");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match Int256::try_from(result) {
            Ok(fee) => fee,
            Err(_) => Int256::zero(),
        }
    }
    
    /// Set the fee per byte
    pub fn set_fee_per_byte(fee: Int256) -> bool {
        let method = ByteString::from("setFeePerByte");
        let mut args = Array::<Any>::new();
        args.push(Any::from(fee));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
    
    /// Get the execution fee factor
    pub fn get_exec_fee_factor() -> Int256 {
        let method = ByteString::from("getExecFeeFactor");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match Int256::try_from(result) {
            Ok(factor) => factor,
            Err(_) => Int256::zero(),
        }
    }
    
    /// Set the execution fee factor
    pub fn set_exec_fee_factor(factor: Int256) -> bool {
        let method = ByteString::from("setExecFeeFactor");
        let mut args = Array::<Any>::new();
        args.push(Any::from(factor));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
    
    /// Get the storage price
    pub fn get_storage_price() -> Int256 {
        let method = ByteString::from("getStoragePrice");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match Int256::try_from(result) {
            Ok(price) => price,
            Err(_) => Int256::zero(),
        }
    }
    
    /// Set the storage price
    pub fn set_storage_price(price: Int256) -> bool {
        let method = ByteString::from("setStoragePrice");
        let mut args = Array::<Any>::new();
        args.push(Any::from(price));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
    
    /// Get whether blocklist is enabled
    pub fn is_blocked(script_hash: H160) -> bool {
        let method = ByteString::from("isBlocked");
        let mut args = Array::<Any>::new();
        args.push(Any::from(script_hash));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(is_blocked) => is_blocked,
            Err(_) => false,
        }
    }
    
    /// Block a contract
    pub fn block_account(script_hash: H160) -> bool {
        let method = ByteString::from("blockAccount");
        let mut args = Array::<Any>::new();
        args.push(Any::from(script_hash));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
    
    /// Unblock a contract
    pub fn unblock_account(script_hash: H160) -> bool {
        let method = ByteString::from("unblockAccount");
        let mut args = Array::<Any>::new();
        args.push(Any::from(script_hash));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
    
    /// Get the maximum transaction fee
    pub fn get_max_transaction_fee() -> Int256 {
        let method = ByteString::from("getMaxTransactionFee");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match Int256::try_from(result) {
            Ok(fee) => fee,
            Err(_) => Int256::zero(),
        }
    }
    
    /// Set the maximum transaction fee
    pub fn set_max_transaction_fee(fee: Int256) -> bool {
        let method = ByteString::from("setMaxTransactionFee");
        let mut args = Array::<Any>::new();
        args.push(Any::from(fee));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
    
    /// Get the minimum deployment fee
    pub fn get_min_deployment_fee() -> Int256 {
        let method = ByteString::from("getMinimumDeploymentFee");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match Int256::try_from(result) {
            Ok(fee) => fee,
            Err(_) => Int256::zero(),
        }
    }
    
    /// Set the minimum deployment fee
    pub fn set_min_deployment_fee(fee: Int256) -> bool {
        let method = ByteString::from("setMinimumDeploymentFee");
        let mut args = Array::<Any>::new();
        args.push(Any::from(fee));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
    
    /// Get the fee for a specific transaction attribute type
    pub fn get_attribute_fee(attribute_type: crate::TransactionAttributeType) -> u32 {
        let method = ByteString::from("getAttributeFee");
        let mut args = Array::<Any>::new();
        args.push(Any::from(attribute_type as u8));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match u32::try_from(result) {
            Ok(fee) => fee,
            Err(_) => 0,
        }
    }
    
    /// Set the fee for a specific transaction attribute type
    pub fn set_attribute_fee(attribute_type: crate::TransactionAttributeType, value: u32) -> bool {
        let method = ByteString::from("setAttributeFee");
        let mut args = Array::<Any>::new();
        args.push(Any::from(attribute_type as u8));
        args.push(Any::from(value));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
}
