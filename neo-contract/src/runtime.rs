//! Runtime support for Neo N3 smart contracts
//!
//! This module provides the necessary functions to interact with the
//! Neo N3 VM at runtime. It includes syscalls, notifications, and
//! other utilities required by smart contracts.

use crate::env::syscall;
use crate::prelude::*;
use crate::types::builtin::any::Any;
use crate::types::builtin::array::Array;
use crate::types::builtin::string::ByteString;
use alloc::string::String;
use alloc::vec::Vec;

/// Notify the blockchain of an event
pub fn notify_event<S: AsRef<str>>(event_name: S, event_data: &Array) {
    let name = event_name.as_ref();
    // Convert event name to ByteString
    let name_bs = ByteString::from(name);
    
    // Call the Neo VM syscall to notify
    syscall::system_runtime_notify(&name_bs, event_data);
}

/// Convert any value to a stack item for the Neo VM
pub fn to_stackitem<T>(value: &T) -> Any 
where 
    T: Into<Any> + Clone
{
    value.clone().into()
}

/// Convert an event struct to an array of stack items
pub fn to_event_args<T>(event: &T) -> Array {
    // This is a placeholder - the real implementation will be provided
    // by the event macro, which will generate code to convert each field
    // of the event struct to a stack item.
    Array::new()
}

/// Get the operation name from the current invocation
pub fn get_invocation_operation() -> ByteString {
    syscall::system_runtime_get_trigger().into()
}

/// Get the arguments from the current invocation
pub fn get_invocation_args() -> Array {
    syscall::system_runtime_get_notification_args()
}

/// Set the result of the current invocation
pub fn set_invocation_result(result: Any) {
    syscall::system_runtime_log(&result);
}

/// Get the current blockchain timestamp
pub fn time() -> u64 {
    syscall::system_runtime_get_time()
}

/// Get the script hash that called the current method
pub fn calling_script_hash() -> H160 {
    syscall::system_runtime_get_caller()
}

/// Check if the given script hash has witnessed the current transaction
pub fn check_witness(hash: &H160) -> bool {
    syscall::system_runtime_check_witness(hash)
}

/// Entry point for deployment
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __neo_deploy_entry() -> bool {
    true
}

/// Entry point for invocation
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __neo_invoke_entry() {
    // Default implementation delegates to the contract
    // This is replaced by the contract macro
}

/// Notify system of an event
pub fn notify<N, D>(event_name: N, event_data: D) 
where
    N: AsRef<str> + core::fmt::Display,
    D: AsRef<Array>,
{
    let name = event_name.as_ref();
    let data = event_data.as_ref();
    notify_event(name, data);
}

/// Runtime utilities for Neo N3 smart contracts
pub struct Runtime;

impl Runtime {
    /// Get the current blockchain timestamp
    pub fn time() -> u64 {
        time()
    }
    
    /// Get the script hash that called the current method
    pub fn calling_script_hash() -> H160 {
        calling_script_hash()
    }
    
    /// Get the script hash of the current executing contract
    pub fn executing_script_hash() -> H160 {
        // Call the syscall
        syscall::system_runtime_get_caller()
    }
    
    /// Check if the given script hash has witnessed the current transaction
    pub fn check_witness(hash: &H160) -> bool {
        check_witness(hash)
    }
    
    /// Log a message to the Neo VM
    pub fn log<S: AsRef<str>>(message: S) {
        let any = Any::from(ByteString::from(message.as_ref()));
        syscall::system_runtime_log(&any);
    }
    
    /// Notify the blockchain of an event
    pub fn notify_with_args<S: AsRef<str>>(event_name: S, args: &Array) {
        let name = event_name.as_ref();
        notify_event(name, args);
    }
    
    /// Get the current platform trigger type
    pub fn get_trigger() -> u8 {
        0
    }
    
    /// Call a smart contract
    pub fn call_contract(script_hash: H160, method: ByteString, args: Array) -> Any {
        // This is a mock implementation - the real implementation would use a syscall
        // to call another contract
        Any::null()
    }
    
    /// Get remaining gas
    pub fn gas_left() -> u64 {
        // This is a mock implementation - the real implementation would use a syscall
        // to get the remaining gas
        1000000
    }
    
    /// Get the current block hash
    pub fn current_block_hash() -> H256 {
        // Mock implementation
        H256::zero()
    }
    
    /// Update the current contract
    pub fn update<S, M, D>(script: S, manifest: M, data: D) -> bool
    where
        S: AsRef<[u8]>,
        M: AsRef<[u8]>,
        D: AsRef<[u8]>,
    {
        // Mock implementation
        true
    }
    
    /// Check if an address is a contract
    pub fn is_contract(address: &H160) -> bool {
        // Mock implementation
        false
    }
    
    /// Notify system of an event with standard interface
    pub fn notify<S: AsRef<str>>(event_name: S, event_data: &Array) {
        let name = event_name.as_ref();
        notify_event(name, event_data);
    }
    
    /// Log a message to the Neo VM (specialized for string literals)
    pub fn log_str(message: &str) {
        let any = Any::from(ByteString::from(message));
        syscall::system_runtime_log(&any);
    }
} 