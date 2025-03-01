// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::format;
use alloc::vec::Vec;
use core::fmt;
use core::ops::{Deref, DerefMut};
use crate::utils::hex;

/// H160 represents a 160-bit hash
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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

    /// Get the bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Convert to a hex string
    pub fn to_hex(&self) -> String {
        let mut hex = String::with_capacity(40);
        for byte in self.0.iter() {
            hex.push_str(&format!("{:02x}", byte));
        }
        hex
    }
    
    /// Convert to a hex string with 0x prefix
    pub fn to_hex_string(&self) -> String {
        let mut hex = String::with_capacity(42);
        hex.push_str("0x");
        for byte in self.0.iter() {
            hex.push_str(&format!("{:02x}", byte));
        }
        hex
    }

    /// Decode a hex string to an H160
    pub fn hex_decode(hex: &str) -> Option<Self> {
        let bytes = hex::decode(hex)?;
        if bytes.len() != 20 {
            return None;
        }
        let mut result = [0; 20];
        result.copy_from_slice(&bytes);
        Some(H160(result))
    }
}

impl Deref for H160 {
    type Target = [u8; 20];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for H160 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<[u8; 20]> for H160 {
    fn from(bytes: [u8; 20]) -> Self {
        H160(bytes)
    }
}

impl From<H160> for [u8; 20] {
    fn from(h160: H160) -> Self {
        h160.0
    }
}

impl fmt::Display for H160 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", self.to_hex())
    }
}
