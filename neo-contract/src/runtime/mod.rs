// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::builtin::{H160, H256, ByteString, Array, Any, Int256};
use crate::CallFlags;
use crate::types::context::StorageContext;

/// Runtime provides access to the Neo runtime
pub struct Runtime;

impl Runtime {
    /// Get the executing script hash
    pub fn executing_script_hash() -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            let h160 = crate::env::syscall_non_wasm::system_runtime_executing_script_hash();
            // Convert from types::builtin::H160 to builtin::H160
            H160::try_from(&h160.0[..]).unwrap()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            let h160 = crate::env::contract::native_executing_script_hash();
            // Convert from types::builtin::H160 to builtin::H160
            H160::try_from(h160.0.as_slice()).unwrap()
        }
    }

    /// Get the calling script hash
    pub fn calling_script_hash() -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            let h160 = crate::env::syscall_non_wasm::system_runtime_calling_script_hash();
            // Convert from types::builtin::H160 to builtin::H160
            H160::try_from(&h160.0[..]).unwrap()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            let h160 = crate::env::contract::native_calling_script_hash();
            // Convert from types::builtin::H160 to builtin::H160
            H160::try_from(h160.0.as_slice()).unwrap()
        }
    }

    /// Check if the witness is valid
    pub fn check_witness(hash: &H160) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            // Convert from builtin::H160 to types::builtin::H160
            let h160 = crate::types::builtin::h160::H160(hash.as_bytes().try_into().unwrap());
            crate::env::syscall_non_wasm::system_runtime_check_witness(h160)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            // Convert from builtin::H160 to types::builtin::H160
            let h160 = crate::types::builtin::h160::H160(hash.as_bytes().try_into().unwrap());
            crate::env::contract::native_check_witness(h160)
        }
    }

    /// Notify an event
    pub fn notify(event_name: &ByteString, args: &Array<Any>) {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            // Convert from builtin::ByteString to types::builtin::ByteString
            let event_name_converted = crate::types::builtin::string::ByteString(event_name.to_vec());
            // Convert from builtin::Array<Any> to types::builtin::Array<Any>
            let args_converted = crate::types::builtin::array::Array::<crate::types::builtin::any::Any>::new();
            // TODO: Convert args properly
            crate::env::syscall_non_wasm::system_runtime_notify(event_name_converted, args_converted)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            // Convert from builtin::ByteString to types::builtin::ByteString
            let event_name_converted = crate::types::builtin::string::ByteString(event_name.to_vec());
            
            // Convert from builtin::Array<Any> to types::builtin::Array<Any>
            let mut args_converted = crate::types::builtin::array::Array::<crate::types::builtin::any::Any>::new();
            // TODO: Implement proper conversion between Array types
            
            crate::env::syscall::system_runtime_notify(event_name_converted, args_converted)
        }
    }

    /// Call a contract
    pub fn call_contract(hash: H160, method: ByteString, args: Array<Any>) -> Any {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            // Convert types for system_contract_call
            let hash_converted = crate::types::builtin::h160::H160(hash.as_bytes().try_into().unwrap());
            let method_converted = crate::types::builtin::string::ByteString(method.to_vec());
            let args_converted = crate::types::builtin::array::Array::<crate::types::builtin::any::Any>::new();
            // TODO: Convert args properly
            
            let result = crate::env::syscall_non_wasm::system_contract_call(
                hash_converted, 
                method_converted, 
                CallFlags::All, 
                args_converted
            );
            
            // Convert result back to builtin::Any
            crate::builtin::any::Any::default()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            // Convert from builtin::H160 to types::builtin::H160
            let hash_converted = crate::types::builtin::h160::H160(hash.as_bytes().try_into().unwrap());
            
            // Convert from builtin::ByteString to types::builtin::ByteString
            let method_converted = crate::types::builtin::string::ByteString(method.to_vec());
            
            // Convert from builtin::Array<Any> to types::builtin::Array<Any>
            let args_converted = crate::types::builtin::array::Array::<crate::types::builtin::any::Any>::new();
            // TODO: Implement proper conversion between Array types
            
            let result = crate::env::contract::native_contract_call(
                hash_converted, 
                method_converted, 
                CallFlags::All, 
                args_converted
            );
            
            // Convert from types::builtin::Any to builtin::Any
            crate::builtin::any::Any::default()
        }
    }
    
    /// Call a contract with specific flags
    pub fn call_contract_with_flags(hash: H160, method: ByteString, flags: CallFlags, args: Array<Any>) -> Any {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            // Convert types for system_contract_call
            let hash_converted = crate::types::builtin::h160::H160(hash.as_bytes().try_into().unwrap());
            let method_converted = crate::types::builtin::string::ByteString(method.to_vec());
            let args_converted = crate::types::builtin::array::Array::<crate::types::builtin::any::Any>::new();
            // TODO: Convert args properly
            
            let result = crate::env::syscall_non_wasm::system_contract_call(
                hash_converted, 
                method_converted, 
                flags, 
                args_converted
            );
            
            // Convert result back to builtin::Any
            crate::builtin::any::Any::default()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            // Convert from builtin::H160 to types::builtin::H160
            let hash_converted = crate::types::builtin::h160::H160(hash.as_bytes().try_into().unwrap());
            
            // Convert from builtin::ByteString to types::builtin::ByteString
            let method_converted = crate::types::builtin::string::ByteString(method.to_vec());
            
            // Convert from builtin::Array<Any> to types::builtin::Array<Any>
            let args_converted = crate::types::builtin::array::Array::<crate::types::builtin::any::Any>::new();
            // TODO: Implement proper conversion between Array types
            
            let result = crate::env::contract::native_contract_call(
                hash_converted, 
                method_converted, 
                flags, 
                args_converted
            );
            
            // Convert from types::builtin::Any to builtin::Any
            crate::builtin::any::Any::default()
        }
    }
    
    /// Get the storage context
    pub fn storage_context() -> StorageContext {
        StorageContext::new()
    }
    
    /// Get a value from storage
    pub fn storage_get(context: StorageContext, key: &[u8]) -> Option<Vec<u8>> {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            crate::env::syscall_non_wasm::system_storage_get(context, key.into())
                .0
                .into()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe {
            crate::env::syscall::system_storage_get(context, key.into())
                .0
                .into()
        }
    }
    
    /// Put a value in storage
    pub fn storage_put(context: StorageContext, key: &[u8], value: &[u8]) {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            crate::env::syscall_non_wasm::system_storage_put(context, key.into(), value.into())
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe {
            crate::env::syscall::system_storage_put(context, key.into(), value.into())
        }
    }
    
    /// Delete a value from storage
    pub fn storage_delete(context: StorageContext, key: &[u8]) {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            crate::env::syscall_non_wasm::system_storage_delete(context, key.into())
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe {
            crate::env::syscall::system_storage_delete(context, key.into())
        }
    }
    
    /// Find values in storage
    pub fn storage_find(context: StorageContext, prefix: &[u8]) -> crate::storage::StorageIterator {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            let iter = crate::env::syscall_non_wasm::system_storage_find(context, prefix.into());
            crate::storage::StorageIterator::new(iter)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe {
            let iter = crate::env::syscall::system_storage_find(context, prefix.into(), crate::FindOptions::RemovePrefix);
            crate::storage::StorageIterator::new(iter)
        }
    }
    
    /// Assert a condition
    pub fn assert(condition: bool, msg: Option<&str>) {
        if !condition {
            if let Some(message) = msg {
                panic!("Assertion failed: {}", message);
            } else {
                panic!("Assertion failed");
            }
        }
    }
    
    /// Get the current platform
    pub fn platform() -> ByteString {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            let platform_str = crate::env::syscall_non_wasm::system_runtime_platform();
            // Convert from types::builtin::ByteString to builtin::ByteString
            ByteString::from(platform_str.0)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            let platform_str = crate::env::syscall::system_runtime_platform();
            // Convert from types::builtin::ByteString to builtin::ByteString
            ByteString::from(platform_str.0)
        }
    }
    
    /// Get the current trigger type
    pub fn trigger() -> u32 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_runtime_trigger()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_runtime_trigger()
        }
    }
    
    /// Get the current timestamp
    pub fn time() -> u64 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_runtime_time()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_runtime_time()
        }
    }
    
    /// Get the gas left
    pub fn gas_left() -> Int256 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            let gas = crate::env::syscall_non_wasm::system_runtime_gas_left();
            Int256::from_i64(gas)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            let gas = crate::env::syscall::system_runtime_gas_left();
            // Convert from types::builtin::Int256 to builtin::Int256
            Int256::from_i64(gas.0 as i64)
        }
    }
    
    /// Get the current network ID
    pub fn network_id() -> i32 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_runtime_get_network()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_runtime_get_network()
        }
    }
    
    /// Get random number
    pub fn get_random() -> u64 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_runtime_get_random()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_runtime_get_random()
        }
    }
    
    /// Check if the hash is a contract
    pub fn is_contract(hash: &H160) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            // Convert from builtin::H160 to types::builtin::H160
            let h160 = crate::types::builtin::h160::H160(hash.as_bytes().try_into().unwrap());
            crate::env::syscall_non_wasm::system_contract_is_contract(h160)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            // Convert from builtin::H160 to types::builtin::H160
            let hash_converted = crate::types::builtin::h160::H160(hash.as_bytes().try_into().unwrap());
            crate::env::syscall::system_contract_is_contract(hash_converted)
        }
    }
    
    /// Update the contract
    pub fn update(script: ByteString, manifest: ByteString, data: Any) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            let script_converted = crate::types::builtin::string::ByteString(script.to_vec());
            let manifest_converted = crate::types::builtin::string::ByteString(manifest.to_vec());
            let data_converted = crate::types::builtin::any::Any::default(); // TODO: Implement proper conversion
            crate::env::syscall_non_wasm::system_contract_update(script_converted, manifest_converted, data_converted)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            // Convert from builtin::ByteString to types::builtin::ByteString
            let script_converted = crate::types::builtin::string::ByteString(script.to_vec());
            let manifest_converted = crate::types::builtin::string::ByteString(manifest.to_vec());
            
            // Convert from builtin::Any to types::builtin::Any
            let data_converted = crate::types::builtin::any::Any::default(); // TODO: Implement proper conversion
            
            crate::env::syscall::system_contract_update(script_converted, manifest_converted, data_converted)
        }
    }
    
    /// Destroy the contract
    pub fn destroy() {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_contract_destroy()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_contract_destroy()
        }
    }
    
    /// Log a message to the VM
    pub fn log(message: &ByteString) {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            let message_converted = crate::types::builtin::string::ByteString(message.to_vec());
            crate::env::syscall_non_wasm::system_runtime_log(message_converted)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            // Convert from builtin::ByteString to types::builtin::ByteString
            let message_converted = crate::types::builtin::string::ByteString(message.to_vec());
            crate::env::syscall::system_runtime_log(message_converted)
        }
    }
    
    /// Get the notification from a transaction
    pub fn get_notifications(hash: &H160) -> Array<Any> {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            let hash_converted = crate::types::builtin::h160::H160(hash.as_bytes().try_into().unwrap());
            let result = crate::env::syscall_non_wasm::system_runtime_get_notifications(hash_converted);
            // Convert from types::builtin::Array to builtin::Array
            let mut converted_array = Array::<Any>::new();
            // TODO: Implement proper conversion between Array types
            converted_array
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            // Convert from builtin::H160 to types::builtin::H160
            let hash_converted = crate::types::builtin::h160::H160(hash.as_bytes().try_into().unwrap());
            
            let result = crate::env::syscall::system_runtime_get_notifications(hash_converted);
            
            // Convert from types::builtin::Array<Any> to builtin::Array<Any>
            let mut converted_array = Array::<Any>::new();
            // TODO: Implement proper conversion between Array types
            
            converted_array
        }
    }
    
    /// Enter the native contract context
    pub fn enter_script(script_hash: &H160) {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            let hash_converted = crate::types::builtin::h160::H160(script_hash.as_bytes().try_into().unwrap());
            crate::env::syscall_non_wasm::system_runtime_enter_script(hash_converted)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            // Convert from builtin::H160 to types::builtin::H160
            let hash_converted = crate::types::builtin::h160::H160(script_hash.as_bytes().try_into().unwrap());
            crate::env::syscall::system_runtime_enter_script(hash_converted)
        }
    }
    
    /// Get the invocation counter
    pub fn get_invocation_counter() -> i32 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_runtime_get_invocation_counter()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_runtime_get_invocation_counter()
        }
    }
    
    /// Find values in storage with options
    pub fn storage_find_with_options(context: StorageContext, prefix: &[u8], options: crate::FindOptions) -> crate::storage::StorageIterator {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            let iter = crate::env::syscall_non_wasm::system_storage_find_with_options(context, prefix.into(), options);
            crate::storage::StorageIterator::new(iter)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe {
            let iter = crate::env::syscall::system_storage_find_with_options(context, prefix.into(), options);
            crate::storage::StorageIterator::new(iter)
        }
    }

    /// Get the storage context for the current contract
    pub fn get_storage_context() -> StorageContext {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_storage_get_context()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_storage_get_context()
        }
    }

    /// Get the storage context for the calling contract
    pub fn get_read_only_storage_context() -> StorageContext {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_storage_get_read_only_context()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_storage_get_read_only_context()
        }
    }

    /// Verify signature with ECDSA
    pub fn verify_with_ecdsa(message: &ByteString, pubkey: &ByteString, signature: &ByteString, curve: u32) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            let message_converted = crate::types::builtin::string::ByteString(message.to_vec());
            let pubkey_converted = crate::types::builtin::string::ByteString(pubkey.to_vec());
            let signature_converted = crate::types::builtin::string::ByteString(signature.to_vec());
            crate::env::syscall_non_wasm::system_crypto_verify_with_ecdsa(
                message_converted, 
                pubkey_converted, 
                signature_converted, 
                curve
            )
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            // Convert from builtin::ByteString to types::builtin::ByteString
            let message_converted = crate::types::builtin::string::ByteString(message.to_vec());
            let pubkey_converted = crate::types::builtin::string::ByteString(pubkey.to_vec());
            let signature_converted = crate::types::builtin::string::ByteString(signature.to_vec());
            
            crate::env::syscall::system_crypto_verify_with_ecdsa(
                message_converted, 
                pubkey_converted, 
                signature_converted, 
                curve
            )
        }
    }

    /// Calculate SHA256 hash
    pub fn sha256(data: &ByteString) -> H256 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            let data_converted = crate::types::builtin::string::ByteString(data.to_vec());
            let result = crate::env::syscall_non_wasm::system_crypto_sha256(data_converted);
            // Convert from types::builtin::H256 to builtin::H256
            H256::try_from(&result.0[..]).unwrap()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            // Convert from builtin::ByteString to types::builtin::ByteString
            let data_converted = crate::types::builtin::string::ByteString(data.to_vec());
            
            let result = crate::env::syscall::system_crypto_sha256(data_converted);
            
            // Convert from types::builtin::H256 to builtin::H256
            H256::try_from(result.0.as_slice()).unwrap()
        }
    }

    /// Calculate RIPEMD160 hash
    pub fn ripemd160(data: &ByteString) -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            let data_converted = crate::types::builtin::string::ByteString(data.to_vec());
            let result = crate::env::syscall_non_wasm::system_crypto_ripemd160(data_converted);
            // Convert from types::builtin::H160 to builtin::H160
            H160::try_from(&result.0[..]).unwrap()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            // Convert from builtin::ByteString to types::builtin::ByteString
            let data_converted = crate::types::builtin::string::ByteString(data.to_vec());
            
            let result = crate::env::syscall::system_crypto_ripemd160(data_converted);
            
            // Convert from types::builtin::H160 to builtin::H160
            H160::try_from(result.0.as_slice()).unwrap()
        }
    }
}
