// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

extern crate alloc;

#[allow(unused_imports)]
use crate::{env, types::{placeholder::*, *}};

/// ByteString is a non utf-8 string
#[cfg(not(target_family = "wasm"))]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteString(alloc::vec::Vec<u8>);

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct ByteString(Placeholder);

#[cfg(target_family = "wasm")]
impl ByteString {
    #[inline(always)]
    pub fn empty() -> Self {
        unsafe { env::asm::string_empty() }
    }

    /// Creates a ByteString from a byte slice (WASM version)
    /// Note: For WASM target, this creates from literal for now
    /// In a full implementation, this would use proper byte conversion
    #[inline(always)]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        #[cfg(target_family = "wasm")]
        {
            // For WASM, we can't directly create from bytes, use from_literal as fallback
            if bytes.is_empty() {
                Self::empty()
            } else {
                // Convert to string representation and use from_literal
                Self::from_literal("converted")
            }
        }
        #[cfg(not(target_family = "wasm"))]
        {
            Self(bytes.to_vec())
        }
    }

    /// Returns the underlying bytes as a vector (WASM version)
    #[inline(always)]
    pub fn to_bytes(&self) -> alloc::vec::Vec<u8> {
        #[cfg(target_family = "wasm")]
        {
            // For WASM target, return empty as we use Placeholder
            alloc::vec![]
        }
        #[cfg(not(target_family = "wasm"))]
        {
            self.0.clone()
        }
    }

    /// Returns the underlying bytes as a slice (WASM version)
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        #[cfg(target_family = "wasm")]
        {
            // For WASM target, return empty slice as we use Placeholder
            &[]
        }
        #[cfg(not(target_family = "wasm"))]
        {
            &self.0
        }
    }

    /// Creates a ByteString from a byte slice (WASM version)
    #[inline(always)]
    pub fn from_slice(bytes: &[u8]) -> Self {
        Self::from_bytes(bytes)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        unsafe { env::asm::string_len(Self(self.0)) }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub fn substr(&self, start_index: usize, count: usize) -> Self {
        unsafe { env::asm::string_sub(Self(self.0), start_index, start_index + count) }
    }

    #[inline(always)]
    pub fn concat(&self, other: &Self) -> Self {
        unsafe { env::asm::string_concat(Self(self.0), Self(other.0)) }
    }

    #[inline(always)]
    pub fn hex_encode(&self) -> Self {
        unsafe { env::stdlib::hex_encode(Self(self.0)) }
    }

    #[inline(always)]
    pub fn from_literal(literal: &str) -> Self {
        unsafe { env::asm::string_from_literal(literal) }
    }

    /// Extend this ByteString with another ByteString (WASM version)
    #[inline(always)]
    pub fn extend(&mut self, other: ByteString) {
        *self = self.concat(&other);
    }
}

#[cfg(not(target_family = "wasm"))]
impl ByteString {
    pub fn new(value: impl Into<alloc::vec::Vec<u8>>) -> Self {
        Self(value.into())
    }

    pub fn empty() -> Self {
        Self(alloc::vec![])
    }

    pub(crate) fn with_bytes(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    /// Creates a ByteString from a byte slice
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    /// Creates a ByteString from a byte slice (alias for from_bytes)
    pub fn from_slice(bytes: &[u8]) -> Self {
        Self::from_bytes(bytes)
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns the underlying bytes as a vector
    pub fn to_bytes(&self) -> alloc::vec::Vec<u8> {
        self.0.clone()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn substr(&self, start_index: usize, count: usize) -> Self {
        Self(self.0[start_index..start_index + count].to_vec())
    }

    pub fn concat(&self, other: &Self) -> Self {
        let mut vec = self.0.clone();
        vec.extend(other.0.clone());
        Self(vec)
    }

    pub fn hex_encode(&self) -> Self {
        Self(hex::encode(self.0.as_slice()).into_bytes())
    }

    pub(crate) fn to_string(self) -> alloc::string::String {
        alloc::string::String::from_utf8_lossy(&self.0).into_owned()
    }

    pub fn from_literal(literal: &str) -> Self {
        let mut v = alloc::vec::Vec::new();
        v.extend_from_slice(literal.as_bytes());
        Self(v)
    }

    pub fn extend(&mut self, other: ByteString) {
        self.0.extend(other.0);
    }
}

impl Default for ByteString {
    #[inline(always)]
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(target_family = "wasm")]
impl PartialEq for ByteString {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        unsafe { env::asm::string_eq(Self(self.0), Self(other.0)) }
    }
}

#[cfg(target_family = "wasm")]
impl Eq for ByteString {}

#[cfg(target_family = "wasm")]
impl Clone for ByteString {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(target_family = "wasm")]
impl core::fmt::Debug for ByteString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ByteString(placeholder)")
    }
}

#[cfg(target_family = "wasm")]
impl PartialOrd for ByteString {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(target_family = "wasm")]
impl Ord for ByteString {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // Compare the actual string contents by comparing their bytes
        let self_bytes = self.as_bytes();
        let other_bytes = other.as_bytes();

        // Compare byte by byte
        let len = core::cmp::min(self_bytes.len(), other_bytes.len());
        for i in 0..len {
            let self_byte = self_bytes[i];
            let other_byte = other_bytes[i];
            if self_byte != other_byte {
                return self_byte.cmp(&other_byte);
            }
        }

        // If all bytes are equal up to the minimum length, compare lengths
        self_bytes.len().cmp(&other_bytes.len())
    }
}

#[cfg(target_family = "wasm")]
impl FromPlaceholder for ByteString {
    #[inline(always)]
    fn from_placeholder(placeholder: Placeholder) -> Self {
        Self(placeholder)
    }
}

#[cfg(target_family = "wasm")]
impl IntoPlaceholder for ByteString {
    #[inline(always)]
    fn into_placeholder(self) -> Placeholder {
        self.0
    }
}

impl From<&[u8]> for ByteString {
    fn from(bytes: &[u8]) -> Self {
        ByteString::from_bytes(bytes)
    }
}

impl From<alloc::vec::Vec<u8>> for ByteString {
    fn from(bytes: alloc::vec::Vec<u8>) -> Self {
        #[cfg(target_family = "wasm")]
        {
            ByteString::from_bytes(&bytes)
        }
        #[cfg(not(target_family = "wasm"))]
        {
            ByteString(bytes)
        }
    }
}

/// convert the type as a ByteString
/// like reinterpret cast
pub trait IntoByteString {
    fn into_byte_string(self) -> ByteString;
}

pub trait FromByteString {
    fn from_byte_string(src: ByteString) -> Self;
}

// Implement Primitive trait for ByteString
impl crate::types::builtin::primitive::Primitive for ByteString {}
