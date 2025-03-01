// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;
use core::fmt;
use core::cmp::Ordering;
use crate::utils::hex;

/// H256 represents a 256-bit hash
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct H256(pub [u8; 32]);

impl H256 {
    /// Create a new H256
    pub fn new(bytes: [u8; 32]) -> Self {
        H256(bytes)
    }

    /// Create a zero H256
    pub fn zero() -> Self {
        H256([0; 32])
    }

    /// Get the bytes of the H256
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Decode a hex string into a H256
    pub fn hex_decode(hex: &str) -> Option<Self> {
        let hex = if hex.starts_with("0x") {
            &hex[2..]
        } else {
            hex
        };

        if hex.len() != 64 {
            return None;
        }

        match hex::decode(hex) {
            Ok(bytes) => {
                let mut result = [0u8; 32];
                result.copy_from_slice(&bytes);
                Some(H256(result))
            },
            Err(_) => None
        }
    }
}

impl Default for H256 {
    fn default() -> Self {
        Self::zero()
    }
}

impl PartialOrd for H256 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for H256 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl fmt::Display for H256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(&self.0))
    }
}
