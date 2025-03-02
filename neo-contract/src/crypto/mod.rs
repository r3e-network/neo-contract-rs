// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::types::builtin::array::Array;
use crate::types::builtin::string::ByteString;
use crate::types::builtin::h256::H256;

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
    /// BLAKE2B
    BLAKE2B = 2,
    /// SHA3_256
    SHA3_256 = 3,
}

/// Hash data using the specified algorithm
pub fn hash(data: ByteString, hash_type: NamedCurveHash) -> ByteString {
    #[cfg(not(target_arch = "wasm32"))]
    unsafe { 
        crate::env::syscall_non_wasm::system_crypto_hash(data, hash_type as u32)
    }
    
    #[cfg(target_arch = "wasm32")]
    unsafe { 
        crate::env::syscall::system_crypto_hash(data, hash_type as u32)
    }
}

/// Compute SHA256 hash of the data
pub fn sha256(data: &[u8]) -> H256 {
    let data_bs = ByteString::from_bytes(data);
    let result = hash(data_bs, NamedCurveHash::SHA256);
    // Convert ByteString to H256
    let bytes: Vec<u8> = result.into();
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes);
    H256::new(array)
}

/// Compute RIPEMD160 hash of the data
pub fn ripemd160(data: &[u8]) -> [u8; 20] {
    let data_bs = ByteString::from_bytes(data);
    let result = hash(data_bs, NamedCurveHash::RIPEMD160);
    // Convert ByteString to [u8; 20]
    let bytes: Vec<u8> = result.into();
    let mut array = [0u8; 20];
    array.copy_from_slice(&bytes);
    array
}

/// Verify an ECDSA signature
pub fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    let message_bs = ByteString::from_bytes(message);
    let signature_bs = ByteString::from_bytes(signature);
    let public_key_bs = ByteString::from_bytes(public_key);
    
    #[cfg(not(target_arch = "wasm32"))]
    unsafe { 
        crate::env::syscall_non_wasm::system_crypto_verify_with_ecdsa(
            message_bs, signature_bs, public_key_bs, 0 // 0 for secp256r1 curve
        )
    }
    
    #[cfg(target_arch = "wasm32")]
    unsafe { 
        crate::env::syscall::system_crypto_verify_with_ecdsa(
            message_bs, signature_bs, public_key_bs, 0 // 0 for secp256r1 curve
        )
    }
}
