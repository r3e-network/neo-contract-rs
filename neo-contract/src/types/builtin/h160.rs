//! H160 type for Neo Contract RS
//!
//! This module defines the H160 type, which is used for addresses and script hashes in Neo.

use alloc::string::String;
use core::convert::TryFrom;
use core::fmt;
use core::ops::Deref;

use crate::utils::hex;
use alloc::format;

/// H160 represents a 160-bit hash (20 bytes) like an address or script hash
#[derive(PartialEq, Eq, Clone, Copy, Hash, Default)]
/// H160 represents a 160-bit hash value, commonly used for Neo addresses
/// Uses repr(transparent) to ensure FFI compatibility with Neo VM
#[repr(transparent)]
pub struct H160(pub [u8; 20]);

impl H160 {
    pub fn from_literal(p0: &str) -> Self { H160::from_slice(&hex::decode(p0).unwrap()) }
}

impl H160 {
    /// Creates a new H160 with all zeros
    pub fn zero() -> Self { H160([0; 20]) }

    /// Checks if the H160 is all zeros
    pub fn is_zero(&self) -> bool { self.0.iter().all(|&b| b == 0) }

    /// Creates an H160 from a slice
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut bytes = [0u8; 20];
        if slice.len() >= 20 {
            bytes.copy_from_slice(&slice[..20]);
        } else {
            bytes[..slice.len()].copy_from_slice(slice);
        }
        H160(bytes)
    }

    /// Returns the bytes as a slice
    pub fn as_bytes(&self) -> &[u8] { &self.0 }

    /// Converts a hex string to H160
    pub fn from_hex(hex: &str) -> Option<Self> {
        // Remove 0x prefix if present
        let hex = if hex.starts_with("0x") { &hex[2..] } else { hex };

        // Check length
        if hex.len() != 40 {
            return None;
        }

        // Parse bytes
        let mut bytes = [0u8; 20];
        for i in 0..20 {
            let byte_str = &hex[i * 2..i * 2 + 2];
            bytes[i] = u8::from_str_radix(byte_str, 16).ok()?;
        }

        Some(H160(bytes))
    }

    /// Converts the H160 to a hex string
    pub fn to_hex(&self) -> String {
        let mut s = String::with_capacity(40);
        for byte in &self.0 {
            s.push_str(&format!("{:02x}", byte));
        }
        s
    }
}

impl Deref for H160 {
    type Target = [u8; 20];

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl AsRef<[u8]> for H160 {
    fn as_ref(&self) -> &[u8] { &self.0 }
}

impl From<[u8; 20]> for H160 {
    fn from(bytes: [u8; 20]) -> Self { H160(bytes) }
}

impl TryFrom<&[u8]> for H160 {
    type Error = ();

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        if slice.len() < 20 {
            return Err(());
        }

        let mut bytes = [0u8; 20];
        bytes.copy_from_slice(&slice[..20]);
        Ok(H160(bytes))
    }
}

impl fmt::Debug for H160 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "H160(0x{})", self.to_hex()) }
}

impl fmt::Display for H160 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "0x{}", self.to_hex()) }
}
