// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use crate::utils::hex;

/// PublicKey represents a public key
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey(pub [u8; 33]);

/// Role represents a Neo role
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    /// StateValidator
    StateValidator = 4,
    /// Oracle
    Oracle = 8,
    /// NeoFSAlphabetNode
    NeoFSAlphabetNode = 16,
}

/// ContractParamType represents a contract parameter type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractParamType {
    /// Any
    Any = 0,
    /// Boolean
    Boolean = 1,
    /// Integer
    Integer = 2,
    /// ByteArray
    ByteArray = 3,
    /// String
    String = 4,
    /// Hash160
    Hash160 = 5,
    /// Hash256
    Hash256 = 6,
    /// PublicKey
    PublicKey = 7,
    /// Signature
    Signature = 8,
    /// Array
    Array = 16,
    /// Map
    Map = 17,
    /// InteropInterface
    InteropInterface = 32,
    /// Void
    Void = 255,
}

impl PublicKey {
    /// Create a new PublicKey
    pub fn new(bytes: [u8; 33]) -> Self {
        PublicKey(bytes)
    }

    /// Create a zero PublicKey
    pub fn zero() -> Self {
        PublicKey([0; 33])
    }

    /// Get the bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Convert to a hex string
    pub fn to_hex(&self) -> String {
        let mut hex = String::with_capacity(66);
        for byte in self.0.iter() {
            hex.push_str(&format!("{:02x}", byte));
        }
        hex
    }

    /// Decode a hex string to a PublicKey
    pub fn hex_decode(hex: &str) -> Option<Self> {
        let bytes = hex::decode(hex)?;
        if bytes.len() != 33 {
            return None;
        }
        let mut result = [0; 33];
        result.copy_from_slice(&bytes);
        Some(PublicKey(result))
    }

    /// Check if the public key is valid
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        true // TODO: implement
    }
    
    /// Create a PublicKey from bytes
    #[cfg(not(target_family = "wasm"))]
    pub fn from_bytes(bytes: [u8; 33]) -> Self {
        Self(bytes)
    }
    
    /// Create a PublicKey from bytes (WASM version)
    #[cfg(target_family = "wasm")]
    pub fn from_bytes(bytes: [u8; 33]) -> Self {
        Self(bytes)
    }
    
    /// Convert to a string representation
    pub fn to_string(&self) -> String {
        self.to_hex()
    }
}
