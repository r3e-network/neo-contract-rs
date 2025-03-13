// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::utils::hex;
use alloc::string::String;
use alloc::vec::Vec;

/// ByteArray represents a static byte array in Neo N3
/// Uses repr(transparent) to ensure FFI compatibility with Neo VM
#[repr(transparent)]
pub struct ByteArray {
    pub value: Vec<u8>,
}

impl ByteArray {
    /// Create a new ByteArray
    pub fn new(value: Vec<u8>) -> Self { Self { value } }

    /// Create a ByteArray from a hex string
    pub fn from_hex(hex: &str) -> Self {
        match hex::decode(hex) {
            Ok(bytes) => Self::new(bytes),
            Err(_) => Self::new(Vec::new()),
        }
    }

    /// Get the bytes of the ByteArray
    pub fn as_bytes(&self) -> &[u8] { &self.value }

    /// Get the length of the ByteArray
    pub fn len(&self) -> usize { self.value.len() }

    /// Check if the ByteArray is empty
    pub fn is_empty(&self) -> bool { self.value.is_empty() }
}

/// Hash160 represents a static Hash160
pub struct Hash160 {
    pub value: [u8; 20],
}

impl Hash160 {
    /// Create a new Hash160
    pub fn new(value: [u8; 20]) -> Self { Self { value } }

    /// Create a Hash160 from a hex string
    pub fn from_hex(hex: &str) -> Self {
        if let Ok(bytes) = hex::decode(hex) {
            if bytes.len() == 20 {
                let mut value = [0u8; 20];
                value.copy_from_slice(&bytes);
                return Self::new(value);
            }
        }
        Self::new([0u8; 20])
    }

    /// Get the bytes of the Hash160
    pub fn as_bytes(&self) -> &[u8] { &self.value }
}

/// ContractHash represents a static contract hash
pub struct ContractHash {
    pub value: [u8; 20],
}

impl ContractHash {
    /// Create a new ContractHash
    pub fn new(value: [u8; 20]) -> Self { Self { value } }

    /// Create a ContractHash from a hex string
    pub fn from_hex(hex: &str) -> Self {
        if let Ok(bytes) = hex::decode(hex) {
            if bytes.len() == 20 {
                let mut value = [0u8; 20];
                value.copy_from_slice(&bytes);
                return Self::new(value);
            }
        }
        Self::new([0u8; 20])
    }

    /// Get the bytes of the ContractHash
    pub fn as_bytes(&self) -> &[u8] { &self.value }
}

/// Integer represents a static integer
pub struct Integer {
    pub value: i128,
}

impl Integer {
    /// Create a new Integer
    pub fn new(value: i128) -> Self { Self { value } }

    /// Get the value of the Integer
    pub fn value(&self) -> i128 { self.value }
}

/// PublicKey represents a static public key
pub struct PublicKey {
    pub value: [u8; 33],
}

impl PublicKey {
    /// Create a new PublicKey
    pub fn new(value: [u8; 33]) -> Self { Self { value } }

    /// Create a PublicKey from a hex string
    pub fn from_hex(hex: &str) -> Self {
        if let Ok(bytes) = hex::decode(hex) {
            if bytes.len() == 33 {
                let mut value = [0u8; 33];
                value.copy_from_slice(&bytes);
                return Self::new(value);
            }
        }
        Self::new([0u8; 33])
    }

    /// Get the bytes of the PublicKey
    pub fn as_bytes(&self) -> &[u8] { &self.value }
}

/// StaticString represents a static string
pub struct StaticString {
    pub value: String,
}

impl StaticString {
    /// Create a new StaticString
    pub fn new(value: String) -> Self { Self { value } }

    /// Get the value of the StaticString
    pub fn value(&self) -> &str { &self.value }
}
