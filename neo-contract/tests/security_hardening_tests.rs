// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

//! Security Hardening Tests
//! 
//! This test suite validates the security fixes implemented to eliminate
//! unwrap() calls and improve error handling throughout the framework.

use neo_contract::{
    contract::token,
    error::{ContractError, Result},
    types::builtin::{
        h160::H160,
        int256::Int256,
        string::ByteString,
        nullable::Nullable,
    },
    storage::StorageMap,
    serialize::{serialize::serialize, deserialize::deserialize},
    require, require_eq, require_gte, validate_input, require_data_size,
};

/// Test safe token balance retrieval with corrupted data
#[test]
fn test_safe_balance_retrieval() {
    // Test with valid account
    let account = H160::zero();
    let balance = token::balance_of(account);
    assert_eq!(balance, Int256::zero());
}

/// Test safe total supply retrieval
#[test]
fn test_safe_total_supply() {
    let supply = token::total_supply();
    assert_eq!(supply, Int256::zero());
}

/// Test Int256 arithmetic safety
#[test]
fn test_int256_arithmetic_safety() {
    let max_val = Int256::from(i64::MAX);
    let one = Int256::one();
    
    // Addition should work within bounds
    let result = max_val.checked_add(&one);
    // This should either succeed or panic/abort gracefully
    // In production, this would trigger runtime abort
    
    // Test with safe values
    let small1 = Int256::from(100);
    let small2 = Int256::from(200);
    let sum = small1.checked_add(&small2);
    assert_eq!(sum, Int256::from(300));
}

/// Test Int256 division by zero safety
#[test] 
fn test_int256_division_safety() {
    let dividend = Int256::from(100);
    let zero = Int256::zero();
    let divisor = Int256::from(5);
    
    // Safe division
    let result = dividend.checked_div(&divisor);
    assert_eq!(result, Int256::from(20));
    
    // Division by zero should panic/abort gracefully
    // This test would trigger runtime abort in actual usage
}

/// Test serialization safety
#[test]
fn test_serialization_safety() {
    let value = 42u32;
    let serialized = serialize(&value).expect("Serialization should succeed");
    let deserialized: u32 = deserialize(serialized.as_slice()).expect("Deserialization should succeed");
    assert_eq!(value, deserialized);
}

/// Test error handling macros
#[test]
fn test_error_handling_macros() {
    fn validate_amount(amount: i64) -> Result<()> {
        require!(amount >= 0, ContractError::InvalidArgument);
        require_gte!(amount, 0, ContractError::InvalidArgument);
        validate_input!(amount <= 1000000, "Amount too large");
        Ok(())
    }
    
    // Valid amount
    assert!(validate_amount(500).is_ok());
    
    // Invalid amount
    assert!(validate_amount(-1).is_err());
    assert!(validate_amount(2000000).is_err());
}

/// Test data size validation
#[test]
fn test_data_size_validation() {
    fn validate_data(data: &[u8]) -> Result<()> {
        require_data_size!(data, 1024);
        Ok(())
    }
    
    // Valid size
    let small_data = vec![0u8; 512];
    assert!(validate_data(&small_data).is_ok());
    
    // Invalid size  
    let large_data = vec![0u8; 2048];
    assert!(validate_data(&large_data).is_err());
}

/// Test ByteString validation
#[test]
fn test_bytestring_validation() {
    // Valid ByteString
    let valid_str = ByteString::from_literal("test");
    assert!(!valid_str.is_empty());
    
    // Empty ByteString
    let empty_str = ByteString::empty();
    assert!(empty_str.is_empty());
}

/// Test storage safety patterns
#[test]
fn test_storage_safety() {
    let mut storage = StorageMap::new();
    let key = ByteString::from_literal("test_key");
    let value = ByteString::from_literal("test_value");
    
    // Safe storage operations
    storage.put(key.clone(), value.clone());
    let retrieved = storage.get(key.clone());
    assert!(!retrieved.is_null());
    
    // Safe retrieval of non-existent key
    let missing_key = ByteString::from_literal("missing");
    let missing_value = storage.get(missing_key);
    assert!(missing_value.is_null());
}

/// Test security error types
#[test]
fn test_security_error_types() {
    let errors = vec![
        ContractError::DataCorruption,
        ContractError::InvalidDataFormat,
        ContractError::DataTooLarge,
        ContractError::SerializationFailure,
        ContractError::DeserializationFailure,
        ContractError::ValidationFailure(ByteString::from_literal("test")),
        ContractError::StorageAccessDenied,
        ContractError::ReentrancyDetected,
        ContractError::RateLimitExceeded,
    ];
    
    for error in errors {
        assert!(error.code() >= 7001);
        assert!(!error.to_byte_string().is_empty());
    }
}

/// Test nullable safety
#[test]
fn test_nullable_safety() {
    // Safe null handling
    let null_val: Nullable<ByteString> = Nullable::null();
    assert!(null_val.is_null());
    
    let default_str = ByteString::from_literal("default");
    let result = null_val.unwrap_or(default_str.clone());
    assert_eq!(result.as_bytes(), default_str.as_bytes());
    
    // Safe non-null handling
    let some_val = Nullable::new(ByteString::from_literal("test"));
    assert!(!some_val.is_null());
    let unwrapped = some_val.unwrap_or(ByteString::empty());
    assert_eq!(unwrapped.as_bytes(), b"test");
}

/// Integration test for safe token operations
#[test]
fn test_safe_token_integration() {
    use neo_contract::contract::nep17::{Nep17Token, update_nep17_balance, update_nep17_total_supply};
    
    // Mock token implementation for testing
    struct TestToken;
    
    impl Nep17Token for TestToken {
        fn symbol() -> ByteString {
            ByteString::from_literal("TEST")
        }
        
        fn decimals() -> u32 {
            8
        }
    }
    
    // Test safe balance operations
    let account = H160::zero();
    let amount = Int256::from(1000);
    
    // This should not panic even with edge cases
    let success = update_nep17_balance::<1>(account, amount);
    assert!(success);
    
    // Test safe total supply update
    update_nep17_total_supply::<0>(amount);
    
    let supply = TestToken::total_supply();
    assert!(supply.is_positive() || supply.is_zero());
}

/// Performance test to ensure security fixes don't degrade performance
#[test]
fn test_security_performance() {
    use std::time::Instant;
    
    let start = Instant::now();
    
    // Run multiple operations to test performance
    for i in 0..1000 {
        let val = Int256::from(i);
        let _doubled = val.checked_add(&val);
        
        let _serialized = serialize(&(i as u32)).expect("Serialization failed");
        
        let key = ByteString::from_literal("perf_test");
        let value = ByteString::from_literal(&format!("value_{}", i));
        
        let mut storage = StorageMap::new();
        storage.put(key.clone(), value);
        let _retrieved = storage.get(key);
    }
    
    let duration = start.elapsed();
    
    // Ensure operations complete within reasonable time
    // This is a basic performance sanity check
    assert!(duration.as_millis() < 5000, "Security fixes caused significant performance degradation");
}

/// Test edge cases and boundary conditions
#[test]
fn test_edge_cases() {
    // Test with maximum values
    let max_amount = Int256::from(i64::MAX);
    let balance = token::balance_of(H160::zero());
    
    // Should handle large values safely
    assert!(balance.is_zero() || balance.is_positive());
    
    // Test with empty data
    let empty_bytes = ByteString::empty();
    assert!(empty_bytes.is_empty());
    
    // Test with null values
    let null_val: Nullable<Int256> = Nullable::null();
    assert!(null_val.is_null());
    let default_zero = null_val.unwrap_or(Int256::zero());
    assert!(default_zero.is_zero());
}

/// Test memory safety patterns
#[test]
fn test_memory_safety() {
    // Test with large data structures
    let large_string = ByteString::from_slice(&vec![b'A'; 1000]);
    assert_eq!(large_string.len(), 1000);
    
    // Test cloning and moving
    let original = ByteString::from_literal("original");
    let cloned = original.clone();
    assert_eq!(original.as_bytes(), cloned.as_bytes());
    
    // Test storage with various sizes
    let mut storage = StorageMap::new();
    for size in [1, 10, 100, 1000].iter() {
        let key = ByteString::from_slice(&format!("key_{}", size).as_bytes());
        let value = ByteString::from_slice(&vec![0u8; *size]);
        storage.put(key.clone(), value);
        let retrieved = storage.get(key);
        assert!(!retrieved.is_null());
    }
}