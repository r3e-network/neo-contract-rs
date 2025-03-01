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
        unsafe { crate::env::syscall_non_wasm::system_runtime_executing_script_hash() }
    }

    /// Get the calling script hash
    pub fn calling_script_hash() -> H160 {
        unsafe { crate::env::syscall_non_wasm::system_runtime_calling_script_hash() }
    }

    /// Check if the witness is valid
    pub fn check_witness(hash: H160) -> bool {
        unsafe { crate::env::syscall_non_wasm::system_runtime_check_witness(hash) }
    }

    /// Notify an event
    pub fn notify(event_name: &ByteString, args: &Array<Any>) {
        unsafe { crate::env::syscall_non_wasm::system_runtime_notify(event_name.clone(), args.clone()) }
    }

    /// Call a contract
    pub fn call_contract(hash: H160, method: ByteString, args: Array<Any>) -> Any {
        unsafe { 
            crate::env::syscall_non_wasm::system_contract_call(
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
        unsafe {
            crate::env::syscall_non_wasm::system_storage_get(context, key)
        }
    }
    
    /// Put a value in storage
    pub fn storage_put(context: StorageContext, key: &[u8], value: &[u8]) {
        unsafe {
            crate::env::syscall_non_wasm::system_storage_put(context, key, value)
        }
    }
    
    /// Delete a value from storage
    pub fn storage_delete(context: StorageContext, key: &[u8]) {
        unsafe {
            crate::env::syscall_non_wasm::system_storage_delete(context, key)
        }
    }
    
    /// Assert a condition
    pub fn assert(condition: bool) {
        if !condition {
            panic!("Assertion failed");
        }
    }
}
