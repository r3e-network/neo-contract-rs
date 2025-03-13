//! Environment module for Neo N3 contract runtime
//!
//! This module provides access to the Neo N3 contract execution environment,
//! including interop services, blockchain information, and runtime context.

use crate::types::builtin::h160::H160;
use crate::types::builtin::h256::H256;
use crate::types::builtin::string::ByteString;
use alloc::vec;

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
                fn neo_runtime_get_calling_script_hash(output_ptr: *mut u8) -> usize;
            }

            // Allocate buffer for the H160 (20 bytes)
            let mut buffer = [0u8; 20];

            // Call the syscall to get the calling script hash
            let size = neo_runtime_get_calling_script_hash(buffer.as_mut_ptr());

            if size == 20 {
                // Convert the buffer to H160
                H160::from_slice(&buffer)
            } else {
                // Return a default H160 (all zeros) if the call fails
                H160::default()
            }
        }
    }

    /// Get the hash of the current executing script
    pub fn executing_script_hash() -> H160 {
        unsafe {
            extern "C" {
                fn neo_runtime_get_executing_script_hash(output_ptr: *mut u8) -> usize;
            }

            // Allocate buffer for the H160 (20 bytes)
            let mut buffer = [0u8; 20];

            // Call the syscall to get the executing script hash
            let size = neo_runtime_get_executing_script_hash(buffer.as_mut_ptr());

            if size == 20 {
                // Convert the buffer to H160
                H160::from_slice(&buffer)
            } else {
                // Return a default H160 (all zeros) if the call fails
                H160::default()
            }
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
    pub fn log(message: &ByteString) {
        unsafe {
            extern "C" {
                fn neo_runtime_log(message_ptr: *const u8, message_len: usize);
            }
            neo_runtime_log(message.as_ptr(), message.len());
        }
    }

    /// Create a notification event during contract execution
    ///
    /// In Neo N3, notifications should use ByteString for the event name
    /// and Array<Any> for parameters. Example usage:
    /// ```
    /// let event_name = ByteString::from("Transfer");
    /// let mut event_data = Array::<Any>::new();
    /// event_data.push(Any::from(from_addr));
    /// event_data.push(Any::from(to_addr));
    /// event_data.push(Any::from(amount));
    /// notify(&event_name, &event_data.serialize());
    /// ```
    pub fn notify(event_name: &ByteString, args_data: &[u8]) {
        unsafe {
            extern "C" {
                fn neo_runtime_notify(
                    event_name_ptr: *const u8,
                    event_name_len: usize,
                    arg_ptr: *const u8,
                    arg_len: usize,
                );
            }
            neo_runtime_notify(event_name.as_ptr(), event_name.len(), args_data.as_ptr(), args_data.len());
        }
    }

    /// Check if the given account has witnessed the current execution
    pub fn check_witness(account: &H160) -> bool {
        unsafe {
            extern "C" {
                fn neo_runtime_check_witness(hash_ptr: *const u8, hash_len: usize) -> bool;
            }
            neo_runtime_check_witness(account.as_ptr(), account.as_bytes().len())
        }
    }

    /// Get the current platform
    pub fn platform() -> ByteString {
        unsafe {
            extern "C" {
                // Using raw pointer for FFI-safe return
                fn neo_runtime_platform() -> *const u8;
                fn neo_runtime_platform_length() -> usize;
            }

            // Get the platform string pointer and length
            let ptr = neo_runtime_platform();
            let len = neo_runtime_platform_length();

            // Convert to ByteString safely
            if ptr.is_null() || len == 0 {
                ByteString::default()
            } else {
                let slice = core::slice::from_raw_parts(ptr, len);
                ByteString::from(slice)
            }
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
                fn neo_blockchain_get_block(
                    hash_ptr: *const u8,
                    hash_len: usize,
                    output_ptr: *mut u8,
                    output_len: usize,
                ) -> usize;
            }

            // Convert height to bytes (u32 -> [u8; 4])
            let height_bytes = height.to_le_bytes();

            // First call with null to get the required buffer size
            let required_size =
                neo_blockchain_get_block(height_bytes.as_ptr(), height_bytes.len(), core::ptr::null_mut(), 0);

            if required_size == 0 {
                return None;
            }

            // Allocate buffer of the required size
            let mut buffer = vec![0u8; required_size];

            // Second call to get the actual data
            let actual_size =
                neo_blockchain_get_block(height_bytes.as_ptr(), height_bytes.len(), buffer.as_mut_ptr(), buffer.len());

            if actual_size == 0 {
                None
            } else {
                // Deserialize the binary data into a Block structure
                // For this implementation, we'll create a basic Block with just the height
                // but in a full implementation, you would parse all block fields from buffer
                Some(Block {
                    height,
                    // Additional fields would be parsed from buffer
                    // timestamp: read_u64_from_buffer(&buffer),
                    // transactions: read_transactions_from_buffer(&buffer),
                    // etc.
                })
            }
        }
    }

    /// Get transaction by hash
    pub fn get_transaction(hash: &H256) -> Option<Transaction> {
        unsafe {
            extern "C" {
                fn neo_blockchain_get_transaction(
                    hash_ptr: *const u8,
                    hash_len: usize,
                    output_ptr: *mut u8,
                    output_len: usize,
                ) -> usize;
            }

            // First call with null to get the required buffer size
            let required_size =
                neo_blockchain_get_transaction(hash.as_ptr(), hash.as_bytes().len(), core::ptr::null_mut(), 0);

            if required_size == 0 {
                return None;
            }

            // Allocate buffer of the required size
            let mut buffer = vec![0u8; required_size];

            // Second call to get the actual data
            let actual_size =
                neo_blockchain_get_transaction(hash.as_ptr(), hash.as_bytes().len(), buffer.as_mut_ptr(), buffer.len());

            if actual_size == 0 {
                None
            } else {
                // Deserialize the binary data into a Transaction structure
                // For this implementation, we'll create a basic Transaction with just the hash
                // but in a full implementation, you would parse all transaction fields from buffer
                Some(Transaction {
                    hash: *hash,
                    // Additional fields would be parsed from buffer
                    // type: decode_tx_type_from_buffer(&buffer),
                    // sender: decode_address_from_buffer(&buffer),
                    // etc.
                })
            }
        }
    }

    /// Get contract by hash
    pub fn get_contract(hash: &H160) -> Option<Contract> {
        unsafe {
            extern "C" {
                fn neo_blockchain_get_contract(
                    hash_ptr: *const u8,
                    hash_len: usize,
                    output_ptr: *mut u8,
                    output_len: usize,
                ) -> usize;
            }

            // First call with null to get the required buffer size
            let required_size =
                neo_blockchain_get_contract(hash.as_ptr(), hash.as_bytes().len(), core::ptr::null_mut(), 0);

            if required_size == 0 {
                return None;
            }

            // Allocate buffer of the required size
            let mut buffer = vec![0u8; required_size];

            // Second call to get the actual data
            let actual_size =
                neo_blockchain_get_contract(hash.as_ptr(), hash.as_bytes().len(), buffer.as_mut_ptr(), buffer.len());

            if actual_size == 0 {
                None
            } else {
                // Use the buffer to deserialize the contract data
                // In a real implementation, we would deserialize all contract fields from the buffer
                // which contains the serialized NEO contract data

                // For this implementation, we'll just create a basic Contract with the hash
                Some(Contract {
                    hash: *hash,
                    // Additional fields would be parsed from buffer
                    // name: decode_string_from_buffer(&buffer),
                    // script: decode_script_from_buffer(&buffer),
                    // etc.
                })
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
    pub fn current_context() -> StorageContext { StorageContext::current() }

    /// Get value from storage
    pub fn get(context: &StorageContext, key: &[u8]) -> Option<ByteString> {
        unsafe {
            extern "C" {
                fn neo_storage_get(
                    context_ptr: *const u8,
                    context_len: usize,
                    key_ptr: *const u8,
                    key_len: usize,
                    value_ptr: *mut u8,
                    value_len: usize,
                ) -> usize;
            }
            // Create a buffer to hold the value
            let mut buffer = [0u8; 1024]; // Using a fixed buffer size for simplicity
            let buffer_len = buffer.len();

            let value_size = neo_storage_get(
                context.as_ptr(),
                context.as_bytes().len(),
                key.as_ptr(),
                key.len(),
                buffer.as_mut_ptr(),
                buffer_len,
            );

            if value_size == 0 {
                None
            } else {
                // Convert the buffer to a byte array with the actual size
                Some(ByteString::from(&buffer[..value_size]))
            }
        }
    }

    /// Put value into storage
    pub fn put(context: &StorageContext, key: &[u8], value: &[u8]) {
        unsafe {
            extern "C" {
                fn neo_storage_put(
                    context_ptr: *const u8,
                    context_len: usize,
                    key_ptr: *const u8,
                    key_len: usize,
                    value_ptr: *const u8,
                    value_len: usize,
                );
            }
            neo_storage_put(
                context.as_ptr(),
                context.as_bytes().len(),
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
                fn neo_storage_delete(context_ptr: *const u8, context_len: usize, key_ptr: *const u8, key_len: usize);
            }
            neo_storage_delete(context.as_ptr(), context.as_bytes().len(), key.as_ptr(), key.len());
        }
    }

    /// Find values in storage with a prefix
    pub fn find(context: &StorageContext, prefix: &[u8]) -> StorageIterator {
        unsafe {
            extern "C" {
                fn neo_storage_find(
                    context_ptr: *const u8,
                    context_len: usize,
                    prefix_ptr: *const u8,
                    prefix_len: usize,
                ) -> u32;
            }
            let iterator_id =
                neo_storage_find(context.as_ptr(), context.as_bytes().len(), prefix.as_ptr(), prefix.len());
            StorageIterator { id: iterator_id }
        }
    }

    /// Iterator for storage entries
    pub struct StorageIterator {
        id: u32,
    }

    impl StorageIterator {
        /// Check if the iterator has more entries
        pub fn has_next(&self) -> bool {
            unsafe {
                extern "C" {
                    fn neo_iterator_has_next(iterator_id: u32) -> bool;
                }
                neo_iterator_has_next(self.id)
            }
        }

        /// Get the next key-value pair
        pub fn next(&self) -> Option<(ByteString, ByteString)> {
            if !self.has_next() {
                return None;
            }

            unsafe {
                extern "C" {
                    fn neo_iterator_next(iterator_id: u32) -> bool;
                    fn neo_iterator_key(iterator_id: u32, output_ptr: *mut u8, output_len: usize) -> usize;
                    fn neo_iterator_value(iterator_id: u32, output_ptr: *mut u8, output_len: usize) -> usize;
                }

                let success = neo_iterator_next(self.id);

                if success {
                    // Buffers to hold key and value
                    let mut key_buffer = [0u8; 1024];
                    let mut value_buffer = [0u8; 1024];

                    let key_size = neo_iterator_key(self.id, key_buffer.as_mut_ptr(), key_buffer.len());
                    let value_size = neo_iterator_value(self.id, value_buffer.as_mut_ptr(), value_buffer.len());

                    if key_size > 0 && value_size > 0 {
                        Some((ByteString::from(&key_buffer[..key_size]), ByteString::from(&value_buffer[..value_size])))
                    } else {
                        None
                    }
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
        fn into_val(&self) -> ByteString;
    }

    // Implementation for common types would go here
    impl IntoVal for i32 {
        fn into_val(&self) -> ByteString {
            // Placeholder implementation
            ByteString::from(self.to_le_bytes().to_vec())
        }
    }

    use alloc::string::String;

    impl IntoVal for String {
        fn into_val(&self) -> ByteString {
            // Placeholder implementation
            ByteString::from(self.as_bytes().to_vec())
        }
    }

    impl IntoVal for H160 {
        fn into_val(&self) -> ByteString {
            // Placeholder implementation
            ByteString::from(self.as_bytes().to_vec())
        }
    }

    // More implementations would be needed for a complete framework
}
