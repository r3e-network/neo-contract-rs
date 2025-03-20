//! Test utilities for Neo Contract RS
//!
//! This module provides utilities for testing Neo smart contracts.
//! It includes mock objects and utility functions to simplify testing.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use crate::prelude::*;
use crate::types::builtin::any::Any;
use crate::types::builtin::array::Array;
use crate::types::builtin::string::ByteString;
use core::cell::RefCell;

/// MockRuntime for testing Neo contracts
pub struct MockRuntime;

/// Storage for mock data during tests
#[derive(Default)]
pub struct MockStorage {
    /// Mock storage data (key-value pairs)
    pub data: BTreeMap<Vec<u8>, Vec<u8>>,
    /// Captured events (name, args)
    pub events: Vec<(String, Vec<Any>)>,
}

impl MockStorage {
    /// Create a new empty mock storage
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
            events: Vec::new(),
        }
    }
    
    /// Get the singleton instance of mock storage
    pub fn get_instance() -> Self {
        // This is not a real singleton pattern since we can't have static mut
        // in no_std. For testing purposes, we'll create a new instance each time.
        MockStorage::new()
    }
    
    /// Store a key-value pair
    pub fn put(key: &[u8], value: &[u8]) {
        // In a real implementation, this would update a shared instance
        // For now, just log that the operation was requested
        #[cfg(feature = "mock-log")]
        crate::runtime::Runtime::log_str(&format!("MockStorage.put({})", key.len()));
    }
    
    /// Get a value by key
    pub fn get(key: &[u8]) -> Option<Vec<u8>> {
        // In a real implementation, this would fetch from a shared instance
        // For now, just return None to indicate no value
        #[cfg(feature = "mock-log")]
        crate::runtime::Runtime::log_str(&format!("MockStorage.get({})", key.len()));
        None // Return None instead of empty vec
    }
    
    /// Delete a key-value pair
    pub fn delete(key: &[u8]) {
        // In a real implementation, this would update a shared instance
        // For now, just log that the operation was requested
        #[cfg(feature = "mock-log")]
        crate::runtime::Runtime::log_str(&format!("MockStorage.delete({})", key.len()));
    }
    
    /// Record an event
    pub fn record_event(name: &str, args: &Array) {
        // In a real implementation, this would update a shared events list
        #[cfg(feature = "mock-log")]
        crate::runtime::Runtime::log_str(&format!("MockStorage.record_event({})", name));
    }
    
    /// Clear all storage data
    pub fn clear() {
        // In a real implementation, this would clear the storage
        #[cfg(feature = "mock-log")]
        crate::runtime::Runtime::log_str("MockStorage.clear()");
    }
}

impl MockRuntime {
    /// Mock check_witness that always returns true
    pub fn mock_check_witness(_address: &H160) -> bool {
        true
    }
    
    /// Mock get_time that returns a fixed timestamp
    pub fn mock_get_time() -> u64 {
        1629264000 // 2021-08-18T00:00:00Z
    }

    /// Mock notify that captures events for testing
    pub fn mock_notify(name: &str, args: &Array) {
        // Add the event to the captured events list
        MockStorage::record_event(name, args);
    }
    
    /// Mock storage get operation
    pub fn mock_storage_get(key: &[u8]) -> Vec<u8> {
        MockStorage::get(key).unwrap_or_default()
    }
    
    /// Mock storage put operation
    pub fn mock_storage_put(key: &[u8], value: &[u8]) {
        MockStorage::put(key, value);
    }
    
    /// Mock storage delete operation
    pub fn mock_storage_delete(key: &[u8]) {
        MockStorage::delete(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mock_storage() {
        // Start with a clean state
        MockStorage::clear();
        
        // Put and get data
        MockStorage::put(b"key1", b"value1");
        MockStorage::put(b"key2", b"value2");
        
        assert_eq!(MockStorage::get(b"key1"), Some(b"value1".to_vec()));
        assert_eq!(MockStorage::get(b"key2"), Some(b"value2".to_vec()));
        assert_eq!(MockStorage::get(b"nonexistent"), None);
        
        // Delete data
        MockStorage::delete(b"key1");
        assert_eq!(MockStorage::get(b"key1"), None);
        
        // Clear all data
        MockStorage::clear();
        assert_eq!(MockStorage::get(b"key2"), None);
    }
}
