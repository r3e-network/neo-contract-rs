// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{
    env,
    types::{placeholder::*, *},
};

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct H256([u8; 32]);

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct H256(Placeholder);

impl H256 {
    pub const SIZE: usize = 32;

    #[inline(always)]
    #[rustfmt::skip]
    pub fn zero() -> Self {
        #[cfg(target_family = "wasm")]
        unsafe { env::extension::h256_zero() }

        #[cfg(not(target_family = "wasm"))]
        H256([0u8; 32])
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn hex_encode(&self) -> ByteString {
        let mut b = self.0.clone();
        b.reverse();
        ByteString::new("0x".to_string() + &hex::encode(b.as_slice()))
    }

    #[cfg(not(target_family = "wasm"))]
    pub(crate) fn hex_decode(hex: &str) -> Self {
        let hex = if hex.starts_with("0x") || hex.starts_with("0X") {
            &hex[2..]
        } else {
            hex
        };

        let bytes = hex::decode(hex).unwrap();
        let mut buf = [0u8; 32];
        buf.copy_from_slice(&bytes);

        buf.reverse();
        H256(buf)
    }
}

impl PartialEq for H256 {
    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn eq(&self, other: &Self) -> bool {
        unsafe { env::extension::h256_eq(*self, *other) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Clone for H256 {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Eq for H256 {}
impl Copy for H256 {}

#[cfg(target_family = "wasm")]
crate::impl_placeholder!(H256);

impl IntoByteString for H256 {
    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn into_byte_string(self) -> ByteString {
        unsafe { env::extension::h256_to_byte_string(self) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn into_byte_string(self) -> ByteString {
        ByteString::with_bytes(self.0.as_slice())
    }
}
