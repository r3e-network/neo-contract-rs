// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::format;
use alloc::vec::Vec;
use core::fmt;
use core::ops::{Deref, DerefMut};
use crate::utils::hex;

/// H256 represents a 256-bit hash
#[derive(Debug, Clone, PartialEq, Eq, Default)]
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

    /// Get the bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Convert to a hex string
    pub fn to_hex(&self) -> String {
        let mut hex = String::with_capacity(64);
        for byte in self.0.iter() {
            hex.push_str(&format!("{:02x}", byte));
        }
        hex
    }

    /// Decode a hex string to an H256
    pub fn hex_decode(hex: &str) -> Option<Self> {
        let bytes = hex::decode(hex)?;
        if bytes.len() != 32 {
            return None;
        }
        let mut result = [0; 32];
        result.copy_from_slice(&bytes);
        Some(H256(result))
    }
}

impl Deref for H256 {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for H256 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<[u8; 32]> for H256 {
    fn from(bytes: [u8; 32]) -> Self {
        H256(bytes)
    }
}

impl From<H256> for [u8; 32] {
    fn from(h256: H256) -> Self {
        h256.0
    }
}

impl fmt::Display for H256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", self.to_hex())
    }
}
