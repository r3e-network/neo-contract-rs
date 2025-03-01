use alloc::string::String;
// Copyright @ 2024 - present, R3E Network
use alloc::string::String;
// All Rights Reserved.
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
#[link(wasm_import_module = "neo.crypto")]
use alloc::string::String;
#[allow(improper_ctypes)]
#[cfg(target_family = "wasm")]
extern "C" {
    /// `sha256` computes the SHA-256 hash of the given data(ByteString or Buffer).
    pub(crate) fn sha256(data: Placeholder) -> H256;

    /// `ripemd160` computes the RIPEMD-160 hash of the given data(ByteString or Buffer).
    pub(crate) fn ripemd160(data: Placeholder) -> H160;

    /// `keccak256` computes the Keccak-256 hash of the given data(ByteString or Buffer).
    pub(crate) fn keccak256(data: Placeholder) -> H256;

    /// `verify_ecdsa` verifies the ECDSA signature of the given message with the given public key.
    pub(crate) fn verify_ecdsa(
        message: ByteString,
        public_key: PublicKey,
        sign: ByteString,
        named_curve_hash: NamedCurveHash,
    ) -> bool;

    pub(crate) fn verify_ed25519(
        message: ByteString,
        public_key: PublicKey,
        sign: ByteString,
    ) -> bool;
}
