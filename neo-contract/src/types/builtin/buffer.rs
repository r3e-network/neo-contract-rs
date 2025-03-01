// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use core::fmt;
use crate::types::builtin::string::ByteString;

/// Buffer represents a byte buffer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buffer {
    /// Data
    pub data: Vec<u8>,
}

impl Buffer {
    /// Create a new buffer
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    /// Create a buffer from a byte array
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            data: bytes.to_vec(),
        }
    }

    /// Get the length of the buffer
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get the bytes of the buffer
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Get a mutable reference to the bytes of the buffer
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Convert the buffer to a ByteString
    pub fn to_byte_string(&self) -> ByteString {
        ByteString::from(self.data.clone())
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buffer({})", self.len())
    }
}
