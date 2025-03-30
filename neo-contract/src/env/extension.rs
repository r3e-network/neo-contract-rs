// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![allow(unused)]

#[cfg(target_family = "wasm")]
use crate::types::{placeholder::*, *};

#[link(wasm_import_module = "neo.extension")]
#[allow(improper_ctypes)]
#[cfg(target_family = "wasm")]
extern "C" {
    /// `h160_zero` returns a zero `H160`.
    pub(crate) fn h160_zero() -> H160;

    /// `h160_eq` checks if two `H160` are equal.
    pub(crate) fn h160_eq(a: H160, b: H160) -> bool;

    /// `h256_zero` returns a zero `H256`.
    pub(crate) fn h256_zero() -> H256;

    /// `h256_eq` checks if two `H256` are equal.
    pub(crate) fn h256_eq(a: H256, b: H256) -> bool;

    /// `h160_to_byte_string` converts a `H160` to a `ByteString`.
    pub(crate) fn h160_to_byte_string(src: H160) -> ByteString;

    /// `h256_to_byte_string` converts a `H256` to a `ByteString`.
    pub(crate) fn h256_to_byte_string(src: H256) -> ByteString;

    /// `h160_from_byte_string` converts a `ByteString` to a `H160`.
    pub(crate) fn h160_from_byte_string(src: ByteString) -> H160;

    /// `h256_from_byte_string` converts a `ByteString` to a `H256`.
    pub(crate) fn h256_from_byte_string(src: ByteString) -> H256;

    /// `byte_to_string` converts a `u8` to a `ByteString`.
    pub(crate) fn byte_to_string(src: u8) -> ByteString;

    /// `nullable_is_null` checks if a `Placeholder` is null.
    pub(crate) fn nullable_is_null(src: Placeholder) -> bool;

    /// `nullable_null` returns a null `Placeholder`.
    pub(crate) fn nullable_null() -> Placeholder;

    /// `int256_to_byte_string` converts a `Int256` to a `ByteString`.
    pub(crate) fn int256_to_byte_string(src: Int256) -> ByteString;

    /// `int256_from_byte_string` converts a `ByteString` to a `Int256`.
    pub(crate) fn int256_from_byte_string(src: ByteString) -> Int256;

    /// `placeholder_to_bool` converts a `Placeholder` to a `bool`.
    pub(crate) fn placeholder_to_bool(src: Placeholder) -> bool;

    /// `placeholder_from_bool` converts a `bool` to a `Placeholder`.
    pub(crate) fn placeholder_from_bool(src: bool) -> Placeholder;

    /// `placeholder_to_u8` converts a `Placeholder` to a `u8`.
    pub(crate) fn placeholder_to_u8(src: Placeholder) -> u8;

    /// `placeholder_from_u8` converts a `u8` to a `Placeholder`.
    pub(crate) fn placeholder_from_u8(src: u8) -> Placeholder;

    /// `placeholder_to_u16` converts a `Placeholder` to a `u16`.
    pub(crate) fn placeholder_to_u16(src: Placeholder) -> u16;

    /// `placeholder_from_u16` converts a `u16` to a `Placeholder`.
    pub(crate) fn placeholder_from_u16(src: u16) -> Placeholder;

    /// `placeholder_to_u32` converts a `Placeholder` to a `u32`.
    pub(crate) fn placeholder_to_u32(src: Placeholder) -> u32;

    /// `placeholder_from_u32` converts a `u32` to a `Placeholder`.
    pub(crate) fn placeholder_from_u32(src: u32) -> Placeholder;

    /// `placeholder_to_u64` converts a `Placeholder` to a `u64`.
    pub(crate) fn placeholder_to_u64(src: Placeholder) -> u64;

    /// `placeholder_from_u64` converts a `u64` to a `Placeholder`.
    pub(crate) fn placeholder_from_u64(src: u64) -> Placeholder;

    // `placeholder_to_usize` converts a `Placeholder` to a `usize`.
    pub(crate) fn placeholder_to_usize(src: Placeholder) -> usize;

    /// `placeholder_from_usize` converts a `usize` to a `Placeholder`.
    pub(crate) fn placeholder_from_usize(src: usize) -> Placeholder;

    // `placeholder_to_i8` converts a `Placeholder` to a `i8`.
    pub(crate) fn placeholder_to_i8(src: Placeholder) -> i8;

    /// `placeholder_from_i8` converts a `i8` to a `Placeholder`.
    pub(crate) fn placeholder_from_i8(src: i8) -> Placeholder;

    // `placeholder_to_i16` converts a `Placeholder` to a `i16`.
    pub(crate) fn placeholder_to_i16(src: Placeholder) -> i16;

    /// `placeholder_from_i16` converts a `i16` to a `Placeholder`.
    pub(crate) fn placeholder_from_i16(src: i16) -> Placeholder;

    // `placeholder_to_i32` converts a `Placeholder` to a `i32`.
    pub(crate) fn placeholder_to_i32(src: Placeholder) -> i32;

    /// `placeholder_from_i32` converts a `i32` to a `Placeholder`.
    pub(crate) fn placeholder_from_i32(src: i32) -> Placeholder;

    // `placeholder_to_i64` converts a `Placeholder` to a `i64`.
    pub(crate) fn placeholder_to_i64(src: Placeholder) -> i64;

    /// `placeholder_from_i64` converts a `i64` to a `Placeholder`.
    pub(crate) fn placeholder_from_i64(src: i64) -> Placeholder;

    // `placeholder_to_isize` converts a `Placeholder` to a `isize`.
    pub(crate) fn placeholder_to_isize(src: Placeholder) -> isize;

    /// `placeholder_from_isize` converts a `isize` to a `Placeholder`.
    pub(crate) fn placeholder_from_isize(src: isize) -> Placeholder;

    /// `array_pack_1` packs a single value into an array.
    /// T value -> Array<T>[value]
    pub(crate) fn array_pack_1(item1: Placeholder) -> Placeholder;

    /// `array_pack_2` packs two values into an array.
    /// T value1, T value2 -> Array<T>[value1, value2]
    pub(crate) fn array_pack_2(item1: Placeholder, item2: Placeholder) -> Placeholder;

    /// `array_pack_3` packs three values into an array.
    /// T value1, T value2, T value3 -> Array<T>[value1, value2, value3]
    pub(crate) fn array_pack_3(item1: Placeholder, item2: Placeholder, item3: Placeholder) -> Placeholder;

    /// `array_pack_4` packs four values into an array.
    /// T value1, T value2, T value3, T value4 -> Array<T>[value1, value2, value3, value4]
    pub(crate) fn array_pack_4(item1: Placeholder, item2: Placeholder, item3: Placeholder, item4: Placeholder) -> Placeholder;
}
