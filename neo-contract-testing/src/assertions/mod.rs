//! Assertion utilities for Neo contract testing
//!
//! This module provides assertion utilities for testing
//! Neo smart contracts.

use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;

use crate::mock::{MockStorage, MockRuntime, MockEvents};
use crate::TestError;

/// Assert that a key exists in storage
pub fn assert_storage_has(key: &[u8]) -> Result<(), TestError> {
    if MockStorage::has(key) {
        Ok(())
    } else {
        Err(TestError::new(format!("Storage does not have key: {:?}", key)))
    }
}

/// Assert that a key does not exist in storage
pub fn assert_storage_not_has(key: &[u8]) -> Result<(), TestError> {
    if !MockStorage::has(key) {
        Ok(())
    } else {
        Err(TestError::new(format!("Storage has key when it should not: {:?}", key)))
    }
}

/// Assert that a key in storage has a specific value
pub fn assert_storage_equals(key: &[u8], expected: &[u8]) -> Result<(), TestError> {
    match MockStorage::get(key) {
        Some(value) => {
            if value == expected {
                Ok(())
            } else {
                Err(TestError::new(format!(
                    "Storage value for key {:?} does not match. Expected {:?}, got {:?}",
                    key, expected, value
                )))
            }
        }
        None => Err(TestError::new(format!("Storage does not have key: {:?}", key))),
    }
}

/// Assert that storage contains a set of key-value pairs
pub fn assert_storage_contains(pairs: &[(&[u8], &[u8])]) -> Result<(), TestError> {
    for (key, expected) in pairs {
        assert_storage_equals(key, expected)?;
    }
    Ok(())
}

/// Assert that a witness is verified
pub fn assert_witness_verified(witness: &[u8]) -> Result<(), TestError> {
    if MockRuntime::check_witness(witness) {
        Ok(())
    } else {
        Err(TestError::new(format!("Witness not verified: {:?}", witness)))
    }
}

/// Assert that a witness is not verified
pub fn assert_witness_not_verified(witness: &[u8]) -> Result<(), TestError> {
    if !MockRuntime::check_witness(witness) {
        Ok(())
    } else {
        Err(TestError::new(format!("Witness verified when it should not be: {:?}", witness)))
    }
}

/// Assert that the block height matches the expected value
pub fn assert_block_height(expected: u32) -> Result<(), TestError> {
    let actual = MockRuntime::get_block_height();
    if actual == expected {
        Ok(())
    } else {
        Err(TestError::new(format!(
            "Block height does not match. Expected {}, got {}",
            expected, actual
        )))
    }
}

/// Assert that the timestamp matches the expected value
pub fn assert_timestamp(expected: u64) -> Result<(), TestError> {
    let actual = MockRuntime::get_time();
    if actual == expected {
        Ok(())
    } else {
        Err(TestError::new(format!(
            "Timestamp does not match. Expected {}, got {}",
            expected, actual
        )))
    }
}

/// Assert that an event with the given name was emitted
pub fn assert_event_emitted(name: &str) -> Result<(), TestError> {
    if MockEvents::has_event(name) {
        Ok(())
    } else {
        Err(TestError::new(format!("Event not emitted: {}", name)))
    }
}

/// Assert that an event with the given name was not emitted
pub fn assert_event_not_emitted(name: &str) -> Result<(), TestError> {
    if !MockEvents::has_event(name) {
        Ok(())
    } else {
        Err(TestError::new(format!("Event emitted when it should not be: {}", name)))
    }
}

/// Assert that the number of events with the given name matches the expected count
pub fn assert_event_count(name: &str, expected: usize) -> Result<(), TestError> {
    let actual = MockEvents::get_event_count_by_name(name);
    if actual == expected {
        Ok(())
    } else {
        Err(TestError::new(format!(
            "Event count for {} does not match. Expected {}, got {}",
            name, expected, actual
        )))
    }
}

/// Assert that the total number of events matches the expected count
pub fn assert_total_event_count(expected: usize) -> Result<(), TestError> {
    let actual = MockEvents::get_event_count();
    if actual == expected {
        Ok(())
    } else {
        Err(TestError::new(format!(
            "Total event count does not match. Expected {}, got {}",
            expected, actual
        )))
    }
}

/// Assert that an event with the given name and arguments was emitted
pub fn assert_event_args(name: &str, args: &[&[u8]]) -> Result<(), TestError> {
    let events = MockEvents::get_events_by_name(name);
    
    for event in events {
        if event.args.len() == args.len() {
            let mut all_match = true;
            
            for (i, expected_arg) in args.iter().enumerate() {
                if event.args[i] != *expected_arg {
                    all_match = false;
                    break;
                }
            }
            
            if all_match {
                return Ok(());
            }
        }
    }
    
    Err(TestError::new(format!(
        "Event {} with specified arguments not found",
        name
    )))
}

/// Assert that gas consumed is less than or equal to the expected value
pub fn assert_gas_consumed_le(expected: i64) -> Result<(), TestError> {
    let gas_consumed = 10_000_000 - MockRuntime::get_gas_left();
    if gas_consumed <= expected {
        Ok(())
    } else {
        Err(TestError::new(format!(
            "Gas consumed ({}) is greater than expected ({})",
            gas_consumed, expected
        )))
    }
}

/// Assert that a storage prefix contains a specific number of items
pub fn assert_storage_prefix_count(prefix: &[u8], expected: usize) -> Result<(), TestError> {
    let items = MockStorage::get_pairs_with_prefix(prefix);
    let actual = items.len();
    
    if actual == expected {
        Ok(())
    } else {
        Err(TestError::new(format!(
            "Storage prefix count for {:?} does not match. Expected {}, got {}",
            prefix, expected, actual
        )))
    }
}

/// Assert that a storage prefix contains specific key-value pairs
pub fn assert_storage_prefix_contains(
    prefix: &[u8],
    expected_pairs: &[(&[u8], &[u8])],
) -> Result<(), TestError> {
    let items = MockStorage::get_pairs_with_prefix(prefix);
    
    for (expected_key, expected_value) in expected_pairs {
        let full_key = [prefix, *expected_key].concat();
        let found = items.iter().any(|(k, v)| *k == full_key && *v == expected_value.to_vec());
        
        if !found {
            return Err(TestError::new(format!(
                "Storage prefix {:?} does not contain key {:?} with value {:?}",
                prefix, expected_key, expected_value
            )));
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_storage_assertions() {
        // Setup
        MockStorage::reset();
        MockStorage::put(b"key1", b"value1");
        MockStorage::put(b"key2", b"value2");
        
        // Test assert_storage_has
        assert!(assert_storage_has(b"key1").is_ok());
        assert!(assert_storage_has(b"nonexistent").is_err());
        
        // Test assert_storage_not_has
        assert!(assert_storage_not_has(b"nonexistent").is_ok());
        assert!(assert_storage_not_has(b"key1").is_err());
        
        // Test assert_storage_equals
        assert!(assert_storage_equals(b"key1", b"value1").is_ok());
        assert!(assert_storage_equals(b"key1", b"wrong").is_err());
        assert!(assert_storage_equals(b"nonexistent", b"value").is_err());
        
        // Test assert_storage_contains
        let pairs = [
            (b"key1" as &[u8], b"value1" as &[u8]),
            (b"key2" as &[u8], b"value2" as &[u8]),
        ];
        assert!(assert_storage_contains(&pairs).is_ok());
        
        let wrong_pairs = [
            (b"key1" as &[u8], b"value1" as &[u8]),
            (b"key2" as &[u8], b"wrong" as &[u8]),
        ];
        assert!(assert_storage_contains(&wrong_pairs).is_err());
    }
    
    #[test]
    fn test_runtime_assertions() {
        // Setup
        MockRuntime::reset();
        MockRuntime::add_witness(b"witness1");
        MockRuntime::set_block_height(100);
        MockRuntime::set_time(1620000000000);
        
        // Test assert_witness_verified
        assert!(assert_witness_verified(b"witness1").is_ok());
        assert!(assert_witness_verified(b"witness2").is_err());
        
        // Test assert_witness_not_verified
        assert!(assert_witness_not_verified(b"witness2").is_ok());
        assert!(assert_witness_not_verified(b"witness1").is_err());
        
        // Test assert_block_height
        assert!(assert_block_height(100).is_ok());
        assert!(assert_block_height(200).is_err());
        
        // Test assert_timestamp
        assert!(assert_timestamp(1620000000000).is_ok());
        assert!(assert_timestamp(1620000000001).is_err());
    }
    
    #[test]
    fn test_event_assertions() {
        // Setup
        MockEvents::reset();
        MockEvents::emit("Event1", vec![vec![1, 2, 3]], 1000);
        MockEvents::emit("Event1", vec![vec![4, 5, 6]], 2000);
        MockEvents::emit("Event2", vec![vec![7, 8, 9]], 3000);
        
        // Test assert_event_emitted
        assert!(assert_event_emitted("Event1").is_ok());
        assert!(assert_event_emitted("Event2").is_ok());
        assert!(assert_event_emitted("Event3").is_err());
        
        // Test assert_event_not_emitted
        assert!(assert_event_not_emitted("Event3").is_ok());
        assert!(assert_event_not_emitted("Event1").is_err());
        
        // Test assert_event_count
        assert!(assert_event_count("Event1", 2).is_ok());
        assert!(assert_event_count("Event2", 1).is_ok());
        assert!(assert_event_count("Event1", 1).is_err());
        
        // Test assert_total_event_count
        assert!(assert_total_event_count(3).is_ok());
        assert!(assert_total_event_count(2).is_err());
        
        // Test assert_event_args
        assert!(assert_event_args("Event1", &[b"\x01\x02\x03"]).is_ok());
        assert!(assert_event_args("Event1", &[b"\x04\x05\x06"]).is_ok());
        assert!(assert_event_args("Event1", &[b"\x07\x08\x09"]).is_err());
    }
    
    #[test]
    fn test_storage_prefix_assertions() {
        // Setup
        MockStorage::reset();
        MockStorage::put(b"prefix1:key1", b"value1");
        MockStorage::put(b"prefix1:key2", b"value2");
        MockStorage::put(b"prefix2:key3", b"value3");
        
        // Test assert_storage_prefix_count
        assert!(assert_storage_prefix_count(b"prefix1:", 2).is_ok());
        assert!(assert_storage_prefix_count(b"prefix2:", 1).is_ok());
        assert!(assert_storage_prefix_count(b"prefix1:", 3).is_err());
        
        // Test assert_storage_prefix_contains
        let prefix1_pairs = [
            (b"key1" as &[u8], b"value1" as &[u8]),
            (b"key2" as &[u8], b"value2" as &[u8]),
        ];
        assert!(assert_storage_prefix_contains(b"prefix1:", &prefix1_pairs).is_ok());
        
        let wrong_pairs = [
            (b"key1" as &[u8], b"value1" as &[u8]),
            (b"key2" as &[u8], b"wrong" as &[u8]),
        ];
        assert!(assert_storage_prefix_contains(b"prefix1:", &wrong_pairs).is_err());
        
        // Non-existent prefix
        assert!(assert_storage_prefix_count(b"nonexistent:", 0).is_ok());
    }
}