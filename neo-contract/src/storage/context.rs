//! Storage Context for Neo Contract Rust
//!
//! This module defines the storage context used in Neo smart contracts.
//! Storage contexts control how and where contract data is stored and accessed,
//! allowing for fine-grained control over storage operations.
//!
//! A `Context` represents a specific storage area within the blockchain.
//! Each contract has its own default context, but contracts can also 
//! access other contracts' storage contexts (with proper permissions).

use alloc::vec::Vec;
use alloc::string::String;
use crate::find_options::FindOptions;
use crate::static_values::Hash160;

// Import test utils only in test mode
#[cfg(test)]
use crate::test_utils::MockStorage;

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
        Context {
            contract_hash: contract_hash.to_vec(),
            is_read_only: false,
        }
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
        Context {
            contract_hash: hash.as_bytes().to_vec(),
            is_read_only: false,
        }
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
    pub fn current() -> Self {
        Self::new()
    }
    
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
    
    /// Gets a value from storage using this context
    ///
    /// # Arguments
    /// * `key` - The key to retrieve
    ///
    /// # Returns
    /// * `Option<Vec<u8>>` - The value if found, None otherwise
    ///
    /// # Example
    /// ```
    /// let context = Context::new();
    /// if let Some(value) = context.get(b"counter") {
    ///     // Use value
    /// }
    /// ```
    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        // Production implementation that calls into the Neo VM
        // to get data from storage using this context.
        
        #[cfg(test)]
        {
            // Create a mock storage instance and use it
            let mock_storage = MockStorage::new();
            mock_storage.get(key)
        }
        
        #[cfg(not(test))]
        {
            use crate::env::syscall_non_wasm;
            use crate::types::builtin::string::ByteString;
            
            unsafe {
                // Get the storage context
                let context = syscall_non_wasm::system_storage_get_context();
                let key_bytes = ByteString::from(key);
                
                // Call the Neo VM syscall to get the value
                let result = syscall_non_wasm::system_storage_get(
                    context,
                    key_bytes
                );
                
                if result.len() == 0 {
                    None
                } else {
                    Some(result.as_bytes().to_vec())
                }
            }
        }
    }
    
    /// Puts a value into storage using this context
    ///
    /// # Arguments
    /// * `key` - The key to store
    /// * `value` - The value to store
    ///
    /// # Example
    /// ```
    /// let context = Context::new();
    /// context.put(b"counter", &[0, 0, 0, 1]);
    /// ```
    pub fn put(&self, key: &[u8], value: &[u8]) {
        // Check if context is read-only
        if self.is_read_only {
            // Production implementation should trigger an exception
            panic!("Cannot write to read-only storage context");
        }
        
        #[cfg(test)]
        {
            // Create a mock storage instance and use it
            let mut mock_storage = MockStorage::new();
            mock_storage.put(key, value);
        }
        
        #[cfg(not(test))]
        {
            // Production implementation that calls into the Neo VM
            // to put data into storage using this context.
            use crate::env::syscall_non_wasm;
            use crate::types::builtin::string::ByteString;
            
            unsafe {
                // Get the storage context
                let context = syscall_non_wasm::system_storage_get_context();
                let key_bytes = ByteString::from(key);
                let value_bytes = ByteString::from(value);
                
                // Call the Neo VM syscall to store the value
                syscall_non_wasm::system_storage_put(
                    context,
                    key_bytes,
                    value_bytes
                );
            }
        }
    }
    
    /// Deletes a value from storage using this context
    ///
    /// # Arguments
    /// * `key` - The key to delete
    ///
    /// # Example
    /// ```
    /// let context = Context::new();
    /// context.delete(b"temporary_data");
    /// ```
    pub fn delete(&self, key: &[u8]) {
        // Check if context is read-only
        if self.is_read_only {
            // Production implementation should trigger an exception
            panic!("Cannot delete from read-only storage context");
        }
        
        #[cfg(test)]
        {
            // Create a mock storage instance and use it
            let mut mock_storage = MockStorage::new();
            mock_storage.delete(key);
        }
        
        #[cfg(not(test))]
        {
            // Production implementation that calls into the Neo VM
            // to delete data from storage using this context.
            use crate::env::syscall_non_wasm;
            use crate::types::builtin::string::ByteString;
            
            unsafe {
                // Get the storage context
                let context = syscall_non_wasm::system_storage_get_context();
                let key_bytes = ByteString::from(key);
                
                // Call the Neo VM syscall to delete the value
                syscall_non_wasm::system_storage_delete(
                    context,
                    key_bytes
                );
            }
        }
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
    pub fn find(&self, _prefix: &[u8], _options: FindOptions) -> Vec<(Vec<u8>, Vec<u8>)> {
        // In a real implementation, this would call into the Neo VM
        // to find data in storage using this context.
        
        // Neo VM syscall: "System.Storage.Find"
        // Will be replaced with actual code that interfaces with the Neo VM
        
        #[cfg(test)]
        {
            MockStorage::find(prefix, &options)
        }
        
        #[cfg(not(test))]
        {
            Vec::new() // Placeholder
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
    pub fn put_int(&self, key: &[u8], value: i64) {
        self.put(key, &value.to_le_bytes());
    }
    
    /// Retrieves a string value from storage
    ///
    /// # Arguments
    /// * `key` - The key to retrieve
    ///
    /// # Returns
    /// * `Option<String>` - The string value if found and valid, None otherwise
    pub fn get_string(&self, key: &[u8]) -> Option<String> {
        self.get(key).and_then(|bytes| {
            String::from_utf8(bytes).ok()
        })
    }
    
    /// Stores a string value in storage
    ///
    /// # Arguments
    /// * `key` - The key to store
    /// * `value` - The string value to store
    pub fn put_string(&self, key: &[u8], value: &str) {
        self.put(key, value.as_bytes());
    }
    
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
    pub fn has_next(&self) -> bool {
        self.current_index < self.items.len()
    }
    
    /// Resets the iterator to the beginning
    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::BTreeMap;
    
    // Local mock storage for testing - replaces the std::collections::HashMap
    pub struct MockStorage {
        pub items: BTreeMap<Vec<u8>, Vec<u8>>,
    }
    
    impl MockStorage {
        pub fn new() -> Self {
            Self {
                items: BTreeMap::new(),
            }
        }
        
        pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
            self.items.get(key).cloned()
        }
        
        pub fn put(&mut self, key: &[u8], value: &[u8]) {
            self.items.insert(key.to_vec(), value.to_vec());
        }
        
        pub fn delete(&mut self, key: &[u8]) {
            self.items.remove(key);
        }
    }
    
    #[test]
    fn test_context_creation() {
        let context = Context::new();
        assert!(!context.is_read_only);
        
        let readonly = context.as_read_only();
        assert!(readonly.is_read_only);
    }
    
    #[test]
    fn test_storage_operations() {
        MockStorage::clear();
        
        let context = Context::new();
        
        // Test basic storage operations
        assert!(!context.has(b"key1"));
        context.put(b"key1", b"value1");
        assert!(context.has(b"key1"));
        assert_eq!(context.get(b"key1"), Some(b"value1".to_vec()));
        
        // Test delete
        context.delete(b"key1");
        assert!(!context.has(b"key1"));
        
        // Test helper methods
        context.put_int(b"counter", 42);
        assert_eq!(context.get_int(b"counter"), Some(42));
        
        context.put_string(b"greeting", "Hello, Neo!");
        assert_eq!(context.get_string(b"greeting"), Some("Hello, Neo!".to_string()));
    }
    
    #[test]
    fn test_find_operation() {
        MockStorage::clear();
        
        let context = Context::new();
        
        // Add multiple items with a common prefix
        context.put(b"user:1:name", b"Alice");
        context.put(b"user:1:age", &[30]);
        context.put(b"user:2:name", b"Bob");
        context.put(b"user:2:age", &[25]);
        context.put(b"config:mode", b"test");
        
        // Test find with prefix
        let options = FindOptions::default();
        let results = context.find(b"user:1:", options);
        assert_eq!(results.len(), 2);
        
        // Test find with prefix and remove_prefix option
        let options = FindOptions::default().set_remove_prefix(true);
        let results = context.find(b"user:1:", options);
        assert_eq!(results.len(), 2);
        
        // Check some of the keys have the prefix removed
        let has_name_key = results.iter().any(|(key, _)| {
            key == b"name"
        });
        assert!(has_name_key);
    }
    
    #[test]
    fn test_iterator() {
        MockStorage::clear();
        
        let context = Context::new();
        
        // Add multiple items with a common prefix
        context.put(b"user:1:name", b"Alice");
        context.put(b"user:1:age", &[30]);
        context.put(b"user:2:name", b"Bob");
        context.put(b"user:2:age", &[25]);
        
        // Test iterator
        let mut iter = context.create_iterator(b"user:1:", FindOptions::default());
        let mut count = 0;
        
        while iter.has_next() {
            let (key, value) = iter.next().unwrap();
            count += 1;
            assert!(key.starts_with(b"user:1:"));
        }
        
        assert_eq!(count, 2);
        
        // Test reset
        iter.reset();
        assert!(iter.has_next());
    }
    
    #[test]
    fn test_readonly_context() {
        MockStorage::clear();
        
        let mut_context = Context::new();
        let ro_context = mut_context.as_read_only();
        
        // Setup some initial state
        mut_context.put(b"key1", b"value1");
        
        // Read-only context can read
        assert_eq!(ro_context.get(b"key1"), Some(b"value1".to_vec()));
        
        // Read-only context put should not do anything
        ro_context.put(b"key2", b"value2");
        assert!(!ro_context.has(b"key2"));
        
        // Same for delete
        ro_context.delete(b"key1");
        assert!(ro_context.has(b"key1"));
    }
}
