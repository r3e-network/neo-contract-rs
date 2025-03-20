use neo_contract::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;

// Mock storage for testing
thread_local! {
    static MOCK_STORAGE: RefCell<HashMap<Vec<u8>, Vec<u8>>> = RefCell::new(HashMap::new());
}

// Mock implementation of storage functions for testing
mod mock_storage {
    use super::*;

    pub fn get(key: &[u8]) -> Option<Vec<u8>> {
        MOCK_STORAGE.with(|storage| storage.borrow().get(&key.to_vec()).cloned())
    }

    pub fn put(key: &[u8], value: &[u8]) {
        MOCK_STORAGE.with(|storage| {
            storage.borrow_mut().insert(key.to_vec(), value.to_vec());
        });
    }

    pub fn delete(key: &[u8]) {
        MOCK_STORAGE.with(|storage| {
            storage.borrow_mut().remove(&key.to_vec());
        });
    }

    pub fn clear() {
        MOCK_STORAGE.with(|storage| {
            storage.borrow_mut().clear();
        });
    }
    
    pub fn has(key: &[u8]) -> bool {
        MOCK_STORAGE.with(|storage| storage.borrow().contains_key(&key.to_vec()))
    }
}

// Test fixture to simplify test setup and teardown
struct TestFixture;

impl TestFixture {
    fn new() -> Self {
        mock_storage::clear();
        Self
    }
}

// Test basic storage operations using the mock
#[test]
fn test_basic_storage() {
    let _fixture = TestFixture::new();
    
    // Put a value
    mock_storage::put(b"test_key", b"test_value");
    
    // Get the value
    let value = mock_storage::get(b"test_key").unwrap();
    assert_eq!(value, b"test_value");
    
    // Check exists
    assert!(mock_storage::has(b"test_key"));
    
    // Delete the value
    mock_storage::delete(b"test_key");
    
    // Verify it's gone
    assert!(!mock_storage::has(b"test_key"));
    assert_eq!(mock_storage::get(b"test_key"), None);
}

// Test for integer storage
#[test]
fn test_integer_storage() {
    let _fixture = TestFixture::new();
    
    // Store an integer
    let counter_value: u32 = 42;
    let counter_bytes = counter_value.to_le_bytes();
    mock_storage::put(b"counter", &counter_bytes);
    
    // Retrieve and verify
    let stored_bytes = mock_storage::get(b"counter").unwrap();
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&stored_bytes);
    let stored_value = u32::from_le_bytes(bytes);
    
    assert_eq!(stored_value, 42);
}

// Test for string storage
#[test]
fn test_string_storage() {
    let _fixture = TestFixture::new();
    
    // Store a string
    let message = "Hello, Neo!";
    mock_storage::put(b"greeting", message.as_bytes());
    
    // Retrieve and verify
    let stored_bytes = mock_storage::get(b"greeting").unwrap();
    let stored_message = std::str::from_utf8(&stored_bytes).unwrap();
    
    assert_eq!(stored_message, "Hello, Neo!");
}

// Test for prefixed keys
#[test]
fn test_storage_prefixes() {
    let _fixture = TestFixture::new();
    
    // Store values with prefixes
    let prefix = b"user:";
    let user1_key = [prefix.to_vec(), b"1".to_vec()].concat();
    let user2_key = [prefix.to_vec(), b"2".to_vec()].concat();
    
    mock_storage::put(&user1_key, b"Alice");
    mock_storage::put(&user2_key, b"Bob");
    
    // Verify retrieval
    assert_eq!(mock_storage::get(&user1_key).unwrap(), b"Alice");
    assert_eq!(mock_storage::get(&user2_key).unwrap(), b"Bob");
}

// Test updating values
#[test]
fn test_updating_storage() {
    let _fixture = TestFixture::new();
    
    // Store initial value
    mock_storage::put(b"settings", b"initial");
    
    // Update value
    mock_storage::put(b"settings", b"updated");
    
    // Verify updated value
    assert_eq!(mock_storage::get(b"settings").unwrap(), b"updated");
}
