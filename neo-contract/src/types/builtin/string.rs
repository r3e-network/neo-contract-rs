// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::{placeholder::*, *}};

/// ByteString is NOT an utf-8 string
#[cfg(not(target_family = "wasm"))]
#[repr(C)]
#[derive(Hash)]
pub struct ByteString(Vec<u8>);

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct ByteString(Placeholder);

#[cfg(target_family = "wasm")]
impl ByteString {
    #[inline(always)]
    pub fn empty() -> Self {
        unsafe { env::asm::string_empty() }
    }

    #[inline(always)]
    pub fn one_byte<const BYTE: u8>() -> Self {
        unsafe { env::extension::byte_to_string(BYTE) }
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
    pub fn concat(&self, other: Self) -> Self {
        unsafe { env::asm::string_concat(Self(self.0), other) }
    }

    #[inline(always)]
    pub fn hex_encode(&self) -> Self {
        unsafe { env::stdlib::hex_encode(Self(self.0)) }
    }

    #[inline(always)]
    pub fn hex_decode(&self) -> Self {
        unsafe { env::stdlib::hex_decode(Self(self.0)) }
    }

    #[inline(always)]
    pub fn base58_encode(&self) -> Self {
        unsafe { env::stdlib::base58_encode(Self(self.0)) }
    }

    #[inline(always)]
    pub fn base58_decode(&self) -> Self {
        unsafe { env::stdlib::base58_decode(Self(self.0)) }
    }

    #[inline(always)]
    pub fn base58check_encode(&self) -> Self {
        unsafe { env::stdlib::base58check_encode(Self(self.0)) }
    }

    #[inline(always)]
    pub fn base58check_decode(&self) -> Self {
        unsafe { env::stdlib::base58check_decode(Self(self.0)) }
    }

    #[inline(always)]
    pub fn base64_encode(&self) -> Self {
        unsafe { env::stdlib::base64_encode(Self(self.0)) }
    }

    #[inline(always)]
    pub fn base64_decode(&self) -> Self {
        unsafe { env::stdlib::base64_decode(Self(self.0)) }
    }

    #[inline(always)]
    pub fn sha256(&self) -> H256 {
        unsafe { env::crypto::sha256(self.0) }
    }

    #[inline(always)]
    pub fn ripemd160(&self) -> H160 {
        unsafe { env::crypto::ripemd160(self.0) }
    }

    #[inline(always)]
    pub fn keccak256(&self) -> H256 {
        unsafe { env::crypto::keccak256(self.0) }
    }
}

#[cfg(not(target_family = "wasm"))]
impl ByteString {
    pub(crate) fn new(value: String) -> Self {
        Self(value.into())
    }

    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn one_byte<const BYTE: u8>() -> Self {
        Self(vec![BYTE])
    }

    pub(crate) fn with_bytes(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
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

    pub fn concat(&self, other: Self) -> Self {
        let mut vec = self.0.clone();
        vec.extend(other.0.clone());
        Self(vec)
    }

    pub fn hex_encode(&self) -> Self {
        Self(hex::encode(self.0.as_slice()).into_bytes())
    }

    pub fn hex_decode(&self) -> Self {
        Self(hex::decode(self.0.as_slice()).unwrap())
    }

    pub(crate) fn to_string(self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }
}

impl Default for ByteString {
    #[inline(always)]
    fn default() -> Self {
        Self::empty()
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

/// convert the type as a ByteString
/// like reinterpret cast
pub trait IntoByteString {
    fn into_byte_string(self) -> ByteString;
}

pub trait FromByteString {
    fn from_byte_string(src: ByteString) -> Self;
}
