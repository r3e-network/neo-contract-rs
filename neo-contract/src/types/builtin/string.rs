//! ByteString type for Neo Contract RS
//!
//! This module defines the ByteString type, which is used to represent strings
//! and binary data in Neo smart contracts.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
use core::ops::Deref;

/// ByteString represents a string or binary data in Neo
#[derive(PartialEq, Eq, Clone, Default)]
/// ByteString represents a string of bytes in Neo N3
/// Uses repr(transparent) to ensure FFI compatibility with Neo VM
#[repr(transparent)]
pub struct ByteString(pub Vec<u8>);

impl ByteString {
    /// Creates a new ByteString from a Vec<u8>
    pub fn new(bytes: Vec<u8>) -> Self { ByteString(bytes) }

    /// Creates an empty ByteString
    pub fn empty() -> Self { ByteString(Vec::new()) }

    /// Returns the bytes as a slice
    pub fn as_bytes(&self) -> &[u8] { &self.0 }

    /// Returns the length of the ByteString
    pub fn len(&self) -> usize { self.0.len() }

    /// Checks if the ByteString is empty
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Converts a byte slice to a ByteString
    pub fn from_bytes(bytes: &[u8]) -> Self { ByteString(bytes.to_vec()) }

    /// Creates a ByteString from a raw pointer
    pub fn from_raw(ptr: *const u8) -> Self {
        unsafe {
            extern "C" {
                fn neo_data_length(ptr: *const u8) -> usize;
                fn neo_data_copy(dest: *mut u8, src: *const u8, len: usize);
            }

            let data_len = neo_data_length(ptr);
            let mut data = Vec::with_capacity(data_len);

            neo_data_copy(data.as_mut_ptr(), ptr, data_len);
            data.set_len(data_len);

            ByteString(data)
        }
    }

    /// Attempts to convert the ByteString to a UTF-8 string
    pub fn to_utf8(&self) -> Option<String> { core::str::from_utf8(&self.0).ok().map(|s| s.to_string()) }
}

impl Deref for ByteString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl AsRef<[u8]> for ByteString {
    fn as_ref(&self) -> &[u8] { &self.0 }
}

impl From<&[u8]> for ByteString {
    fn from(bytes: &[u8]) -> Self { ByteString(bytes.to_vec()) }
}

impl From<Vec<u8>> for ByteString {
    fn from(bytes: Vec<u8>) -> Self { ByteString(bytes) }
}

impl From<&str> for ByteString {
    fn from(s: &str) -> Self { ByteString(s.as_bytes().to_vec()) }
}

impl From<String> for ByteString {
    fn from(s: String) -> Self { ByteString(s.into_bytes()) }
}

impl fmt::Debug for ByteString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match core::str::from_utf8(&self.0) {
            Ok(s) => write!(f, "ByteString(\"{}\")", s),
            Err(_) => {
                write!(f, "ByteString(0x")?;
                for byte in &self.0 {
                    write!(f, "{:02x}", byte)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl fmt::Display for ByteString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match core::str::from_utf8(&self.0) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => {
                write!(f, "0x")?;
                for byte in &self.0 {
                    write!(f, "{:02x}", byte)?;
                }
                Ok(())
            }
        }
    }
}

impl AsRef<str> for ByteString {
    fn as_ref(&self) -> &str {
        // Convert the bytes to a string slice, using utf8_unchecked
        // This is safe if we know the bytes are valid UTF-8.
        // For a production implementation, this would need to handle invalid UTF-8.
        unsafe {
            core::str::from_utf8_unchecked(self.as_bytes())
        }
    }
}
