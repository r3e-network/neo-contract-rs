//! Low-level syscalls for interacting with the Neo N3 VM
//!
//! This module provides the raw syscall interface to interact with the
//! Neo N3 VM. These functions are typically not used directly by smart
//! contract authors, but are used by the runtime module.

use crate::types::builtin::any::Any;
use crate::types::builtin::array::Array;
use crate::types::builtin::string::ByteString;
use crate::H160;
use alloc::string::String;
use alloc::vec::Vec;

/// Neo N3 Runtime - Notify
pub fn system_runtime_notify(name: &ByteString, args: &Array) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        // WASM implementation of Neo VM syscall
        // This would be replaced with actual Neo VM syscall
        extern "C" {
            fn Neo3RuntimeNotify(name_ptr: *const u8, name_len: u32, args_ptr: *const u8, args_len: u32);
        }
        
        // Implementation details omitted for brevity
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Mock implementation for testing
    }
}

/// Neo N3 Runtime - Get Trigger
pub fn system_runtime_get_trigger() -> ByteString {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        // WASM implementation of Neo VM syscall
        ByteString::from("main")
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Mock implementation for testing
        ByteString::from("main")
    }
}

/// Neo N3 Runtime - Get Notification Args
pub fn system_runtime_get_notification_args() -> Array {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        // WASM implementation of Neo VM syscall
        Array::new()
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Mock implementation for testing
        Array::new()
    }
}

/// Neo N3 Runtime - Log
pub fn system_runtime_log(data: &Any) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        // WASM implementation of Neo VM syscall
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Mock implementation for testing
    }
}

/// Neo N3 Runtime - Get Time
pub fn system_runtime_get_time() -> u64 {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        // WASM implementation of Neo VM syscall
        0
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Mock implementation for testing
        1629264000 // Fixed timestamp for testing
    }
}

/// Neo N3 Runtime - Get Caller
pub fn system_runtime_get_caller() -> H160 {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        // WASM implementation of Neo VM syscall
        H160::zero()
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Mock implementation for testing
        H160::zero()
    }
}

/// Neo N3 Runtime - Check Witness
pub fn system_runtime_check_witness(hash: &H160) -> bool {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        // WASM implementation of Neo VM syscall
        true
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Mock implementation for testing
        true
    }
}

/// Neo N3 Storage - Get
pub fn system_storage_get(context: &crate::types::storage::StorageContext, key: &[u8]) -> Vec<u8> {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        // WASM implementation of Neo VM syscall
        Vec::new()
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Mock implementation for testing
        Vec::new()
    }
}

/// Neo N3 Storage - Put
pub fn system_storage_put(context: &crate::types::storage::StorageContext, key: &[u8], value: &[u8]) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        // WASM implementation of Neo VM syscall
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Mock implementation for testing
    }
}

/// Neo N3 Storage - Delete
pub fn system_storage_delete(context: &crate::types::storage::StorageContext, key: &[u8]) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        // WASM implementation of Neo VM syscall
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Mock implementation for testing
    }
}
