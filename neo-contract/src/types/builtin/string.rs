// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::ops::{Deref, DerefMut};

/// ByteString represents a byte string
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteString(pub Vec<u8>);

impl ByteString {
    /// Create a new empty ByteString
    pub fn new() -> Self {
        ByteString(Vec::new())
    }

    /// Create a new ByteString from a byte array
    pub fn from_bytes(bytes: &[u8]) -> Self {
        ByteString(bytes.to_vec())
    }

    /// Create a new ByteString from a string
    pub fn from_string(s: &str) -> Self {
        ByteString(s.as_bytes().to_vec())
    }

    /// Get the underlying bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Get the length of the string
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the string is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// Custom implementation to convert ByteString to String
// We don't implement ToString directly to avoid conflict with the blanket impl
impl ByteString {
    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }
}

impl AsRef<[u8]> for ByteString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for ByteString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ByteString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

impl From<ByteString> for Vec<u8> {
    fn from(s: ByteString) -> Self {
        s.0
    }
}

impl fmt::Display for ByteString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
