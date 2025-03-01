// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;
use core::fmt;
use core::cmp::Ordering;
use crate::utils::hex;

/// H160 represents a 160-bit hash
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct H160(pub [u8; 20]);

impl H160 {
    /// Create a new H160
    pub fn new(bytes: [u8; 20]) -> Self {
        H160(bytes)
    }

    /// Create a zero H160
    pub fn zero() -> Self {
        H160([0; 20])
    }

    /// Get the bytes of the H160
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Decode a hex string into a H160
    pub fn hex_decode(hex: &str) -> Option<Self> {
        let hex = if hex.starts_with("0x") {
            &hex[2..]
        } else {
            hex
        };

        if hex.len() != 40 {
            return None;
        }

        match hex::decode(hex) {
            Ok(bytes) => {
                let mut result = [0u8; 20];
                result.copy_from_slice(&bytes);
                Some(H160(result))
            },
            Err(_) => None
        }
    }
    
    /// Convert to a hex string
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }
    
    /// Convert to a hex string with 0x prefix
    pub fn to_hex_string(&self) -> String {
        format!("0x{}", self.to_hex())
    }
    
    /// Alias for hex_decode for compatibility
    pub fn from_hex_string(hex: &str) -> Self {
        Self::hex_decode(hex).unwrap_or_else(Self::zero)
    }
}

impl Default for H160 {
    fn default() -> Self {
        Self::zero()
    }
}

impl PartialOrd for H160 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for H160 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl fmt::Display for H160 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(&self.0))
    }
}
