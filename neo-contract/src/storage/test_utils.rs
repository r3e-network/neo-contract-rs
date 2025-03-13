//! Testing utilities for the storage module
//!
//! This module provides utilities for testing storage-related functionality
//! without requiring a real Neo VM environment.

use crate::find_options::FindOptions;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

// Define a static mock storage for testing
// In a no_std environment, we can't use thread_local, so we'll use a static with proper locking if needed
static mut MOCK_STORAGE: Option<BTreeMap<Vec<u8>, Vec<u8>>> = None;

/// Mock storage for testing
///
/// This type provides a simulation of the Neo storage layer for testing
/// without requiring a real Neo VM environment.
pub struct MockStorage;

impl MockStorage {
    /// Clears all mock storage data
    pub fn clear() {
        Self::with_storage(|storage| {
            storage.clear();
        });
    }

    /// Gets a value from mock storage
    ///
    /// # Arguments
    /// * `key` - The key to retrieve
    ///
    /// # Returns
    /// * `Option<Vec<u8>>` - The value if found, None otherwise
    pub fn get(key: &[u8]) -> Option<Vec<u8>> { Self::with_storage(|storage| storage.get(key).cloned()) }

    /// Puts a value into mock storage
    ///
    /// # Arguments
    /// * `key` - The key to store
    /// * `value` - The value to store
    pub fn put(key: &[u8], value: &[u8]) {
        Self::with_storage(|storage| {
            storage.insert(key.to_vec(), value.to_vec());
        });
    }

    /// Deletes a value from mock storage
    ///
    /// # Arguments
    /// * `key` - The key to delete
    pub fn delete(key: &[u8]) {
        Self::with_storage(|storage| {
            storage.remove(key);
        });
    }

    /// Finds entries in mock storage with the given prefix
    ///
    /// # Arguments
    /// * `prefix` - The prefix to search for
    /// * `options` - Options for the find operation
    ///
    /// # Returns
    /// A vector of key-value pairs matching the prefix
    pub fn find(prefix: &[u8], options: &FindOptions) -> Vec<(Vec<u8>, Vec<u8>)> {
        Self::with_storage(|storage| {
            let mut results = Vec::new();

            for (key, value) in storage.iter() {
                if key.starts_with(prefix) {
                    let result_key = if options.contains(FindOptions::REMOVE_PREFIX) {
                        key[prefix.len()..].to_vec()
                    } else {
                        key.clone()
                    };

                    results.push((result_key, value.clone()));
                }
            }

            results
        })
    }

    /// Checks if a key exists in mock storage
    ///
    /// # Arguments
    /// * `key` - The key to check
    ///
    /// # Returns
    /// * `bool` - True if the key exists, false otherwise
    pub fn has(key: &[u8]) -> bool { Self::with_storage(|storage| storage.contains_key(key)) }

    /// Executes a function with access to the mock storage
    ///
    /// This is a helper method for accessing the mock storage.
    ///
    /// # Arguments
    /// * `f` - The function to execute
    ///
    /// # Returns
    /// The result of the function
    fn with_storage<F, R>(f: F) -> R
    where F: FnOnce(&mut BTreeMap<Vec<u8>, Vec<u8>>) -> R {
        unsafe {
            // Initialize storage if it hasn't been initialized yet
            if MOCK_STORAGE.is_none() {
                MOCK_STORAGE = Some(BTreeMap::new());
            }

            // Get a mutable reference to the storage and execute the function
            f(MOCK_STORAGE.as_mut().unwrap())
        }
    }

    /// Gets all key-value pairs in mock storage
    ///
    /// # Returns
    /// A vector of all key-value pairs in storage
    pub fn get_all() -> Vec<(Vec<u8>, Vec<u8>)> {
        Self::with_storage(|storage| storage.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
    }

    /// Dumps the contents of mock storage as a debug string
    ///
    /// # Returns
    /// A string representation of all key-value pairs in storage
    pub fn dump() -> String {
        Self::with_storage(|storage| {
            let mut result = String::new();

            for (key, value) in storage.iter() {
                // Try to interpret keys and values as UTF-8 strings if possible
                // Try to get string representation or use a placeholder for keys
                let key_str = match core::str::from_utf8(key) {
                    Ok(s) => s,
                    Err(_) => "[Binary Data]",
                };

                // Try to get string representation or use a placeholder for values
                let value_str = match core::str::from_utf8(value) {
                    Ok(s) => s,
                    Err(_) => "[Binary Data]",
                };

                // Build the string without using format!
                result.push_str(key_str);
                result.push_str(" => ");
                result.push_str(value_str);
                result.push_str("\n");
            }

            result
        })
    }

    /// Sets up a test with initial key-value pairs
    ///
    /// # Arguments
    /// * `pairs` - A vector of key-value pairs to initialize storage with
    pub fn setup(pairs: Vec<(Vec<u8>, Vec<u8>)>) {
        Self::clear();

        for (key, value) in pairs {
            Self::put(&key, &value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_storage_basic() {
        MockStorage::clear();

        // Test basic operations
        assert!(!MockStorage::has(b"key1"));

        MockStorage::put(b"key1", b"value1");
        assert!(MockStorage::has(b"key1"));

        assert_eq!(MockStorage::get(b"key1"), Some(b"value1".to_vec()));

        // Test delete
        MockStorage::delete(b"key1");
        assert!(!MockStorage::has(b"key1"));
    }

    #[test]
    fn test_mock_storage_find() {
        MockStorage::clear();

        // Add test data
        MockStorage::put(b"user:1:name", b"Alice");
        MockStorage::put(b"user:1:age", &[30]);
        MockStorage::put(b"user:2:name", b"Bob");
        MockStorage::put(b"user:2:age", &[25]);

        // Test find with prefix
        let results = MockStorage::find(b"user:1:", &FindOptions::default());
        assert_eq!(results.len(), 2);

        // Test find with prefix and remove_prefix option
        let mut options = FindOptions::default();
        options.add(FindOptions::REMOVE_PREFIX);
        let results = MockStorage::find(b"user:1:", &options);
        assert_eq!(results.len(), 2);

        // Check that prefixes were removed
        let has_name_key = results.iter().any(|(key, _)| key == b"name");
        assert!(has_name_key);
    }

    #[test]
    fn test_mock_storage_setup() {
        // Setup test data
        let pairs = vec![(b"key1".to_vec(), b"value1".to_vec()), (b"key2".to_vec(), b"value2".to_vec())];
        MockStorage::setup(pairs);

        // Verify setup
        assert!(MockStorage::has(b"key1"));
        assert!(MockStorage::has(b"key2"));
        assert_eq!(MockStorage::get(b"key1"), Some(b"value1".to_vec()));
        assert_eq!(MockStorage::get(b"key2"), Some(b"value2".to_vec()));
    }

    #[test]
    fn test_mock_storage_get_all() {
        MockStorage::clear();

        // Add test data
        MockStorage::put(b"key1", b"value1");
        MockStorage::put(b"key2", b"value2");

        // Get all entries
        let entries = MockStorage::get_all();
        assert_eq!(entries.len(), 2);

        // Check entries
        let has_key1 = entries.iter().any(|(key, value)| key == b"key1" && value == b"value1");
        let has_key2 = entries.iter().any(|(key, value)| key == b"key2" && value == b"value2");

        assert!(has_key1);
        assert!(has_key2);
    }
}
