// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use core::fmt;

/// Bytes represents a byte array
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    /// Create a new Bytes
    pub fn new(bytes: Vec<u8>) -> Self {
        Bytes(bytes)
    }

    /// Create an empty Bytes
    pub fn empty() -> Self {
        Bytes(Vec::new())
    }

    /// Get the bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Get the length
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the Bytes is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for Bytes {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bytes({})", self.len())
    }
}
