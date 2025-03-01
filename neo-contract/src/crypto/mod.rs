// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::types::builtin::array::Array;
use crate::types::builtin::string::ByteString;

/// Check a signature
pub fn check_sign(public_key: ByteString, sign: ByteString) -> bool {
    unsafe { crate::env::syscall_non_wasm::system_crypto_check_sign(public_key, sign) }
}

/// Check multiple signatures
pub fn check_multi_signs(public_keys: Array<ByteString>, signs: Array<ByteString>) -> bool {
    unsafe { crate::env::syscall_non_wasm::system_crypto_check_multi_signs(public_keys, signs) }
}

/// Named curve hash
pub enum NamedCurveHash {
    /// SHA256
    SHA256 = 0,
    /// RIPEMD160
    RIPEMD160 = 1,
}

/// Hash data using the specified algorithm
pub fn hash(data: ByteString, hash_type: NamedCurveHash) -> ByteString {
    // In a real implementation, this would call the crypto hash syscall
    ByteString::new()
}
