//! Mock implementation of Neo storage
//!
//! This module provides a mock implementation of Neo storage
//! for testing smart contracts.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::cell::RefCell;

thread_local! {
    /// Global storage state
    static STORAGE: RefCell<BTreeMap<Vec<u8>, Vec<u8>>> = RefCell::new(BTreeMap::new());
}

/// Mock implementation of Neo storage
pub struct MockStorage;

impl MockStorage {
    /// Reset storage to empty state
    pub fn reset() {
        STORAGE.with(|storage| {
            storage.borrow_mut().clear();
        });
    }

    /// Get a value from storage
    pub fn get(key: &[u8]) -> Option<Vec<u8>> { STORAGE.with(|storage| storage.borrow().get(&key.to_vec()).cloned()) }

    /// Put a value into storage
    pub fn put(key: &[u8], value: &[u8]) {
        STORAGE.with(|storage| {
            storage.borrow_mut().insert(key.to_vec(), value.to_vec());
        });
    }

    /// Remove a value from storage
    pub fn remove(key: &[u8]) {
        STORAGE.with(|storage| {
            storage.borrow_mut().remove(&key.to_vec());
        });
    }

    /// Check if a key exists in storage
    pub fn has(key: &[u8]) -> bool { STORAGE.with(|storage| storage.borrow().contains_key(&key.to_vec())) }

    /// Get all key-value pairs from storage
    pub fn get_all() -> Vec<(Vec<u8>, Vec<u8>)> {
        STORAGE.with(|storage| storage.borrow().iter().map(|(k, v)| (k.clone(), v.clone())).collect())
    }

    /// Setup initial storage state
    pub fn setup(pairs: Vec<(Vec<u8>, Vec<u8>)>) {
        // Clear existing storage
        Self::reset();

        // Add all pairs
        for (key, value) in pairs {
            Self::put(&key, &value);
        }
    }

    /// Get all keys that match a prefix
    pub fn get_keys_with_prefix(prefix: &[u8]) -> Vec<Vec<u8>> {
        STORAGE.with(|storage| {
            storage.borrow().iter().filter(|(k, _)| k.starts_with(prefix)).map(|(k, _)| k.clone()).collect()
        })
    }

    /// Get all values for keys that match a prefix
    pub fn get_values_with_prefix(prefix: &[u8]) -> Vec<Vec<u8>> {
        STORAGE.with(|storage| {
            storage.borrow().iter().filter(|(k, _)| k.starts_with(prefix)).map(|(_, v)| v.clone()).collect()
        })
    }

    /// Get all key-value pairs for keys that match a prefix
    pub fn get_pairs_with_prefix(prefix: &[u8]) -> Vec<(Vec<u8>, Vec<u8>)> {
        STORAGE.with(|storage| {
            storage
                .borrow()
                .iter()
                .filter(|(k, _)| k.starts_with(prefix))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_get() {
        // Reset storage
        MockStorage::reset();

        // Put a value
        MockStorage::put(b"test_key", b"test_value");

        // Get the value
        let value = MockStorage::get(b"test_key");
        assert_eq!(value, Some(b"test_value".to_vec()));

        // Get a non-existent value
        let value = MockStorage::get(b"non_existent_key");
        assert_eq!(value, None);
    }

    #[test]
    fn test_has() {
        // Reset storage
        MockStorage::reset();

        // Check a non-existent key
        assert!(!MockStorage::has(b"test_key"));

        // Put a value
        MockStorage::put(b"test_key", b"test_value");

        // Check the key exists
        assert!(MockStorage::has(b"test_key"));
    }

    #[test]
    fn test_remove() {
        // Reset storage
        MockStorage::reset();

        // Put a value
        MockStorage::put(b"test_key", b"test_value");

        // Check it exists
        assert!(MockStorage::has(b"test_key"));

        // Remove the value
        MockStorage::remove(b"test_key");

        // Check it doesn't exist
        assert!(!MockStorage::has(b"test_key"));
    }

    #[test]
    fn test_setup() {
        // Setup initial state
        let pairs = vec![(b"key1".to_vec(), b"value1".to_vec()), (b"key2".to_vec(), b"value2".to_vec())];
        MockStorage::setup(pairs);

        // Check values
        assert_eq!(MockStorage::get(b"key1"), Some(b"value1".to_vec()));
        assert_eq!(MockStorage::get(b"key2"), Some(b"value2".to_vec()));
    }

    #[test]
    fn test_get_all() {
        // Setup initial state
        let pairs = vec![(b"key1".to_vec(), b"value1".to_vec()), (b"key2".to_vec(), b"value2".to_vec())];
        MockStorage::setup(pairs);

        // Get all pairs
        let all_pairs = MockStorage::get_all();
        assert_eq!(all_pairs.len(), 2);

        // Check pairs
        assert!(all_pairs.contains(&(b"key1".to_vec(), b"value1".to_vec())));
        assert!(all_pairs.contains(&(b"key2".to_vec(), b"value2".to_vec())));
    }

    #[test]
    fn test_prefix_operations() {
        // Setup initial state
        let pairs = vec![
            (b"prefix1:key1".to_vec(), b"value1".to_vec()),
            (b"prefix1:key2".to_vec(), b"value2".to_vec()),
            (b"prefix2:key3".to_vec(), b"value3".to_vec()),
        ];
        MockStorage::setup(pairs);

        // Get keys with prefix
        let keys = MockStorage::get_keys_with_prefix(b"prefix1:");
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&b"prefix1:key1".to_vec()));
        assert!(keys.contains(&b"prefix1:key2".to_vec()));

        // Get values with prefix
        let values = MockStorage::get_values_with_prefix(b"prefix1:");
        assert_eq!(values.len(), 2);
        assert!(values.contains(&b"value1".to_vec()));
        assert!(values.contains(&b"value2".to_vec()));

        // Get pairs with prefix
        let pairs = MockStorage::get_pairs_with_prefix(b"prefix1:");
        assert_eq!(pairs.len(), 2);
        assert!(pairs.contains(&(b"prefix1:key1".to_vec(), b"value1".to_vec())));
        assert!(pairs.contains(&(b"prefix1:key2".to_vec(), b"value2".to_vec())));
    }
}
