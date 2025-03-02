// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use alloc::string::String;

use crate::builtin::{ByteString, H160, H256, Int256};

/// Trait for types that can be used as storage keys
pub trait StorageKey {
    /// Convert the key to bytes
    fn to_bytes(&self) -> Vec<u8>;
}

// Implement StorageKey for common types
impl StorageKey for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        alloc::vec![*self]
    }
}

impl StorageKey for u16 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl StorageKey for u32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl StorageKey for u64 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl StorageKey for i8 {
    fn to_bytes(&self) -> Vec<u8> {
        alloc::vec![*self as u8]
    }
}

impl StorageKey for i16 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl StorageKey for i32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl StorageKey for i64 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl StorageKey for bool {
    fn to_bytes(&self) -> Vec<u8> {
        alloc::vec![if *self { 1 } else { 0 }]
    }
}

impl StorageKey for &str {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl StorageKey for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl StorageKey for ByteString {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl StorageKey for H160 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl StorageKey for H256 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl StorageKey for Int256 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl<T: StorageKey> StorageKey for &T {
    fn to_bytes(&self) -> Vec<u8> {
        (*self).to_bytes()
    }
}

// Implement StorageKey for byte arrays of various sizes
impl StorageKey for [u8; 0] {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl StorageKey for [u8; 5] {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl StorageKey for [u8; 8] {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl StorageKey for [u8; 11] {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl StorageKey for [u8; 13] {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl StorageKey for [u8; 9] {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}
