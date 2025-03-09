//! Environment module for Neo N3 contract runtime
//!
//! This module provides access to the Neo N3 contract execution environment,
//! including interop services, blockchain information, and runtime context.

use crate::prelude::*;
use crate::types::builtin::h160::H160;
use crate::types::builtin::h256::H256;
use crate::types::builtin::string::ByteString as ByteArray;
use crate::call_flags::CallFlags;

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Module for syscalls in non-WASM environments
pub mod syscall_non_wasm;

/// Module for syscalls in WASM environments
pub mod syscall;

/// Contract module
pub mod contract;

/// Runtime module provides access to the execution context of the contract
pub mod runtime {
    use super::*;

    /// Get the execution trigger type of the contract
    pub fn trigger_type() -> u8 {
        unsafe {
            extern "C" {
                fn neo_get_trigger_type() -> u8;
            }
            neo_get_trigger_type()
        }
    }

    /// Get the timestamp of the current block
    pub fn time() -> u64 {
        unsafe {
            extern "C" {
                fn neo_runtime_get_time() -> u64;
            }
            neo_runtime_get_time()
        }
    }

    /// Get the invocation counter of the current execution
    pub fn invocation_counter() -> u32 {
        unsafe {
            extern "C" {
                fn neo_runtime_get_invocation_counter() -> u32;
            }
            neo_runtime_get_invocation_counter()
        }
    }

    /// Get the contract hash of the current execution
    pub fn current_contract() -> H160 {
        unsafe {
            extern "C" {
                fn neo_runtime_get_current_contract() -> H160;
            }
            neo_runtime_get_current_contract()
        }
    }

    /// Get hash of the script that initiated the current execution
    pub fn calling_script_hash() -> H160 {
        unsafe {
            extern "C" {
                fn neo_runtime_get_calling_script_hash() -> H160;
            }
            neo_runtime_get_calling_script_hash()
        }
    }

    /// Get the hash of the current executing script
    pub fn executing_script_hash() -> H160 {
        unsafe {
            extern "C" {
                fn neo_runtime_get_executing_script_hash() -> H160;
            }
            neo_runtime_get_executing_script_hash()
        }
    }

    /// Get the script container of the current execution (transaction)
    pub fn script_container() -> H256 {
        unsafe {
            extern "C" {
                fn neo_runtime_get_script_container() -> H256;
            }
            neo_runtime_get_script_container()
        }
    }

    /// Log a message during contract execution
    pub fn log(message: &str) {
        unsafe {
            extern "C" {
                fn neo_runtime_log(message_ptr: *const u8, message_len: usize);
            }
            neo_runtime_log(message.as_ptr(), message.len());
        }
    }

    /// Create a notification event during contract execution
    pub fn notify(event_name: &str, arg: &[u8]) {
        unsafe {
            extern "C" {
                fn neo_runtime_notify(
                    event_name_ptr: *const u8,
                    event_name_len: usize,
                    arg_ptr: *const u8,
                    arg_len: usize,
                );
            }
            neo_runtime_notify(
                event_name.as_ptr(),
                event_name.len(),
                arg.as_ptr(),
                arg.len(),
            );
        }
    }

    /// Check if the given account has witnessed the current execution
    pub fn check_witness(account: &H160) -> bool {
        unsafe {
            extern "C" {
                fn neo_runtime_check_witness(account_ptr: *const u8) -> bool;
            }
            neo_runtime_check_witness(account.as_ptr())
        }
    }

    /// Get the current platform
    pub fn platform() -> ByteArray {
        unsafe {
            extern "C" {
                fn neo_runtime_platform() -> ByteArray;
            }
            neo_runtime_platform()
        }
    }

    /// Get the network fee consumed
    pub fn gas_consumed() -> i64 {
        unsafe {
            extern "C" {
                fn neo_runtime_gas_consumed() -> i64;
            }
            neo_runtime_gas_consumed()
        }
    }

    /// Abort the execution with a message
    pub fn abort(message: &str) -> ! {
        unsafe {
            extern "C" {
                fn neo_runtime_abort(message_ptr: *const u8, message_len: usize) -> !;
            }
            neo_runtime_abort(message.as_ptr(), message.len())
        }
    }

    /// Get the caller of the current invocation
    pub fn caller() -> H160 {
        unsafe {
            extern "C" {
                fn neo_runtime_caller() -> H160;
            }
            neo_runtime_caller()
        }
    }
}

/// Blockchain module provides access to blockchain data and operations
pub mod blockchain {
    use super::*;

    /// Get current blockchain height
    pub fn height() -> u32 {
        unsafe {
            extern "C" {
                fn neo_blockchain_get_height() -> u32;
            }
            neo_blockchain_get_height()
        }
    }

    /// Get block by height
    pub fn get_block(height: u32) -> Option<Block> {
        unsafe {
            extern "C" {
                fn neo_blockchain_get_block(height: u32) -> *const u8;
            }
            let block_ptr = neo_blockchain_get_block(height);
            if block_ptr.is_null() {
                None
            } else {
                // In a real implementation, we would deserialize the block data
                Some(Block { height })
            }
        }
    }

    /// Get transaction by hash
    pub fn get_transaction(hash: &H256) -> Option<Transaction> {
        unsafe {
            extern "C" {
                fn neo_blockchain_get_transaction(hash_ptr: *const u8) -> *const u8;
            }
            let tx_ptr = neo_blockchain_get_transaction(hash.as_ptr());
            if tx_ptr.is_null() {
                None
            } else {
                // In a real implementation, we would deserialize the transaction data
                Some(Transaction { hash: *hash })
            }
        }
    }

    /// Get contract by hash
    pub fn get_contract(hash: &H160) -> Option<Contract> {
        unsafe {
            extern "C" {
                fn neo_blockchain_get_contract(hash_ptr: *const u8) -> *const u8;
            }
            let contract_ptr = neo_blockchain_get_contract(hash.as_ptr());
            if contract_ptr.is_null() {
                None
            } else {
                // In a real implementation, we would deserialize the contract data
                Some(Contract { hash: *hash })
            }
        }
    }

    /// Placeholder struct for Block
    pub struct Block {
        pub height: u32,
    }

    /// Placeholder struct for Transaction
    pub struct Transaction {
        pub hash: H256,
    }

    /// Placeholder struct for Contract
    pub struct Contract {
        pub hash: H160,
    }
}

/// Storage module provides access to contract storage
pub mod storage {
    use super::*;
    use crate::types::context::StorageContext;

    /// Get the current storage context
    pub fn current_context() -> StorageContext {
        StorageContext::current()
    }

    /// Get value from storage
    pub fn get(context: &StorageContext, key: &[u8]) -> Option<ByteArray> {
        unsafe {
            extern "C" {
                fn neo_storage_get(
                    context_ptr: *const u8,
                    key_ptr: *const u8,
                    key_len: usize,
                ) -> *const u8;
            }
            let value_ptr = neo_storage_get(
                context.as_ptr(),
                key.as_ptr(),
                key.len(),
            );
            if value_ptr.is_null() {
                None
            } else {
                // In a real implementation, we would deserialize the value data
                Some(ByteArray::from_raw(value_ptr))
            }
        }
    }

    /// Put value into storage
    pub fn put(context: &StorageContext, key: &[u8], value: &[u8]) {
        unsafe {
            extern "C" {
                fn neo_storage_put(
                    context_ptr: *const u8,
                    key_ptr: *const u8,
                    key_len: usize,
                    value_ptr: *const u8,
                    value_len: usize,
                );
            }
            neo_storage_put(
                context.as_ptr(),
                key.as_ptr(),
                key.len(),
                value.as_ptr(),
                value.len(),
            );
        }
    }

    /// Delete value from storage
    pub fn delete(context: &StorageContext, key: &[u8]) {
        unsafe {
            extern "C" {
                fn neo_storage_delete(
                    context_ptr: *const u8,
                    key_ptr: *const u8,
                    key_len: usize,
                );
            }
            neo_storage_delete(
                context.as_ptr(),
                key.as_ptr(),
                key.len(),
            );
        }
    }

    /// Find values in storage with a prefix
    pub fn find(context: &StorageContext, prefix: &[u8]) -> StorageIterator {
        unsafe {
            extern "C" {
                fn neo_storage_find(
                    context_ptr: *const u8,
                    prefix_ptr: *const u8,
                    prefix_len: usize,
                ) -> *const u8;
            }
            let iterator_ptr = neo_storage_find(
                context.as_ptr(),
                prefix.as_ptr(),
                prefix.len(),
            );
            StorageIterator {
                ptr: iterator_ptr,
            }
        }
    }

    /// Iterator for storage entries
    pub struct StorageIterator {
        ptr: *const u8,
    }

    impl StorageIterator {
        /// Check if the iterator has more entries
        pub fn has_next(&self) -> bool {
            unsafe {
                extern "C" {
                    fn neo_iterator_has_next(iterator_ptr: *const u8) -> bool;
                }
                neo_iterator_has_next(self.ptr)
            }
        }

        /// Get the next key-value pair
        pub fn next(&self) -> Option<(ByteArray, ByteArray)> {
            if !self.has_next() {
                return None;
            }
            
            unsafe {
                extern "C" {
                    fn neo_iterator_next(
                        iterator_ptr: *const u8,
                        key_out: *mut *const u8,
                        value_out: *mut *const u8,
                    ) -> bool;
                }
                
                let mut key_ptr: *const u8 = core::ptr::null();
                let mut value_ptr: *const u8 = core::ptr::null();
                
                let success = neo_iterator_next(self.ptr, &mut key_ptr, &mut value_ptr);
                
                if success && !key_ptr.is_null() && !value_ptr.is_null() {
                    Some((
                        ByteArray::from_raw(key_ptr),
                        ByteArray::from_raw(value_ptr),
                    ))
                } else {
                    None
                }
            }
        }
    }
}

/// Helper module for converting types to Neo VM values
pub mod into_val {
    use super::*;
    
    /// Trait for converting Rust types to Neo VM values
    pub trait IntoVal {
        /// Convert self to a Neo VM value
        fn into_val(&self) -> ByteArray;
    }
    
    // Implementation for common types would go here
    impl IntoVal for i32 {
        fn into_val(&self) -> ByteArray {
            // Placeholder implementation
            ByteArray::from(self.to_le_bytes().to_vec())
        }
    }
    
    impl IntoVal for String {
        fn into_val(&self) -> ByteArray {
            // Placeholder implementation
            ByteArray::from(self.as_bytes().to_vec())
        }
    }
    
    impl IntoVal for H160 {
        fn into_val(&self) -> ByteArray {
            // Placeholder implementation
            ByteArray::from(self.as_bytes().to_vec())
        }
    }
    
    // More implementations would be needed for a complete framework
}
