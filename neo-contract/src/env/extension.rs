// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![allow(unused)]

#[cfg(target_family = "wasm")]
use crate::types::*;

#[link(wasm_import_module = "neo.extension")]
#[allow(improper_ctypes)]
#[cfg(target_family = "wasm")]
extern "C" {
    /// `h160_eq` checks if two `H160` are equal.
    pub(crate) fn h160_eq(a: H160, b: H160) -> bool;

    /// `h256_eq` checks if two `H256` are equal.
    pub(crate) fn h256_eq(a: H256, b: H256) -> bool;

    pub(crate) fn h160_to_byte_string(src: H160) -> ByteString;

    pub(crate) fn h256_to_byte_string(src: H256) -> ByteString;

    pub(crate) fn h160_from_byte_string(src: ByteString) -> H160;

    pub(crate) fn h256_from_byte_string(src: ByteString) -> H256;

    pub(crate) fn concat_u8_byte_string(prefix: u8, src: ByteString) -> ByteString;

    pub(crate) fn nullable_is_null(src: Placeholder) -> bool;

    pub(crate) fn nullable_null() -> Placeholder;

    pub(crate) fn int256_to_byte_string(src: Int256) -> ByteString;

    pub(crate) fn int256_from_byte_string(src: ByteString) -> Int256;
}
