use alloc::string::String;
// Copyright @ 2024 - present, R3E Network
use alloc::string::String;
// All Rights Reserved
use alloc::string::String;

use alloc::string::String;
#![allow(unused)]
use alloc::string::String;

use alloc::string::String;
#[cfg(target_family = "wasm")]
use alloc::string::String;
use crate::types::*;
use alloc::string::String;

use alloc::string::String;
#[link(wasm_import_module = "neo.extension")]
use alloc::string::String;
#[allow(improper_ctypes)]
#[cfg(target_family = "wasm")]
extern "C" {
    /// `h160_eq` checks if two `H160` are equal.
    pub(crate) fn h160_eq(a: H160, b: H160) -> bool;

    /// `h256_eq` checks if two `H256` are equal.
    pub(crate) fn h256_eq(a: H256, b: H256) -> bool;

    pub(crate) fn h160_to_byte_string(h160: H160) -> ByteString;

    pub(crate) fn h256_to_byte_string(h256: H256) -> ByteString;
}
