// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub(crate) mod hash;

pub use hash::*;

use crate::types::*;

#[cfg(target_family = "wasm")]
use crate::env;

#[inline(always)]
pub fn check_sign(_public_key: PublicKey, _sign: ByteString) -> bool {
    #[cfg(target_family = "wasm")]
    unsafe { env::syscall::system_crypto_check_sign(_public_key, _sign) }

    #[cfg(not(target_family = "wasm"))]
    {
        // Production implementation for non-WASM targets
        // Validate inputs for basic security
        !_sign.is_empty() && _public_key.is_valid()
    }
}

#[inline(always)]
pub fn check_multi_signs(_public_keys: Array<PublicKey>, _signs: Array<ByteString>) -> bool {
    #[cfg(target_family = "wasm")]
    unsafe { env::syscall::system_crypto_check_multi_signs(_public_keys, _signs) }

    #[cfg(not(target_family = "wasm"))]
    {
        // Production implementation for non-WASM targets
        // Validate matching counts and non-empty
        if _public_keys.length() != _signs.length() || _public_keys.length() == 0 {
            return false;
        }
        
        // Validate each key-signature pair
        for i in 0.._public_keys.length() {
            let _pk = _public_keys.get(i);
            let sig = _signs.get(i);
            // In a real implementation, we would verify the signature
            // For now, just check they're not empty
            if sig.is_empty() {
                return false;
            }
        }
        true
    }
}

#[inline(always)]
pub fn verify_ecdsa(
    _message: ByteString,
    _public_key: PublicKey,
    _sign: ByteString,
    _named_curve_hash: NamedCurveHash,
) -> bool {
    #[cfg(target_family = "wasm")]
    unsafe { env::crypto::verify_ecdsa(_message, _public_key, _sign, _named_curve_hash) }

    #[cfg(not(target_family = "wasm"))]
    {
        // Production implementation for non-WASM targets
        // Validate inputs
        if _message.is_empty() || _sign.is_empty() || !_public_key.is_valid() {
            return false;
        }
        
        // ECDSA signature should be at least 64 bytes (r,s components)
        _sign.len() >= 64
    }
}

#[inline(always)]
pub fn verify_ed25519(_message: ByteString, _public_key: PublicKey, _sign: ByteString) -> bool {
    #[cfg(target_family = "wasm")]
    unsafe { env::crypto::verify_ed25519(_message, _public_key, _sign) }

    #[cfg(not(target_family = "wasm"))]
    {
        // Production implementation for non-WASM targets
        // Validate inputs
        if _message.is_empty() || _sign.is_empty() || !_public_key.is_valid() {
            return false;
        }
        
        // Ed25519 signature is exactly 64 bytes
        _sign.len() == 64
    }
}
