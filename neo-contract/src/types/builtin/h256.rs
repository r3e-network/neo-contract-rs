//! H256 type for Neo Contract RS
//!
//! This module defines the H256 type, which is used for transaction hashes and other 256-bit hashes in Neo.

use core::fmt;
use core::ops::Deref;
use core::convert::TryFrom;
use alloc::string::String;

use alloc::format;

/// H256 represents a 256-bit hash (32 bytes) like a transaction hash
#[derive(PartialEq, Eq, Clone, Copy, Hash, Default)]
/// H256 represents a 256-bit hash value, commonly used for tx hashes in Neo
/// Uses repr(transparent) to ensure FFI compatibility with Neo VM
#[repr(transparent)]
pub struct H256(pub [u8; 32]);

impl H256 {
    /// Creates a new H256 with all zeros
    pub fn zero() -> Self {
        H256([0; 32])
    }
    
    /// Checks if the H256 is all zeros
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }
    
    /// Creates an H256 from a slice
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut bytes = [0u8; 32];
        if slice.len() >= 32 {
            bytes.copy_from_slice(&slice[..32]);
        } else {
            bytes[..slice.len()].copy_from_slice(slice);
        }
        H256(bytes)
    }
    
    /// Returns the bytes as a slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    
    /// Converts a hex string to H256
    pub fn from_hex(hex: &str) -> Option<Self> {
        // Remove 0x prefix if present
        let hex = if hex.starts_with("0x") { &hex[2..] } else { hex };
        
        // Check length
        if hex.len() != 64 {
            return None;
        }
        
        // Parse bytes
        let mut bytes = [0u8; 32];
        for i in 0..32 {
            let byte_str = &hex[i*2..i*2+2];
            bytes[i] = u8::from_str_radix(byte_str, 16).ok()?;
        }
        
        Some(H256(bytes))
    }
    
    /// Converts the H256 to a hex string
    pub fn to_hex(&self) -> String {
        let mut s = String::with_capacity(64);
        for byte in &self.0 {
            s.push_str(&format!("{:02x}", byte));
        }
        s
    }
}

impl Deref for H256 {
    type Target = [u8; 32];
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for H256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 32]> for H256 {
    fn from(bytes: [u8; 32]) -> Self {
        H256(bytes)
    }
}

impl TryFrom<&[u8]> for H256 {
    type Error = ();
    
    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        if slice.len() < 32 {
            return Err(());
        }
        
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&slice[..32]);
        Ok(H256(bytes))
    }
}

impl fmt::Debug for H256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "H256(0x{})", self.to_hex())
    }
}

impl fmt::Display for H256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", self.to_hex())
    }
}
