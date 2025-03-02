// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::builtin::{H160, ByteString, Array, Any, Int256};
use crate::CallFlags;
use crate::types::context::StorageContext;

/// Runtime provides access to the Neo runtime
pub struct Runtime;

impl Runtime {
    /// Get the executing script hash
    pub fn executing_script_hash() -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { crate::env::syscall_non_wasm::system_runtime_executing_script_hash() }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { crate::env::contract::native_executing_script_hash() }
    }

    /// Get the calling script hash
    pub fn calling_script_hash() -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { crate::env::syscall_non_wasm::system_runtime_calling_script_hash() }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { crate::env::contract::native_calling_script_hash() }
    }

    /// Check if the witness is valid
    pub fn check_witness(hash: &H160) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { crate::env::syscall_non_wasm::system_runtime_check_witness(hash.clone()) }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { crate::env::contract::native_check_witness(hash.clone()) }
    }

    /// Notify an event
    pub fn notify(event_name: &ByteString, args: &Array<Any>) {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { crate::env::syscall_non_wasm::system_runtime_notify(event_name.clone(), args.clone()) }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { crate::env::syscall::system_runtime_notify(event_name.clone(), args.clone()) }
    }

    /// Call a contract
    pub fn call_contract(hash: H160, method: ByteString, args: Array<Any>) -> Any {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_contract_call(
                hash, 
                method, 
                CallFlags::All, 
                args
            )
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::contract::native_contract_call(
                hash, 
                method, 
                CallFlags::All, 
                args
            )
        }
    }
    
    /// Call a contract with specific flags
    pub fn call_contract_with_flags(hash: H160, method: ByteString, flags: CallFlags, args: Array<Any>) -> Any {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_contract_call(
                hash, 
                method, 
                flags, 
                args
            )
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::contract::native_contract_call(
                hash, 
                method, 
                flags, 
                args
            )
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
            let iter = crate::env::syscall_non_wasm::system_storage_find(context, prefix.into(), crate::FindOptions::RemovePrefix);
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
            crate::env::syscall_non_wasm::system_runtime_platform()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_runtime_platform()
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
            crate::env::syscall_non_wasm::system_runtime_gas_left()
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_runtime_gas_left()
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
            crate::env::syscall_non_wasm::system_contract_is_contract(hash.clone())
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_contract_is_contract(hash.clone())
        }
    }
    
    /// Update the contract
    pub fn update(script: ByteString, manifest: ByteString, data: Any) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_contract_update(script, manifest, data)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_contract_update(script, manifest, data)
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
            crate::env::syscall_non_wasm::system_runtime_log(message.clone())
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_runtime_log(message.clone())
        }
    }
    
    /// Get the notification from a transaction
    pub fn get_notifications(hash: &H160) -> Array<Any> {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_runtime_get_notifications(hash.clone())
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_runtime_get_notifications(hash.clone())
        }
    }
    
    /// Enter the native contract context
    pub fn enter_script(script_hash: &H160) {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_runtime_enter_script(script_hash.clone())
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_runtime_enter_script(script_hash.clone())
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
            crate::env::syscall_non_wasm::system_crypto_verify_with_ecdsa(message.clone(), pubkey.clone(), signature.clone(), curve)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_crypto_verify_with_ecdsa(message.clone(), pubkey.clone(), signature.clone(), curve)
        }
    }

    /// Calculate SHA256 hash
    pub fn sha256(data: &ByteString) -> H256 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_crypto_sha256(data.clone())
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_crypto_sha256(data.clone())
        }
    }

    /// Calculate RIPEMD160 hash
    pub fn ripemd160(data: &ByteString) -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_crypto_ripemd160(data.clone())
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_crypto_ripemd160(data.clone())
        }
    }
}
