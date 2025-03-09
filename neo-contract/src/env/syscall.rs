//! Low-level syscalls for Neo N3 smart contracts
//!
//! This module provides raw access to the Neo VM syscalls.
//! These functions are meant to be used by the higher-level modules
//! and not directly by contract developers.

use core::mem::MaybeUninit;
use alloc::vec::Vec;
use crate::types::context::StorageContext;
use crate::types::Any;

/// Runtime syscalls

/// Get the current blockchain timestamp
pub fn runtime_get_time() -> u64 {
    // In a real implementation, this would call the actual Neo VM syscall
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_runtime_get_time() -> u64;
        }
        unsafe { neo_runtime_get_time() }
    }
    
    // For testing, return a fixed timestamp
    #[cfg(test)]
    {
        1617235200000 // April 1, 2021
    }
}

/// Check if the given hash has witnessed the current transaction
pub fn runtime_check_witness(hash: &[u8]) -> bool {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_runtime_check_witness(hash_ptr: *const u8, hash_len: usize) -> bool;
        }
        unsafe { neo_runtime_check_witness(hash.as_ptr(), hash.len()) }
    }
    
    #[cfg(test)]
    {
        // For testing, always return true
        true
    }
}

/// Get the script hash of the current executing contract
pub fn runtime_get_executing_script_hash() -> Vec<u8> {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_runtime_get_executing_script_hash(output_ptr: *mut u8) -> usize;
        }
        
        // Prepare a buffer for the result (20 bytes for script hash)
        let mut buffer = [0u8; 20];
        
        unsafe {
            let len = neo_runtime_get_executing_script_hash(buffer.as_mut_ptr());
            buffer[..len].to_vec()
        }
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy hash
        (1..=20).collect()
    }
}

/// Get the script hash of the calling contract
pub fn runtime_get_calling_script_hash() -> Vec<u8> {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_runtime_get_calling_script_hash(output_ptr: *mut u8) -> usize;
        }
        
        // Prepare a buffer for the result (20 bytes for script hash)
        let mut buffer = [0u8; 20];
        
        unsafe {
            let len = neo_runtime_get_calling_script_hash(buffer.as_mut_ptr());
            buffer[..len].to_vec()
        }
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy hash
        (21..=40).collect()
    }
}

/// Log a message to the Neo VM
pub fn runtime_log(message: &[u8]) {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_runtime_log(message_ptr: *const u8, message_len: usize);
        }
        unsafe { neo_runtime_log(message.as_ptr(), message.len()) }
    }
    
    #[cfg(test)]
    {
        // For testing, print to stdout
        let message_str = core::str::from_utf8(message).unwrap_or("[Invalid UTF-8]");
        println!("[LOG] {}", message_str);
    }
}

/// Notify an event with the given name and data
pub fn runtime_notify(event_name: &[u8], data: &[u8]) {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_runtime_notify(
                event_name_ptr: *const u8, event_name_len: usize,
                data_ptr: *const u8, data_len: usize
            );
        }
        
        unsafe {
            neo_runtime_notify(
                event_name.as_ptr(), event_name.len(),
                data.as_ptr(), data.len()
            )
        }
    }
    
    #[cfg(test)]
    {
        // For testing, print to stdout
        let event_str = core::str::from_utf8(event_name).unwrap_or("[Invalid UTF-8]");
        println!("[EVENT] {}: {:?}", event_str, data);
    }
}

/// Get the current platform trigger type
pub fn runtime_get_trigger() -> u8 {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_runtime_get_trigger() -> u8;
        }
        unsafe { neo_runtime_get_trigger() }
    }
    
    #[cfg(test)]
    {
        // For testing, return Application trigger (0x10)
        0x10
    }
}

/// Get the current network ID
pub fn runtime_get_network() -> u8 {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_runtime_get_network() -> u8;
        }
        unsafe { neo_runtime_get_network() }
    }
    
    #[cfg(test)]
    {
        // For testing, return TestNet (1)
        1
    }
}

/// Storage syscalls

/// Get a value from storage
pub fn storage_get(context: &StorageContext, key: &[u8]) -> Vec<u8> {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_storage_get(
                context_ptr: *const u8, context_len: usize,
                key_ptr: *const u8, key_len: usize,
                value_ptr: *mut u8, value_len: usize
            ) -> usize;
        }
        
        // First call with null to get the size of the result
        let value_len = unsafe {
            neo_storage_get(
                context.as_bytes().as_ptr(), context.as_bytes().len(),
                key.as_ptr(), key.len(),
                core::ptr::null_mut(), 0
            )
        };
        
        // If length is 0, key doesn't exist
        if value_len == 0 {
            return Vec::new();
        }
        
        // Allocate buffer and get the value
        let mut buffer = Vec::with_capacity(value_len);
        unsafe {
            buffer.set_len(value_len);
            neo_storage_get(
                context.as_bytes().as_ptr(), context.as_bytes().len(),
                key.as_ptr(), key.len(),
                buffer.as_mut_ptr(), value_len
            );
        }
        
        buffer
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy value
        Vec::new()
    }
}

/// Put a value into storage
pub fn storage_put(context: &StorageContext, key: &[u8], value: &[u8]) {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_storage_put(
                context_ptr: *const u8, context_len: usize,
                key_ptr: *const u8, key_len: usize,
                value_ptr: *const u8, value_len: usize
            );
        }
        
        unsafe {
            neo_storage_put(
                context.as_bytes().as_ptr(), context.as_bytes().len(),
                key.as_ptr(), key.len(),
                value.as_ptr(), value.len()
            )
        }
    }
    
    #[cfg(test)]
    {
        // For testing, do nothing
    }
}

/// Delete a value from storage
pub fn storage_delete(context: &StorageContext, key: &[u8]) {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_storage_delete(
                context_ptr: *const u8, context_len: usize,
                key_ptr: *const u8, key_len: usize
            );
        }
        
        unsafe {
            neo_storage_delete(
                context.as_bytes().as_ptr(), context.as_bytes().len(),
                key.as_ptr(), key.len()
            )
        }
    }
    
    #[cfg(test)]
    {
        // For testing, do nothing
    }
}

/// Find storage entries with a given prefix
pub fn storage_find(context: &StorageContext, prefix: &[u8]) -> u32 {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_storage_find(
                context_ptr: *const u8, context_len: usize,
                prefix_ptr: *const u8, prefix_len: usize
            ) -> u32;
        }
        
        unsafe {
            neo_storage_find(
                context.as_bytes().as_ptr(), context.as_bytes().len(),
                prefix.as_ptr(), prefix.len()
            )
        }
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy iterator ID
        1
    }
}

/// Iterator operations

/// Check if the iterator has more elements
pub fn iterator_next(iterator_id: u32) -> bool {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_iterator_next(iterator_id: u32) -> bool;
        }
        
        unsafe { neo_iterator_next(iterator_id) }
    }
    
    #[cfg(test)]
    {
        // For testing, always return false (no more elements)
        false
    }
}

/// Get the key of the current iterator element
pub fn iterator_key(iterator_id: u32) -> Vec<u8> {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_iterator_key(iterator_id: u32, output_ptr: *mut u8, output_len: usize) -> usize;
        }
        
        // First call with null to get the size of the result
        let key_len = unsafe { neo_iterator_key(iterator_id, core::ptr::null_mut(), 0) };
        
        // Allocate buffer and get the key
        let mut buffer = Vec::with_capacity(key_len);
        unsafe {
            buffer.set_len(key_len);
            neo_iterator_key(iterator_id, buffer.as_mut_ptr(), key_len);
        }
        
        buffer
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy key
        Vec::new()
    }
}

/// Get the value of the current iterator element
pub fn iterator_value(iterator_id: u32) -> Vec<u8> {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_iterator_value(iterator_id: u32, output_ptr: *mut u8, output_len: usize) -> usize;
        }
        
        // First call with null to get the size of the result
        let value_len = unsafe { neo_iterator_value(iterator_id, core::ptr::null_mut(), 0) };
        
        // Allocate buffer and get the value
        let mut buffer = Vec::with_capacity(value_len);
        unsafe {
            buffer.set_len(value_len);
            neo_iterator_value(iterator_id, buffer.as_mut_ptr(), value_len);
        }
        
        buffer
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy value
        Vec::new()
    }
}

/// Close an iterator
pub fn iterator_close(iterator_id: u32) {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_iterator_close(iterator_id: u32);
        }
        
        unsafe { neo_iterator_close(iterator_id) }
    }
    
    #[cfg(test)]
    {
        // For testing, do nothing
    }
}

/// Blockchain syscalls

/// Get the current blockchain height
pub fn blockchain_get_height() -> u32 {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_blockchain_get_height() -> u32;
        }
        
        unsafe { neo_blockchain_get_height() }
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy height
        1000000
    }
}

/// Get a block by hash
pub fn blockchain_get_block(hash: &[u8]) -> Vec<u8> {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_blockchain_get_block(
                hash_ptr: *const u8, hash_len: usize,
                output_ptr: *mut u8, output_len: usize
            ) -> usize;
        }
        
        // First call with null to get the size of the result
        let block_len = unsafe { 
            neo_blockchain_get_block(hash.as_ptr(), hash.len(), core::ptr::null_mut(), 0)
        };
        
        // If length is 0, block doesn't exist
        if block_len == 0 {
            return Vec::new();
        }
        
        // Allocate buffer and get the block
        let mut buffer = Vec::with_capacity(block_len);
        unsafe {
            buffer.set_len(block_len);
            neo_blockchain_get_block(hash.as_ptr(), hash.len(), buffer.as_mut_ptr(), block_len);
        }
        
        buffer
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy block
        Vec::new()
    }
}

/// Get a transaction by hash
pub fn blockchain_get_transaction(hash: &[u8]) -> Vec<u8> {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_blockchain_get_transaction(
                hash_ptr: *const u8, hash_len: usize,
                output_ptr: *mut u8, output_len: usize
            ) -> usize;
        }
        
        // First call with null to get the size of the result
        let tx_len = unsafe { 
            neo_blockchain_get_transaction(hash.as_ptr(), hash.len(), core::ptr::null_mut(), 0)
        };
        
        // If length is 0, transaction doesn't exist
        if tx_len == 0 {
            return Vec::new();
        }
        
        // Allocate buffer and get the transaction
        let mut buffer = Vec::with_capacity(tx_len);
        unsafe {
            buffer.set_len(tx_len);
            neo_blockchain_get_transaction(hash.as_ptr(), hash.len(), buffer.as_mut_ptr(), tx_len);
        }
        
        buffer
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy transaction
        Vec::new()
    }
}

/// Get the transaction height
pub fn blockchain_get_transaction_height(hash: &[u8]) -> u32 {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_blockchain_get_transaction_height(hash_ptr: *const u8, hash_len: usize) -> u32;
        }
        
        unsafe { neo_blockchain_get_transaction_height(hash.as_ptr(), hash.len()) }
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy height
        1000000
    }
}

/// Get a contract by hash
pub fn blockchain_get_contract(hash: &[u8]) -> Vec<u8> {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_blockchain_get_contract(
                hash_ptr: *const u8, hash_len: usize,
                output_ptr: *mut u8, output_len: usize
            ) -> usize;
        }
        
        // First call with null to get the size of the result
        let contract_len = unsafe { 
            neo_blockchain_get_contract(hash.as_ptr(), hash.len(), core::ptr::null_mut(), 0)
        };
        
        // If length is 0, contract doesn't exist
        if contract_len == 0 {
            return Vec::new();
        }
        
        // Allocate buffer and get the contract
        let mut buffer = Vec::with_capacity(contract_len);
        unsafe {
            buffer.set_len(contract_len);
            neo_blockchain_get_contract(hash.as_ptr(), hash.len(), buffer.as_mut_ptr(), contract_len);
        }
        
        buffer
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy contract
        Vec::new()
    }
}

/// Crypto syscalls

/// Verify signature using ECDSA with the given curve
pub fn crypto_verify_with_ecdsa(message: &[u8], signature: &[u8], public_key: &[u8], curve: u32) -> bool {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_crypto_verify_with_ecdsa(
                message_ptr: *const u8, message_len: usize,
                signature_ptr: *const u8, signature_len: usize,
                public_key_ptr: *const u8, public_key_len: usize,
                curve: u32
            ) -> bool;
        }
        
        unsafe {
            neo_crypto_verify_with_ecdsa(
                message.as_ptr(), message.len(),
                signature.as_ptr(), signature.len(),
                public_key.as_ptr(), public_key.len(),
                curve
            )
        }
    }
    
    #[cfg(test)]
    {
        // For testing, always return true
        true
    }
}

/// Compute SHA256 hash
pub fn crypto_sha256(data: &[u8]) -> Vec<u8> {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_crypto_sha256(
                data_ptr: *const u8, data_len: usize,
                output_ptr: *mut u8
            );
        }
        
        // SHA256 always outputs 32 bytes
        let mut buffer = [0u8; 32];
        
        unsafe {
            neo_crypto_sha256(data.as_ptr(), data.len(), buffer.as_mut_ptr());
        }
        
        buffer.to_vec()
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy hash
        vec![0; 32]
    }
}

/// Compute RIPEMD160 hash
pub fn crypto_ripemd160(data: &[u8]) -> Vec<u8> {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_crypto_ripemd160(
                data_ptr: *const u8, data_len: usize,
                output_ptr: *mut u8
            );
        }
        
        // RIPEMD160 always outputs 20 bytes
        let mut buffer = [0u8; 20];
        
        unsafe {
            neo_crypto_ripemd160(data.as_ptr(), data.len(), buffer.as_mut_ptr());
        }
        
        buffer.to_vec()
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy hash
        vec![0; 20]
    }
}

/// Contract syscalls

/// Call a contract method
pub fn contract_call(
    hash: &[u8], method: &[u8], args: &[Any], call_flags: u32
) -> Vec<u8> {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_contract_call(
                hash_ptr: *const u8, hash_len: usize,
                method_ptr: *const u8, method_len: usize,
                args_ptr: *const u8, args_len: usize,
                call_flags: u32,
                output_ptr: *mut u8, output_len: usize
            ) -> usize;
        }
        
        // Serialize args into a buffer
        let args_serialized = serialize_args(args);
        
        // First call with null to get the size of the result
        let result_len = unsafe { 
            neo_contract_call(
                hash.as_ptr(), hash.len(),
                method.as_ptr(), method.len(),
                args_serialized.as_ptr(), args_serialized.len(),
                call_flags,
                core::ptr::null_mut(), 0
            )
        };
        
        // If length is 0, call failed or returned null
        if result_len == 0 {
            return Vec::new();
        }
        
        // Allocate buffer and get the result
        let mut buffer = Vec::with_capacity(result_len);
        unsafe {
            buffer.set_len(result_len);
            neo_contract_call(
                hash.as_ptr(), hash.len(),
                method.as_ptr(), method.len(),
                args_serialized.as_ptr(), args_serialized.len(),
                call_flags,
                buffer.as_mut_ptr(), result_len
            );
        }
        
        buffer
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy result
        Vec::new()
    }
}

/// Create a new contract
pub fn contract_create(nef_file: &[u8], manifest: &[u8]) -> Vec<u8> {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_contract_create(
                nef_file_ptr: *const u8, nef_file_len: usize,
                manifest_ptr: *const u8, manifest_len: usize,
                output_ptr: *mut u8
            );
        }
        
        // Contract creation returns a script hash (20 bytes)
        let mut buffer = [0u8; 20];
        
        unsafe {
            neo_contract_create(
                nef_file.as_ptr(), nef_file.len(),
                manifest.as_ptr(), manifest.len(),
                buffer.as_mut_ptr()
            );
        }
        
        buffer.to_vec()
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy script hash
        vec![0; 20]
    }
}

/// Update a contract
pub fn contract_update(nef_file: &[u8], manifest: &[u8]) {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_contract_update(
                nef_file_ptr: *const u8, nef_file_len: usize,
                manifest_ptr: *const u8, manifest_len: usize
            );
        }
        
        unsafe {
            neo_contract_update(
                nef_file.as_ptr(), nef_file.len(),
                manifest.as_ptr(), manifest.len()
            );
        }
    }
    
    #[cfg(test)]
    {
        // For testing, do nothing
    }
}

/// System syscalls

/// Get the execution engine running state
pub fn execution_engine_get_state() -> i32 {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_execution_engine_get_state() -> i32;
        }
        
        unsafe { neo_execution_engine_get_state() }
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy state
        1
    }
}

/// Get script container
pub fn execution_engine_get_script_container() -> Vec<u8> {
    #[cfg(not(test))]
    {
        extern "C" {
            fn neo_execution_engine_get_script_container(
                output_ptr: *mut u8, output_len: usize
            ) -> usize;
        }
        
        // First call with null to get the size of the result
        let container_len = unsafe {
            neo_execution_engine_get_script_container(core::ptr::null_mut(), 0)
        };
        
        // If length is 0, container is null
        if container_len == 0 {
            return Vec::new();
        }
        
        // Allocate buffer and get the container
        let mut buffer = Vec::with_capacity(container_len);
        unsafe {
            buffer.set_len(container_len);
            neo_execution_engine_get_script_container(buffer.as_mut_ptr(), container_len);
        }
        
        buffer
    }
    
    #[cfg(test)]
    {
        // For testing, return a dummy container
        Vec::new()
    }
}

/// Helper functions

/// Serialize an array of Any values
fn serialize_args(args: &[Any]) -> Vec<u8> {
    // In a real implementation, this would serialize the arguments
    // according to the Neo VM format.
    // For simplicity, we'll just return an empty vector in this example.
    Vec::new()
}
