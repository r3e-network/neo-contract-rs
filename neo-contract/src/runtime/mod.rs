// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::builtin::{H160, ByteString, Array, Any};
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
    pub fn check_witness(hash: H160) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { crate::env::syscall_non_wasm::system_runtime_check_witness(hash) }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { crate::env::contract::native_check_witness(hash) }
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
    
    /// Assert a condition
    pub fn assert(condition: bool) {
        if !condition {
            panic!("Assertion failed");
        }
    }
}
