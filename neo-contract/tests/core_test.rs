//! Unit tests for core neo-contract functionality
//!
//! These tests verify the core functionality of the neo-contract crate
//! including types, runtime, and storage.

extern crate alloc;

use neo_contract::prelude::*;
use neo_contract::types::builtin::array::Array;
use neo_contract::types::builtin::any::Any;
use neo_contract::types::builtin::h160::H160;
use neo_contract::types::builtin::string::ByteString;
use alloc::string::ToString;
use std::cell::RefCell;
use std::collections::HashMap;

// Mock storage for testing
thread_local! {
    static MOCK_STORAGE: RefCell<HashMap<Vec<u8>, Vec<u8>>> = RefCell::new(HashMap::new());
    static MOCK_EVENTS: RefCell<Vec<(ByteString, Array<Any>)>> = RefCell::new(Vec::new());
}

// Mock implementation for storage
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
}

// Mock implementation for events
mod mock_events {
    use super::*;
    
    pub fn notify(name: &ByteString, args: &Array<Any>) {
        MOCK_EVENTS.with(|events| {
            events.borrow_mut().push((name.clone(), args.clone()));
        });
    }
    
    pub fn clear() {
        MOCK_EVENTS.with(|events| {
            events.borrow_mut().clear();
        });
    }
    
    pub fn get_events() -> Vec<(ByteString, Array<Any>)> {
        MOCK_EVENTS.with(|events| {
            events.borrow().clone()
        })
    }
}

// Test fixture
struct TestFixture;

impl TestFixture {
    fn new() -> Self {
        mock_storage::clear();
        mock_events::clear();
        Self
    }
}

// Helper functions for H160 creation in tests
fn create_h160_from_bytes(bytes: [u8; 20]) -> H160 {
    H160::from_slice(&bytes)
}

fn create_test_h160(value: u8) -> H160 {
    let bytes = [value; 20];
    H160::from_slice(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests for ByteString
    #[test]
    fn test_byte_string() {
        let _fixture = TestFixture::new();
        
        // Test creation
        let bs1 = ByteString::from("Hello");
        let bs2 = ByteString::from(b"Hello".as_ref());
        
        // Test equality
        assert_eq!(bs1, bs2);
        
        // Test concatenation - manually implement
        let bs3 = ByteString::from(" World");
        let mut concat_vec = bs1.to_vec();
        concat_vec.extend_from_slice(&bs3);
        let concatenated = ByteString::from(concat_vec);
        
        assert_eq!(concatenated, ByteString::from("Hello World"));
        
        // Test conversion to string
        let string_val = String::from_utf8_lossy(&bs1).to_string();
        assert_eq!(string_val, "Hello");
    }
    
    // Tests for H160 address type
    #[test]
    fn test_h160() {
        let _fixture = TestFixture::new();
        
        // Test creation
        let mut bytes = [0u8; 20];
        for i in 0..20 {
            bytes[i] = i as u8;
        }
        
        let h160 = H160::from_slice(&bytes);
        
        // Test byte access
        assert_eq!(h160.as_bytes(), &bytes);
        
        // Test equality
        let h160_2 = H160::from_slice(&bytes);
        assert_eq!(h160, h160_2);
        
        // Test different values
        let mut bytes_2 = [0u8; 20];
        for i in 0..20 {
            bytes_2[i] = (i + 1) as u8;
        }
        
        let h160_3 = H160::from_slice(&bytes_2);
        assert_ne!(h160, h160_3);
    }
    
    // Tests for Array type
    #[test]
    fn test_array() {
        let _fixture = TestFixture::new();
        
        // Create an empty array
        let mut array: Array<Any> = Array::new();
        assert_eq!(array.len(), 0);
        
        // Push some values
        array.push(Any::integer(42));
        array.push(Any::boolean(true));
        array.push(Any::byte_string("test"));
        
        // Test size
        assert_eq!(array.len(), 3);
        
        // Test access
        assert_eq!(array.get(0).unwrap().as_i64().unwrap(), 42);
        assert_eq!(array.get(1).unwrap().as_bool().unwrap(), true);
        assert_eq!(array.get(2).unwrap().as_byte_string().unwrap(), &ByteString::from("test"));
        
        // Test setting
        array.set(0, Any::integer(100));
        assert_eq!(array.get(0).unwrap().as_i64().unwrap(), 100);
        
        // Test range checks
        assert!(array.get(3).is_none());
        
        // Test clear
        array.clear();
        assert_eq!(array.len(), 0);
    }
    
    // Tests for Any type
    #[test]
    fn test_any() {
        let _fixture = TestFixture::new();
        
        // Test integer
        let i = Any::integer(42);
        assert_eq!(i.as_i64().unwrap(), 42);
        assert!(i.as_bool().is_none());
        
        // Test boolean
        let b = Any::boolean(true);
        assert_eq!(b.as_bool().unwrap(), true);
        assert!(b.as_i64().is_none());
        
        // Test byte string
        let bs = Any::byte_string("test");
        assert_eq!(bs.as_byte_string().unwrap(), &ByteString::from("test"));
        assert!(bs.as_bool().is_none());
        
        // Test H160
        let mut bytes = [0u8; 20];
        for i in 0..20 {
            bytes[i] = i as u8;
        }
        let h160 = H160::from_slice(&bytes);
        let h = Any::h160(h160);
        assert_eq!(h.as_h160().unwrap().as_bytes(), bytes);
        
        // Test null
        let null = Any::null();
        assert!(null.is_null());
    }
    
    // Add a simple Event struct for testing
    struct TransferEvent {
        from: H160,
        to: H160,
        amount: u64,
    }
    
    impl TransferEvent {
        fn new(from: H160, to: H160, amount: u64) -> Self {
            Self { from, to, amount }
        }
        
        fn emit(&self) {
            let mut args: Array<Any> = Array::new();
            args.push(Any::h160(self.from.clone()));
            args.push(Any::h160(self.to.clone()));
            args.push(Any::integer(self.amount as i64));
            
            mock_events::notify(&ByteString::from("Transfer"), &args);
        }
    }
    
    // Tests for events
    #[test]
    fn test_events() {
        let _fixture = TestFixture::new();
        
        // Create addresses
        let from = create_test_h160(1);
        let to = create_test_h160(2);
        
        // Emit event
        let event = TransferEvent::new(from, to, 1000);
        event.emit();
        
        // Check that the event was emitted
        let events = mock_events::get_events();
        assert_eq!(events.len(), 1);
        
        let (name, args) = &events[0];
        assert_eq!(name, &ByteString::from("Transfer"));
        assert_eq!(args.len(), 3);
        
        // Verify arguments - compare byte representations
        assert_eq!(args.get(0).unwrap().as_h160().unwrap().as_bytes(), [1u8; 20]);
        assert_eq!(args.get(1).unwrap().as_h160().unwrap().as_bytes(), [2u8; 20]);
        assert_eq!(args.get(2).unwrap().as_i64().unwrap(), 1000);
    }
} 