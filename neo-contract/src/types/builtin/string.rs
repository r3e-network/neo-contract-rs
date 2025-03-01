// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

/// ByteString represents a byte string
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteString(pub Vec<u8>);

impl ByteString {
    /// Create a new ByteString
    pub fn new(bytes: Vec<u8>) -> Self {
        ByteString(bytes)
    }

    /// Create an empty ByteString
    pub fn empty() -> Self {
        ByteString(Vec::new())
    }

    /// Get the bytes of the ByteString
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Get the length of the ByteString
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the ByteString is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Extend the ByteString with a slice
    pub fn extend_from_slice(&mut self, slice: &[u8]) {
        self.0.extend_from_slice(slice);
    }
}

impl Default for ByteString {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for ByteString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ByteString({})", self.len())
    }
}

impl From<Vec<u8>> for ByteString {
    fn from(bytes: Vec<u8>) -> Self {
        ByteString(bytes)
    }
}

impl From<&[u8]> for ByteString {
    fn from(bytes: &[u8]) -> Self {
        ByteString(bytes.to_vec())
    }
}

impl From<String> for ByteString {
    fn from(s: String) -> Self {
        ByteString(s.into_bytes())
    }
}

impl From<&str> for ByteString {
    fn from(s: &str) -> Self {
        ByteString(s.as_bytes().to_vec())
    }
}

impl AsRef<[u8]> for ByteString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
