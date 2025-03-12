// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};
use crate::prelude::{H160, ByteString, Array, Any, Int256};
use crate::runtime::Runtime;
use crate::transaction_attribute_type::TransactionAttributeType;

/// Policy native contract for system policy management
pub struct Policy;

impl Policy {
    /// Get the Policy contract hash
    #[inline(always)]
    #[rustfmt::skip]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { 
            let h160 = env::contract::native_policy_contract_hash();
            // Convert from types::builtin::H160 to builtin::H160
            H160::try_from(h160.0.as_slice()).unwrap()
        }

        #[cfg(not(target_family = "wasm"))]
        // Create a placeholder hash - in a real implementation this would be properly injected
        H160::zero()
    }
    
    /// Get the fee per byte
    pub fn get_fee_per_byte() -> Int256 {
        let method = ByteString::from("getFeePerByte");
        let args = Array::new();
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract integer value
        if let Any::Integer(value) = result {
            value
        } else {
            Int256::zero()
        }
    }
    
    /// Set the fee per byte
    pub fn set_fee_per_byte(fee: Int256) -> bool {
        let method = ByteString::from("setFeePerByte");
        let mut args = Array::new();
        args.push(Any::from(fee));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract boolean value
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }
    
    /// Get the execution fee factor
    pub fn get_exec_fee_factor() -> Int256 {
        let method = ByteString::from("getExecFeeFactor");
        let args = Array::new();
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract integer value
        if let Any::Integer(value) = result {
            value
        } else {
            Int256::zero()
        }
    }
    
    /// Set the execution fee factor
    pub fn set_exec_fee_factor(factor: Int256) -> bool {
        let method = ByteString::from("setExecFeeFactor");
        let mut args = Array::new();
        args.push(Any::from(factor));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract boolean value
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }
    
    /// Get the storage price
    pub fn get_storage_price() -> Int256 {
        let method = ByteString::from("getStoragePrice");
        let args = Array::new();
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract integer value
        if let Any::Integer(value) = result {
            value
        } else {
            Int256::zero()
        }
    }
    
    /// Set the storage price
    pub fn set_storage_price(price: Int256) -> bool {
        let method = ByteString::from("setStoragePrice");
        let mut args = Array::new();
        args.push(Any::from(price));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract boolean value
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }
    
    /// Check if an account is blocked
    pub fn is_blocked(script_hash: H160) -> bool {
        let method = ByteString::from("isBlocked");
        let mut args = Array::new();
        args.push(Any::from(script_hash));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract boolean value
        if let Any::Boolean(is_blocked) = result {
            is_blocked
        } else {
            false
        }
    }
    
    /// Block an account
    pub fn block_account(script_hash: H160) -> bool {
        let method = ByteString::from("blockAccount");
        let mut args = Array::new();
        args.push(Any::from(script_hash));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract boolean value
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }
    
    /// Unblock an account
    pub fn unblock_account(script_hash: H160) -> bool {
        let method = ByteString::from("unblockAccount");
        let mut args = Array::new();
        args.push(Any::from(script_hash));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract boolean value
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }
    
    /// Get the maximum transaction fee
    pub fn get_max_transaction_fee() -> Int256 {
        let method = ByteString::from("getMaxTransactionFee");
        let args = Array::new();
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract integer value
        if let Any::Integer(value) = result {
            value
        } else {
            Int256::zero()
        }
    }
    
    /// Set the maximum transaction fee
    pub fn set_max_transaction_fee(fee: Int256) -> bool {
        let method = ByteString::from("setMaxTransactionFee");
        let mut args = Array::new();
        args.push(Any::from(fee));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract boolean value
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }
    
    /// Get the minimum deployment fee
    pub fn get_min_deployment_fee() -> Int256 {
        let method = ByteString::from("getMinimumDeploymentFee");
        let args = Array::new();
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract integer value
        if let Any::Integer(value) = result {
            value
        } else {
            Int256::zero()
        }
    }
    
    /// Set the minimum deployment fee
    pub fn set_min_deployment_fee(fee: Int256) -> bool {
        let method = ByteString::from("setMinimumDeploymentFee");
        let mut args = Array::new();
        args.push(Any::from(fee));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract boolean value
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }
    
    /// Get the fee for a transaction attribute
    pub fn get_attribute_fee(attribute_type: TransactionAttributeType) -> u32 {
        let method = ByteString::from("getAttributeFee");
        let mut args = Array::new();
        
        // Convert the TransactionAttributeType to a ByteString since we can't convert u8 directly
        let type_value = ByteString::from_bytes(&[attribute_type as u8]);
        args.push(Any::from(type_value));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract integer value and convert to u32
        if let Any::Integer(value) = result {
            value.to_u64().unwrap_or(0) as u32
        } else {
            0
        }
    }
    
    /// Set the fee for a transaction attribute
    pub fn set_attribute_fee(attribute_type: TransactionAttributeType, value: u32) -> bool {
        let method = ByteString::from("setAttributeFee");
        let mut args = Array::new();
        
        // Convert the TransactionAttributeType to a ByteString since we can't convert u8 directly
        let type_value = ByteString::from_bytes(&[attribute_type as u8]);
        args.push(Any::from(type_value));
        
        // Convert the u32 to Int256
        let fee_value = Int256::from_u64(value as u64);
        args.push(Any::from(fee_value));
        
        let result = Runtime::call_contract(
            Policy::hash(),
            method,
            args
        );
        
        // Extract boolean value
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }
}
