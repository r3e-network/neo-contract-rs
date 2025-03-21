// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

//! Unit tests for neo-contract types.

#![cfg(test)]

use neo_contract::types::*;

#[test]
fn test_byte_string_empty() {
    let empty = ByteString::empty();
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
    assert_eq!(empty.to_bytes().len(), 0);
}

#[test]
fn test_byte_string_from_string() {
    let value = "Hello, Neo!";
    let bs = ByteString::from(value);
    
    assert_eq!(bs.len(), value.len());
    assert!(!bs.is_empty());
    
    // Convert back to string for comparison
    let bytes = bs.to_bytes();
    assert_eq!(bytes, value.as_bytes());
    
    // Test string content
    let str_value = String::from_utf8(bytes.to_vec()).unwrap();
    assert_eq!(str_value, value);
}

#[test]
fn test_byte_string_from_bytes() {
    let bytes = [0x01, 0x02, 0x03, 0x04, 0x05];
    let bs = ByteString::from_bytes(&bytes);
    
    assert_eq!(bs.len(), bytes.len());
    assert!(!bs.is_empty());
    
    // Test byte content
    let retrieved_bytes = bs.to_bytes();
    assert_eq!(retrieved_bytes, bytes);
}

#[test]
fn test_byte_string_concat() {
    let bs1 = ByteString::from("Hello, ");
    let bs2 = ByteString::from("Neo!");
    
    let concatenated = bs1.concat(&bs2);
    
    assert_eq!(concatenated.len(), bs1.len() + bs2.len());
    assert_eq!(concatenated.to_bytes(), b"Hello, Neo!");
    
    // Test with empty string
    let empty = ByteString::empty();
    let concat_with_empty1 = bs1.concat(&empty);
    let concat_with_empty2 = empty.concat(&bs1);
    
    assert_eq!(concat_with_empty1.to_bytes(), bs1.to_bytes());
    assert_eq!(concat_with_empty2.to_bytes(), bs1.to_bytes());
}

#[test]
fn test_byte_string_comparison() {
    let bs1 = ByteString::from("abc");
    let bs2 = ByteString::from("abc");
    let bs3 = ByteString::from("def");
    let bs4 = ByteString::from("abcdef");
    
    // Test equality
    assert_eq!(bs1, bs2);
    assert_ne!(bs1, bs3);
    assert_ne!(bs1, bs4);
    
    // Test comparison methods if available
    // This assumes ByteString implements comparison traits
    assert!(bs1 < bs3);  // "abc" < "def"
    assert!(bs1 < bs4);  // "abc" < "abcdef"
    assert!(bs3 > bs1);  // "def" > "abc"
    assert!(bs4 > bs1);  // "abcdef" > "abc"
}

#[test]
fn test_h160_zero() {
    let zero = H160::zero();
    let zero_bytes = zero.to_bytes();
    
    assert_eq!(zero_bytes.len(), 20);
    for byte in zero_bytes {
        assert_eq!(byte, 0);
    }
}

#[test]
fn test_h160_from_bytes() {
    let bytes = [1u8; 20]; // Create an array of 20 bytes with value 1
    let h160 = H160::from_bytes(&bytes);
    
    let retrieved_bytes = h160.to_bytes();
    assert_eq!(retrieved_bytes.len(), 20);
    assert_eq!(retrieved_bytes, bytes);
    
    // Test with different values
    let bytes2 = [5u8; 20];
    let h160_2 = H160::from_bytes(&bytes2);
    assert_eq!(h160_2.to_bytes(), bytes2);
}

#[test]
fn test_h160_equality() {
    let bytes1 = [1u8; 20];
    let bytes2 = [2u8; 20];
    
    let h160_1a = H160::from_bytes(&bytes1);
    let h160_1b = H160::from_bytes(&bytes1);
    let h160_2 = H160::from_bytes(&bytes2);
    
    // Test equality
    assert_eq!(h160_1a, h160_1b);
    assert_ne!(h160_1a, h160_2);
}

#[test]
fn test_int256_zero() {
    let zero = Int256::zero();
    
    assert!(zero.is_zero());
    assert!(!zero.is_negative());
    assert!(!zero.is_positive());
}

#[test]
fn test_int256_from_i32() {
    // Test positive value
    let positive = Int256::from_i32(42);
    assert!(!positive.is_zero());
    assert!(positive.is_positive());
    assert!(!positive.is_negative());
    assert_eq!(positive.to_i32(), 42);
    
    // Test negative value
    let negative = Int256::from_i32(-42);
    assert!(!negative.is_zero());
    assert!(!negative.is_positive());
    assert!(negative.is_negative());
    assert_eq!(negative.to_i32(), -42);
    
    // Test zero
    let zero = Int256::from_i32(0);
    assert!(zero.is_zero());
    assert!(!zero.is_positive());
    assert!(!zero.is_negative());
    assert_eq!(zero.to_i32(), 0);
}

#[test]
fn test_int256_arithmetic() {
    let a = Int256::from_i32(40);
    let b = Int256::from_i32(2);
    
    // Addition
    let sum = a.checked_add(&b);
    assert_eq!(sum.to_i32(), 42);
    
    // Subtraction
    let diff = a.checked_sub(&b);
    assert_eq!(diff.to_i32(), 38);
    
    // Multiplication
    let product = b.checked_mul(&a);
    assert_eq!(product.to_i32(), 80);
    
    // Division
    let quotient = a.checked_div(&b);
    assert_eq!(quotient.to_i32(), 20);
    
    // Negation
    let neg_a = a.checked_neg();
    assert_eq!(neg_a.to_i32(), -40);
}

#[test]
fn test_int256_serialization() {
    let n = Int256::from_i32(42);
    
    // Convert to ByteString
    let bs = n.into_byte_string();
    assert!(!bs.is_empty());
    
    // Convert back to Int256
    let m = Int256::from_byte_string(bs);
    assert_eq!(m.to_i32(), 42);
}

#[test]
fn test_array_creation() {
    let array = Array::<i32>::new();
    assert_eq!(array.len(), 0);
    
    let array_with_capacity = Array::<ByteString>::with_capacity(10);
    assert_eq!(array_with_capacity.len(), 0);
}

#[test]
fn test_array_operations() {
    let mut array = Array::<i32>::new();
    
    // Test empty array
    assert_eq!(array.len(), 0);
    
    // Test pushing elements
    array.push(1);
    array.push(2);
    array.push(3);
    assert_eq!(array.len(), 3);
    
    // Test getting elements
    assert_eq!(array.get(0), 1);
    assert_eq!(array.get(1), 2);
    assert_eq!(array.get(2), 3);
    
    // Test setting elements
    array.set(1, 42);
    assert_eq!(array.get(1), 42);
}

#[test]
fn test_array_of_byte_strings() {
    let mut array = Array::<ByteString>::new();
    
    // Add some ByteStrings
    array.push(ByteString::from("first"));
    array.push(ByteString::from("second"));
    
    assert_eq!(array.len(), 2);
    assert_eq!(array.get(0).to_bytes(), b"first");
    assert_eq!(array.get(1).to_bytes(), b"second");
} 