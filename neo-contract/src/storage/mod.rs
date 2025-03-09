//! Storage module for Neo Contract Rust
//!
//! This module provides comprehensive storage functionality for Neo N3 smart contracts.
//! It includes abstractions for working with different storage types, contexts, and
//! patterns commonly used in blockchain applications.

pub mod context;
pub mod item;
pub mod map;
pub mod iter;
pub mod versioned;
pub mod pagination;

use alloc::vec::Vec;
use alloc::string::String;
use crate::types::builtin::string::ByteString;
use crate::find_options::FindOptions;
use crate::static_values::Hash160;

// Re-export important types for convenience
pub use crate::storage::context::Context;
pub use crate::storage::context::StorageIterator;
pub use crate::storage::context::StorageError;

#[cfg(test)]
mod test_utils;

#[cfg(test)]
pub use test_utils::MockStorage;

/// Gets a value from storage using the current contract's context
///
/// This is a convenience function that uses the current contract's
/// storage context to retrieve a value.
///
/// # Arguments
/// * `key` - The key to retrieve
///
/// # Returns
/// * `Option<Vec<u8>>` - The value if found, None otherwise
///
/// # Example
/// ```
/// let value = storage::get(b"counter");
/// ```
pub fn get(key: &[u8]) -> Option<Vec<u8>> {
    // Get the current context and use it to retrieve the value
    let ctx = context();
    ctx.get(key)
}

/// Puts a value into storage using the current contract's context
///
/// This is a convenience function that uses the current contract's
/// storage context to store a value.
///
/// # Arguments
/// * `key` - The key to store
/// * `value` - The value to store
///
/// # Example
/// ```
/// storage::put(b"counter", &[0, 0, 0, 1]);
/// ```
pub fn put(key: &[u8], value: &[u8]) {
    // Get the current context and use it to store the value
    let ctx = context();
    ctx.put(key, value)
}

/// Deletes a value from storage using the current contract's context
///
/// This is a convenience function that uses the current contract's
/// storage context to delete a value.
///
/// # Arguments
/// * `key` - The key to delete
///
/// # Example
/// ```
/// storage::delete(b"temporary_data");
/// ```
pub fn delete(key: &[u8]) {
    // Get the current context and use it to delete the value
    let ctx = context();
    ctx.delete(key)
}

/// Finds entries in storage with the given prefix using the current contract's context
///
/// This is a convenience function that uses the current contract's
/// storage context to find values with a given prefix.
///
/// # Arguments
/// * `prefix` - The prefix to search for
/// * `options` - Options for the find operation
///
/// # Returns
/// A vector of key-value pairs matching the prefix
///
/// # Example
/// ```
/// let options = FindOptions::default().set_remove_prefix(true);
/// let results = storage::find(b"user:", options);
/// ```
pub fn find(prefix: &[u8], options: FindOptions) -> Vec<(Vec<u8>, Vec<u8>)> {
    // Get the current context and use it to find values
    let ctx = context();
    ctx.find(prefix, options)
}

/// Checks if a key exists in storage using the current contract's context
///
/// This is a convenience function that uses the current contract's
/// storage context to check if a key exists.
///
/// # Arguments
/// * `key` - The key to check
///
/// # Returns
/// * `bool` - True if the key exists, false otherwise
///
/// # Example
/// ```
/// if storage::has(b"initialized") {
///     // Contract has been initialized
/// }
/// ```
pub fn has(key: &[u8]) -> bool {
    // Get the current context and use it to check if the key exists
    let ctx = context();
    ctx.has(key)
}

/// Gets the current storage context
///
/// This function returns the current contract's storage context,
/// which can be used for storage operations.
///
/// # Returns
/// The current contract's storage context
///
/// # Example
/// ```
/// let ctx = storage::context();
/// ctx.put(b"key", b"value");
/// ```
pub fn context() -> context::Context {
    // This will be replaced with a proper Neo VM syscall
    // that retrieves the current contract's storage context
    context::Context::new()
}

/// Creates a new storage context with the given contract hash
///
/// This function returns a storage context for the specified contract,
/// which can be used to access another contract's storage (with proper permissions).
///
/// # Arguments
/// * `contract_hash` - The contract hash to use for this context
///
/// # Returns
/// A storage context for the specified contract
///
/// # Example
/// ```
/// let other_contract = [0x01, 0x02, 0x03]; // Some contract hash
/// let ctx = storage::context_for(&other_contract);
/// ```
pub fn context_for(contract_hash: &[u8]) -> context::Context {
    // This will be replaced with a proper Neo VM syscall
    // that creates a storage context for the specified contract
    context::Context::for_contract(contract_hash)
}

/// Creates a storage context for a contract specified by its Hash160
///
/// # Arguments
/// * `hash` - The Hash160 of the contract
///
/// # Returns
/// A storage context for the specified contract
///
/// # Example
/// ```
/// let hash = Hash160::from_hex("0x1234567890abcdef1234567890abcdef12345678");
/// let ctx = storage::context_for_hash160(&hash);
/// ```
pub fn context_for_hash160(hash: &Hash160) -> context::Context {
    context::Context::from_hash160(hash)
}

/// Creates a read-only storage context for a contract
///
/// This function returns a read-only storage context for the specified contract,
/// which can be used to read data but not modify it.
///
/// # Arguments
/// * `contract_hash` - The contract hash to use for this context
///
/// # Returns
/// A read-only storage context for the specified contract
///
/// # Example
/// ```
/// let other_contract = [0x01, 0x02, 0x03]; // Some contract hash
/// let ctx = storage::readonly_context_for(&other_contract);
/// ```
pub fn readonly_context_for(contract_hash: &[u8]) -> context::Context {
    let ctx = context_for(contract_hash);
    ctx.as_read_only()
}

/// Retrieves an integer value from storage
///
/// # Arguments
/// * `key` - The key to retrieve
///
/// # Returns
/// * `Option<i64>` - The integer value if found and valid, None otherwise
pub fn get_int(key: &[u8]) -> Option<i64> {
    let ctx = context();
    ctx.get_int(key)
}

/// Stores an integer value in storage
///
/// # Arguments
/// * `key` - The key to store
/// * `value` - The integer value to store
pub fn put_int(key: &[u8], value: i64) {
    let ctx = context();
    ctx.put_int(key, value)
}

/// Retrieves a string value from storage
///
/// # Arguments
/// * `key` - The key to retrieve
///
/// # Returns
/// * `Option<String>` - The string value if found and valid, None otherwise
pub fn get_string(key: &[u8]) -> Option<String> {
    let ctx = context();
    ctx.get_string(key)
}

/// Stores a string value in storage
///
/// # Arguments
/// * `key` - The key to store
/// * `value` - The string value to store
pub fn put_string(key: &[u8], value: &str) {
    let ctx = context();
    ctx.put_string(key, value)
}

/// Creates a storage iterator for a given prefix
///
/// This is a convenience function that creates an iterator over
/// storage entries with a given prefix.
///
/// # Arguments
/// * `prefix` - The prefix to search for
/// * `options` - Options for the iterator
///
/// # Returns
/// A `StorageIterator` that can be used to iterate over matching entries
///
/// # Example
/// ```
/// let mut iter = storage::create_iterator(b"user:", FindOptions::default());
/// while iter.has_next() {
///     let (key, value) = iter.next().unwrap();
///     // Process each matching key-value pair
/// }
/// ```
pub fn create_iterator(prefix: &[u8], options: FindOptions) -> StorageIterator {
    let ctx = context();
    ctx.create_iterator(prefix, options)
}

/// Creates a composite storage key
///
/// This is a utility function for creating keys that consist of
/// multiple parts.
///
/// # Arguments
/// * `parts` - The parts of the key to combine
///
/// # Returns
/// * `Vec<u8>` - The combined key
///
/// # Example
/// ```
/// let key = storage::create_key(&[b"user:", b"123", b":balance"]);
/// ```
pub fn create_key(parts: &[&[u8]]) -> Vec<u8> {
    let mut key = Vec::new();
    for part in parts {
        key.extend_from_slice(part);
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_operations() {
        // Clear any existing mock data
        MockStorage::clear();
        
        // Test basic operations
        assert!(!has(b"key1"));
        put(b"key1", b"value1");
        assert!(has(b"key1"));
        assert_eq!(get(b"key1"), Some(b"value1".to_vec()));
        
        // Test delete
        delete(b"key1");
        assert!(!has(b"key1"));
        
        // Test helper methods
        put_int(b"counter", 42);
        assert_eq!(get_int(b"counter"), Some(42));
        
        put_string(b"greeting", "Hello, Neo!");
        assert_eq!(get_string(b"greeting"), Some("Hello, Neo!".to_string()));
    }
    
    #[test]
    fn test_find_and_iterator() {
        // Clear any existing mock data
        MockStorage::clear();
        
        // Add test data
        put(b"user:1:name", b"Alice");
        put(b"user:1:age", &[30]);
        put(b"user:2:name", b"Bob");
        put(b"user:2:age", &[25]);
        
        // Test find
        let results = find(b"user:1:", FindOptions::default());
        assert_eq!(results.len(), 2);
        
        // Test iterator
        let mut iter = create_iterator(b"user:1:", FindOptions::default());
        let mut count = 0;
        
        while iter.has_next() {
            let (key, value) = iter.next().unwrap();
            count += 1;
            assert!(key.starts_with(b"user:1:"));
        }
        
        assert_eq!(count, 2);
    }
    
    #[test]
    fn test_create_key() {
        let key = create_key(&[b"user:", b"123", b":name"]);
        assert_eq!(key, b"user:123:name");
    }
    
    #[test]
    fn test_context_operations() {
        // Clear any existing mock data
        MockStorage::clear();
        
        // Get the current context
        let ctx = context();
        
        // Test basic operations with the context
        ctx.put(b"key1", b"value1");
        assert!(ctx.has(b"key1"));
        assert_eq!(ctx.get(b"key1"), Some(b"value1".to_vec()));
        
        // Test read-only context
        let ro_ctx = ctx.as_read_only();
        assert_eq!(ro_ctx.get(b"key1"), Some(b"value1".to_vec()));
        
        // The following should not modify storage (since ro_ctx is read-only)
        ro_ctx.put(b"key2", b"value2");
        assert!(!has(b"key2"));
    }
}
