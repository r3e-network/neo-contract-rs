// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![allow(unused)]

use crate::types::{placeholder::*, *};

#[link(wasm_import_module = "neo.stdlib")]
#[allow(improper_ctypes)]
#[cfg(target_family = "wasm")]
extern "C" {
    /// `base58_encode` encodes a string to base58.
    pub(crate) fn base58_encode(data: ByteString) -> ByteString;

    /// `base58_decode` decodes a base58 string to a string.
    pub(crate) fn base58_decode(data: ByteString) -> ByteString;

    /// `base58check_encode` encodes a string to base58check.
    pub(crate) fn base58check_encode(data: ByteString) -> ByteString;

    /// `base58check_decode` decodes a base58check string to a string.
    pub(crate) fn base58check_decode(data: ByteString) -> ByteString;

    /// `base64_encode` encodes a string to base64.
    pub(crate) fn base64_encode(data: ByteString) -> ByteString;

    /// `base64_decode` decodes a base64 string to a string.
    pub(crate) fn base64_decode(data: ByteString) -> ByteString;

    /// `serialize` serializes a placeholder to a byte string.
    pub(crate) fn serialize(data: Placeholder) -> ByteString;

    /// `deserialize` deserializes a byte string to a placeholder.
    pub(crate) fn deserialize(data: ByteString) -> Placeholder;

    /// `json_serialize` serializes a string to JSON.
    pub(crate) fn json_serialize(data: Placeholder) -> ByteString;

    /// `json_deserialize` deserializes a JSON string to a string.
    pub(crate) fn json_deserialize(data: ByteString) -> Placeholder;

    /// `hex_encode` encodes a string to hex.
    pub(crate) fn hex_encode(data: ByteString) -> ByteString;

    /// `hex_decode` decodes a hex string to a string.
    pub(crate) fn hex_decode(data: ByteString) -> ByteString;

    /// `string_split` splits a string into an array of strings.
    pub(crate) fn string_split(data: ByteString, separator: ByteString) -> ByteString;
}
