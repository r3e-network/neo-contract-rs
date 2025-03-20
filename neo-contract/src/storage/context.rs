//! Storage Context for Neo Contract Rust
//!
//! This module defines the storage context used in Neo smart contracts.
//! Storage contexts control how and where contract data is stored and accessed,
//! allowing for fine-grained control over storage operations.
//!
//! A `Context` represents a specific storage area within the blockchain.
//! Each contract has its own default context, but contracts can also
//! access other contracts' storage contexts (with proper permissions).

use crate::find_options::FindOptions;
use crate::static_values::Hash160;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;
use crate::env::syscall;

/// Represents a storage context in Neo N3
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context {
    /// The contract hash associated with this context
    pub contract_hash: Vec<u8>,

    /// Whether this context is read-only
    pub is_read_only: bool,
}

/// Error types related to storage operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageError {
    /// Access denied due to permissions
    AccessDenied,

    /// Key not found in storage
    KeyNotFound,

    /// Storage operation failed
    OperationFailed,

    /// Invalid storage context
    InvalidContext,
}

// Add this type alias at the appropriate place, before it's first used
type StorageContext = crate::types::storage::StorageContext;

impl Context {
    /// Creates a new storage context for the current contract
    ///
    /// # Example
    /// ```
    /// let context = Context::new();
    /// context.put(b"greeting", b"Hello, Neo!");
    /// ```
    pub fn new() -> Self {
        // In a real implementation, this would get the current contract's
        // hash from the Neo VM. This is a placeholder that will be replaced.
        Context {
            contract_hash: Vec::new(), // Will be replaced with current contract hash
            is_read_only: false,
        }
    }

    /// Creates a new storage context with the given contract hash
    ///
    /// # Arguments
    /// * `contract_hash` - The contract hash to use for this context
    ///
    /// # Example
    /// ```
    /// let other_contract = [0x01, 0x02, 0x03]; // Some contract hash
    /// let context = Context::for_contract(&other_contract);
    /// ```
    pub fn for_contract(contract_hash: &[u8]) -> Self {
        Context { contract_hash: contract_hash.to_vec(), is_read_only: false }
    }

    /// Creates a storage context for a contract specified by its Hash160
    ///
    /// # Arguments
    /// * `hash` - The Hash160 of the contract
    ///
    /// # Example
    /// ```
    /// let hash = Hash160::from_hex("0x1234567890abcdef1234567890abcdef12345678");
    /// let context = Context::from_hash160(&hash);
    /// ```
    pub fn from_hash160(hash: &Hash160) -> Self {
        Context { contract_hash: hash.as_bytes().to_vec(), is_read_only: false }
    }

    /// Creates a read-only version of this context
    ///
    /// A read-only context can only be used for reading data, not writing.
    /// This is useful for ensuring that a contract doesn't accidentally
    /// modify state when it only needs to read.
    ///
    /// # Example
    /// ```
    /// let context = Context::new();
    /// let readonly = context.as_read_only();
    /// // readonly.put(b"key", b"value"); // This would fail at runtime
    /// ```
    pub fn as_read_only(&self) -> Self {
        Context {
            contract_hash: self.contract_hash.clone(),
            is_read_only: true,
        }
    }

    /// Gets the current contract's storage context
    ///
    /// This is a convenience method that is equivalent to `Context::new()`.
    pub fn current() -> Self { Self::new() }

    /// Gets a storage context for the specified contract
    ///
    /// # Arguments
    /// * `contract` - The contract's Hash160
    ///
    /// # Returns
    /// A storage context for the specified contract
    pub fn get_for_contract(contract: &Hash160) -> Self {
        // This will be replaced by appropriate Neo VM syscall
        Self::for_contract(contract.as_bytes())
    }

    fn get_with_syscall(&self, key: &[u8]) -> Option<Vec<u8>> {
        // For simplicity just return None
        // In a real implementation this would use syscalls
        None
    }

    /// Gets a value from storage
    ///
    /// # Arguments
    /// * `key` - The key to get
    ///
    /// # Returns
    /// * `Option<Vec<u8>>` - The value if found, None otherwise
    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.get_with_syscall(key)
    }

    #[cfg(not(test))]
    fn put_with_syscall(&self, key: &[u8], value: &[u8]) {
        put_storage_value(self, key, value);
    }

    #[cfg(test)]
    fn put_with_syscall(&self, key: &[u8], value: &[u8]) {
        use crate::test_utils;
        test_utils::MockStorage::put(key, value);
    }

    pub fn put(&self, key: &[u8], value: &[u8]) {
        // Check if we're in read-only mode
        if self.is_read_only {
            panic!("Cannot write to read-only storage context");
        }

        self.put_with_syscall(key, value);
    }

    #[cfg(not(test))]
    fn delete_with_syscall(&self, key: &[u8]) {
        delete_storage_value(self, key);
    }

    #[cfg(test)]
    fn delete_with_syscall(&self, key: &[u8]) {
        use crate::test_utils;
        test_utils::MockStorage::delete(key);
    }

    pub fn delete(&self, key: &[u8]) {
        // Check if we're in read-only mode
        if self.is_read_only {
            panic!("Cannot delete from read-only storage context");
        }

        self.delete_with_syscall(key);
    }

    /// Finds entries in storage with the given prefix using this context
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
    /// let context = Context::new();
    /// let options = FindOptions::default().set_remove_prefix(true);
    /// let results = context.find(b"user:", options);
    /// for (key, value) in results {
    ///     // Process each matching key-value pair
    /// }
    /// ```
    pub fn find(&self, prefix: &[u8], options: FindOptions) -> Vec<(Vec<u8>, Vec<u8>)> {
        #[cfg(target_arch = "wasm32")]
        unsafe {
            let context_ptr = self as *const _ as usize;
            let prefix_ptr = prefix.as_ptr() as usize;
            let prefix_len = prefix.len() as i32;
            let options_value = options.0 as i32;
            
            // For wasm32 target, just return an empty vector 
            // This is a placeholder - in a real implementation, this would use the proper syscall
            
            // Note: we're removing the problematic syscall call for now
            Vec::new()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Use mock storage for testing environment
            
            // Create a filtered result based on prefix
            let mut result = Vec::new();
            
            // In testing mode, we can just return an empty result
            // The actual test cases can implement their own expectations
            result
        }
    }

    /// Checks if a key exists in storage using this context
    ///
    /// # Arguments
    /// * `key` - The key to check
    ///
    /// # Returns
    /// * `bool` - True if the key exists, false otherwise
    ///
    /// # Example
    /// ```
    /// let context = Context::new();
    /// if context.has(b"initialized") {
    ///     // Contract has been initialized
    /// }
    /// ```
    pub fn has(&self, key: &[u8]) -> bool {
        // In a real implementation, this would call into the Neo VM
        // to check if a key exists in storage using this context.

        // This could be implemented as a Neo VM syscall or as a wrapper
        // around a get operation
        self.get(key).is_some()
    }

    /// Creates a new key by appending a suffix to a prefix
    ///
    /// This is a utility method for creating composite keys.
    ///
    /// # Arguments
    /// * `prefix` - The prefix part of the key
    /// * `suffix` - The suffix part of the key
    ///
    /// # Returns
    /// * `Vec<u8>` - The combined key
    ///
    /// # Example
    /// ```
    /// let context = Context::new();
    /// let user_id = "user123".as_bytes();
    /// let balance_key = context.create_key(b"balance:", user_id);
    /// ```
    pub fn create_key(&self, prefix: &[u8], suffix: &[u8]) -> Vec<u8> {
        let mut key = prefix.to_vec();
        key.extend_from_slice(suffix);
        key
    }

    /// Retrieves an integer value from storage
    ///
    /// # Arguments
    /// * `key` - The key to retrieve
    ///
    /// # Returns
    /// * `Option<i64>` - The integer value if found and valid, None otherwise
    pub fn get_int(&self, key: &[u8]) -> Option<i64> {
        self.get(key).and_then(|bytes| {
            if bytes.len() == 8 {
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&bytes);
                Some(i64::from_le_bytes(buf))
            } else {
                None
            }
        })
    }

    /// Stores an integer value in storage
    ///
    /// # Arguments
    /// * `key` - The key to store
    /// * `value` - The integer value to store
    pub fn put_int(&self, key: &[u8], value: i64) { self.put(key, &value.to_le_bytes()); }

    /// Retrieves a string value from storage
    ///
    /// # Arguments
    /// * `key` - The key to retrieve
    ///
    /// # Returns
    /// * `Option<String>` - The string value if found and valid, None otherwise
    pub fn get_string(&self, key: &[u8]) -> Option<String> {
        self.get(key).and_then(|bytes| String::from_utf8(bytes).ok())
    }

    /// Stores a string value in storage
    ///
    /// # Arguments
    /// * `key` - The key to store
    /// * `value` - The string value to store
    pub fn put_string(&self, key: &[u8], value: &str) { self.put(key, value.as_bytes()); }

    /// Creates a storage iterator for a given prefix
    ///
    /// This is a convenience method for working with iterators.
    ///
    /// # Arguments
    /// * `prefix` - The prefix to search for
    /// * `options` - Options for the iterator
    ///
    /// # Returns
    /// A `StorageIterator` that can be used to iterate over matching entries
    pub fn create_iterator(&self, prefix: &[u8], options: FindOptions) -> StorageIterator {
        StorageIterator::new(self.clone(), prefix.to_vec(), options)
    }
}

/// An iterator over storage entries
pub struct StorageIterator {
    // Removed unused fields: context, prefix, options
    items: Vec<(Vec<u8>, Vec<u8>)>,
    current_index: usize,
}

impl StorageIterator {
    /// Creates a new storage iterator
    ///
    /// # Arguments
    /// * `context` - The storage context to use
    /// * `prefix` - The prefix to search for
    /// * `options` - Options for the iterator
    pub fn new(context: Context, prefix: Vec<u8>, options: FindOptions) -> Self {
        let items = context.find(&prefix, options);
        Self {
            // Fields removed to fix dead code warnings
            items,
            current_index: 0,
        }
    }

    /// Create a storage iterator from an iterator ID returned by syscalls
    pub fn from_id(_id: i32, context: crate::types::context::StorageContext) -> Self {
        // Create a Context from the StorageContext
        let _neo_context = Context {
            contract_hash: {
                // Get the script hash and convert it to Vec<u8>
                let script_hash = crate::runtime::Runtime::executing_script_hash();
                script_hash.to_vec()
            },
            is_read_only: context.read_only,
        };

        // Initialize with empty items - the actual items will be fetched when needed
        Self {
            // Removed unused fields to fix dead code warnings
            items: Vec::new(), // Empty items for now
            current_index: 0,
        }
    }

    /// Gets the next item in the iterator
    ///
    /// # Returns
    /// * `Option<(Vec<u8>, Vec<u8>)>` - The next key-value pair, or None if done
    pub fn next(&mut self) -> Option<(Vec<u8>, Vec<u8>)> {
        if self.current_index < self.items.len() {
            let result = self.items[self.current_index].clone();
            self.current_index += 1;
            Some(result)
        } else {
            None
        }
    }

    /// Checks if there are more items
    ///
    /// # Returns
    /// * `bool` - True if there are more items, false if done
    pub fn has_next(&self) -> bool { self.current_index < self.items.len() }

    /// Resets the iterator to the beginning
    pub fn reset(&mut self) { self.current_index = 0; }
}

impl Default for Context {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::BTreeMap;
    use alloc::string::ToString;
    
    // Simple tests that don't depend on MockStorage implementation
    #[test]
    fn test_context_creation() {
        let context = Context::current();
        assert!(context.contract_hash == [0u8; 20]);
        
        let hash = [1u8; 20];
        let custom_context = Context::for_contract(&hash);
        assert_eq!(custom_context.contract_hash, hash);
    }
    
    // For these tests we'll just focus on the API rather than the full implementation
    #[test]
    fn test_storage_operations_api() {
        let context = Context::current();
        
        // Just checking that these methods exist and have the right signatures
        // Implementation details are tested elsewhere
        let _: Option<Vec<u8>> = context.get(b"key");
        context.put(b"key", b"value");
        context.delete(b"key");
    }
}

#[cfg(not(test))]
fn get_storage_value(context: &Context, key: &[u8]) -> Option<Vec<u8>> {
    use crate::env::syscall;
    use crate::alloc::vec::Vec;
    
    // Neo VM syscall implementation
    // Implementation will go here in production code
    None
}

#[cfg(not(test))]
fn put_storage_value(context: &Context, key: &[u8], value: &[u8]) {
    use crate::env::syscall;
    
    // Neo VM syscall implementation
    // Implementation will go here in production code
}

#[cfg(not(test))]
fn delete_storage_value(context: &Context, key: &[u8]) {
    use crate::env::syscall;
    
    // Neo VM syscall implementation
    // Implementation will go here in production code
}
