//! Extended StdLib Native Contract Functions
//! Additional encoding, serialization, and utility functions

use crate::prelude::*;
use crate::types::{ByteString, Int256, H160, Array, Any};

/// StdLib contract hash on Neo N3
pub const STDLIB_HASH: H160 = H160([
    0xac, 0xce, 0x6f, 0xd8, 0x0d, 0x76, 0x48, 0xc9, 0xc5, 0x7e,
    0x9c, 0x31, 0x59, 0x1a, 0x61, 0xec, 0xd2, 0x79, 0x3e, 0xf5,
]);

/// Extended StdLib functions
pub struct StdLibExtended;

impl StdLibExtended {
    /// Serialize an object to bytes
    pub fn serialize(item: Any) -> ByteString {
        crate::services::contract::Contract::call(
            STDLIB_HASH,
            ByteString::from_literal("serialize"),
            Array::from_vec(vec![item]),
            crate::services::contract::CallFlags::READ_ONLY,
        )
        .unwrap_or_default()
        .as_bytes()
        .unwrap_or_default()
    }

    /// Deserialize bytes to an object
    pub fn deserialize(data: ByteString) -> Any {
        crate::services::contract::Contract::call(
            STDLIB_HASH,
            ByteString::from_literal("deserialize"),
            Array::from_vec(vec![data.into_any()]),
            crate::services::contract::CallFlags::READ_ONLY,
        )
        .unwrap_or_default()
    }

    /// Base58 encode
    pub fn base58_encode(data: ByteString) -> ByteString {
        crate::services::contract::Contract::call(
            STDLIB_HASH,
            ByteString::from_literal("base58Encode"),
            Array::from_vec(vec![data.into_any()]),
            crate::services::contract::CallFlags::READ_ONLY,
        )
        .unwrap_or_default()
        .as_bytes()
        .unwrap_or_default()
    }

    /// Base58 decode
    pub fn base58_decode(data: ByteString) -> ByteString {
        crate::services::contract::Contract::call(
            STDLIB_HASH,
            ByteString::from_literal("base58Decode"),
            Array::from_vec(vec![data.into_any()]),
            crate::services::contract::CallFlags::READ_ONLY,
        )
        .unwrap_or_default()
        .as_bytes()
        .unwrap_or_default()
    }

    /// Base58 check encode (with checksum)
    pub fn base58_check_encode(data: ByteString) -> ByteString {
        crate::services::contract::Contract::call(
            STDLIB_HASH,
            ByteString::from_literal("base58CheckEncode"),
            Array::from_vec(vec![data.into_any()]),
            crate::services::contract::CallFlags::READ_ONLY,
        )
        .unwrap_or_default()
        .as_bytes()
        .unwrap_or_default()
    }

    /// Base58 check decode (verify checksum)
    pub fn base58_check_decode(data: ByteString) -> ByteString {
        crate::services::contract::Contract::call(
            STDLIB_HASH,
            ByteString::from_literal("base58CheckDecode"),
            Array::from_vec(vec![data.into_any()]),
            crate::services::contract::CallFlags::READ_ONLY,
        )
        .unwrap_or_default()
        .as_bytes()
        .unwrap_or_default()
    }

    /// Memory compare two byte arrays
    pub fn memory_compare(str1: ByteString, str2: ByteString) -> i32 {
        crate::services::contract::Contract::call(
            STDLIB_HASH,
            ByteString::from_literal("memoryCompare"),
            Array::from_vec(vec![str1.into_any(), str2.into_any()]),
            crate::services::contract::CallFlags::READ_ONLY,
        )
        .and_then(|v| v.as_int())
        .map(|v| v.to_i32().unwrap_or(0))
        .unwrap_or(0)
    }

    /// Memory search - find substring in string
    pub fn memory_search(
        str: ByteString,
        substr: ByteString,
        start: u32,
        backward: bool,
    ) -> i32 {
        crate::services::contract::Contract::call(
            STDLIB_HASH,
            ByteString::from_literal("memorySearch"),
            Array::from_vec(vec![
                str.into_any(),
                substr.into_any(),
                Int256::from(start as i64).into_any(),
                backward.into_any(),
            ]),
            crate::services::contract::CallFlags::READ_ONLY,
        )
        .and_then(|v| v.as_int())
        .map(|v| v.to_i32().unwrap_or(-1))
        .unwrap_or(-1)
    }

    /// String split
    pub fn string_split(str: ByteString, separator: ByteString) -> Array {
        crate::services::contract::Contract::call(
            STDLIB_HASH,
            ByteString::from_literal("stringSplit"),
            Array::from_vec(vec![str.into_any(), separator.into_any()]),
            crate::services::contract::CallFlags::READ_ONLY,
        )
        .and_then(|v| v.as_array())
        .unwrap_or_else(Array::new)
    }

    /// String concatenation (multiple strings)
    pub fn string_concat(strings: Array) -> ByteString {
        crate::services::contract::Contract::call(
            STDLIB_HASH,
            ByteString::from_literal("strConcat"),
            Array::from_vec(vec![strings.into_any()]),
            crate::services::contract::CallFlags::READ_ONLY,
        )
        .unwrap_or_default()
        .as_bytes()
        .unwrap_or_default()
    }

    /// Convert integer to string with specific base
    pub fn itoa_base(value: Int256, base: u8) -> ByteString {
        crate::services::contract::Contract::call(
            STDLIB_HASH,
            ByteString::from_literal("itoa"),
            Array::from_vec(vec![
                value.into_any(),
                Int256::from(base as i64).into_any(),
            ]),
            crate::services::contract::CallFlags::READ_ONLY,
        )
        .unwrap_or_default()
        .as_bytes()
        .unwrap_or_default()
    }

    /// Convert string to integer with specific base
    pub fn atoi_base(value: ByteString, base: u8) -> Int256 {
        crate::services::contract::Contract::call(
            STDLIB_HASH,
            ByteString::from_literal("atoi"),
            Array::from_vec(vec![
                value.into_any(),
                Int256::from(base as i64).into_any(),
            ]),
            crate::services::contract::CallFlags::READ_ONLY,
        )
        .and_then(|v| v.as_int())
        .unwrap_or_else(Int256::zero)
    }
}