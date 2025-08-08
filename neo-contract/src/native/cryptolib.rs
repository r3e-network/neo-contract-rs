//! CryptoLib Native Contract
//! Provides cryptographic functions for Neo N3 smart contracts

use crate::prelude::*;
use crate::types::{ByteString, Int256, H160, PublicKey};

/// CryptoLib contract hash on Neo N3
pub const CRYPTO_LIB_HASH: H160 = H160([
    0x72, 0x6c, 0xb5, 0x77, 0x50, 0x2d, 0xbd, 0x44, 0x5f, 0xcd,
    0x2f, 0xc6, 0x95, 0x54, 0xbe, 0xce, 0xc7, 0x1f, 0xd8, 0xa3,
]);

/// CryptoLib native contract interface
pub struct CryptoLib;

impl CryptoLib {
    /// Computes SHA256 hash
    pub fn sha256(data: ByteString) -> ByteString {
        crate::services::contract::Contract::call(
            CRYPTO_LIB_HASH,
            ByteString::from_literal("sha256"),
            crate::types::CallFlags::READ_ONLY,
            crate::types::Array::from_vec(vec![data.into_any()]),
        )
        .as_bytes()
        .unwrap_or_else(ByteString::empty)
    }

    /// Computes RIPEMD160 hash
    pub fn ripemd160(data: ByteString) -> ByteString {
        crate::services::contract::Contract::call(
            CRYPTO_LIB_HASH,
            ByteString::from_literal("ripemd160"),
            crate::types::CallFlags::READ_ONLY,
            crate::types::Array::from_vec(vec![data.into_any()]),
        )
        .as_bytes()
        .unwrap_or_else(ByteString::empty)
    }

    /// Computes Keccak256 hash (used in Ethereum)
    pub fn keccak256(data: ByteString) -> ByteString {
        crate::services::contract::Contract::call(
            CRYPTO_LIB_HASH,
            ByteString::from_literal("keccak256"),
            crate::types::CallFlags::READ_ONLY,
            crate::types::Array::from_vec(vec![data.into_any()]),
        )
        .as_bytes()
        .unwrap_or_else(ByteString::empty)
    }

    /// Computes Murmur32 hash
    pub fn murmur32(data: ByteString, seed: u32) -> ByteString {
        crate::services::contract::Contract::call(
            CRYPTO_LIB_HASH,
            ByteString::from_literal("murmur32"),
            crate::types::CallFlags::READ_ONLY,
            crate::types::Array::from_vec(vec![
                data.into_any(),
                Int256::from(seed as i64).into_any(),
            ]),
        )
        .as_bytes()
        .unwrap_or_else(ByteString::empty)
    }

    /// Verifies signature with ECDSA
    pub fn verify_with_ecdsa(
        message: ByteString,
        pubkey: PublicKey,
        signature: ByteString,
        curve: EcdsaCurve,
    ) -> bool {
        crate::services::contract::Contract::call(
            CRYPTO_LIB_HASH,
            ByteString::from_literal("verifyWithECDsa"),
            crate::types::CallFlags::READ_ONLY,
            crate::types::Array::from_vec(vec![
                message.into_any(),
                pubkey.into_any(),
                signature.into_any(),
                Int256::from(curve as i64).into_any(),
            ]),
        )
        .as_bool()
        .unwrap_or(false)
    }

    /// BLS12-381 point addition
    pub fn bls12_381_add(x: ByteString, y: ByteString) -> ByteString {
        crate::services::contract::Contract::call(
            CRYPTO_LIB_HASH,
            ByteString::from_literal("bls12_381_add"),
            crate::types::CallFlags::READ_ONLY,
            crate::types::Array::from_vec(vec![x.into_any(), y.into_any()]),
        )
        .as_bytes()
        .unwrap_or_else(ByteString::empty)
    }

    /// BLS12-381 scalar multiplication
    pub fn bls12_381_mul(x: ByteString, mul: ByteString, neg: bool) -> ByteString {
        crate::services::contract::Contract::call(
            CRYPTO_LIB_HASH,
            ByteString::from_literal("bls12_381_mul"),
            crate::types::CallFlags::READ_ONLY,
            crate::types::Array::from_vec(vec![
                x.into_any(),
                mul.into_any(),
                neg.into_any(),
            ]),
        )
        .as_bytes()
        .unwrap_or_else(ByteString::empty)
    }

    /// BLS12-381 pairing
    pub fn bls12_381_pairing(x: ByteString, y: ByteString) -> ByteString {
        crate::services::contract::Contract::call(
            CRYPTO_LIB_HASH,
            ByteString::from_literal("bls12_381_pairing"),
            crate::types::CallFlags::READ_ONLY,
            crate::types::Array::from_vec(vec![x.into_any(), y.into_any()]),
        )
        .as_bytes()
        .unwrap_or_else(ByteString::empty)
    }
}

/// ECDSA curve types
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EcdsaCurve {
    /// secp256k1 curve (used by Bitcoin)
    Secp256k1 = 0,
    /// secp256r1 curve (used by Neo)
    Secp256r1 = 1,
}