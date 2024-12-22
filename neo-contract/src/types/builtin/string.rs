// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};

/// ByteString is a non utf-8 string
#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct ByteString(Vec<u8>);

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct ByteString(Placeholder);

#[cfg(target_family = "wasm")]
impl ByteString {
    #[inline(always)]
    pub fn len(&self) -> usize {
        unsafe { env::asm::string_len(Self(self.0)) }
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
}

#[cfg(not(target_family = "wasm"))]
impl ByteString {
    pub(crate) fn new(value: String) -> Self {
        Self(value.into())
    }

    pub fn len(&self) -> usize {
        self.0.len()
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

    pub(crate) fn to_string(self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }
}

impl PartialEq for ByteString {
    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn eq(&self, other: &Self) -> bool {
        unsafe { env::asm::string_eq(Self(self.0), Self(other.0)) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ByteString {}

impl Clone for ByteString {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(target_family = "wasm")]
crate::impl_placeholder!(ByteString);

pub trait IntoByteString {
    fn into_byte_string(self) -> ByteString;
}
