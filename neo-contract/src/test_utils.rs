//! Test utilities for Neo Contract RS
//!
//! This module provides utilities for testing Neo smart contracts.
//! It includes mock objects and utility functions to simplify testing.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// Mock storage for testing storage operations
#[derive(Debug, Default)]
pub struct MockStorage {
    /// Map of storage entries
    pub items: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl MockStorage {
    /// Create a new, empty mock storage
    pub fn new() -> Self { Self { items: BTreeMap::new() } }

    /// Get a value from mock storage
    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> { self.items.get(key).cloned() }

    /// Store a value in mock storage
    pub fn put(&mut self, key: &[u8], value: &[u8]) { self.items.insert(key.to_vec(), value.to_vec()); }

    /// Delete a value from mock storage
    pub fn delete(&mut self, key: &[u8]) { self.items.remove(key); }

    /// Check if a key exists in storage
    pub fn contains_key(&self, key: &[u8]) -> bool { self.items.contains_key(key) }
}
