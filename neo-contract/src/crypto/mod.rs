// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub(crate) mod hash;

pub use hash::*;

use crate::types::*;

#[inline(always)]
pub fn check_sign(public_key: PublicKey, sign: ByteString) -> bool {
    #[cfg(target_family = "wasm")]
    unsafe { env::syscall::system_crypto_check_sign(public_key, sign) }

    #[cfg(not(target_family = "wasm"))]
    unsafe { crate::env::syscall_non_wasm::system_crypto_check_sign(public_key, sign) }
}

#[inline(always)]
pub fn check_multi_signs(public_keys: Array<PublicKey>, signs: Array<ByteString>) -> bool {
    #[cfg(target_family = "wasm")]
    unsafe { env::syscall::system_crypto_check_multi_signs(public_keys, signs) }

    #[cfg(not(target_family = "wasm"))]
    unsafe { crate::env::syscall_non_wasm::system_crypto_check_multi_signs(public_keys, signs) }
}

#[inline(always)]
pub fn verify_ecdsa(
    _message: ByteString,
    _public_key: PublicKey,
    _sign: ByteString,
    _named_curve_hash: NamedCurveHash,
) -> bool {
    #[cfg(target_family = "wasm")]
    unsafe { env::crypto::verify_ecdsa(message, public_key, sign, named_curve_hash) }

    #[cfg(not(target_family = "wasm"))]
    false // Placeholder for non-WASM implementation
}

#[inline(always)]
pub fn verify_ed25519(_message: ByteString, _public_key: PublicKey, _sign: ByteString) -> bool {
    #[cfg(target_family = "wasm")]
    unsafe { env::crypto::verify_ed25519(message, public_key, sign) }

    #[cfg(not(target_family = "wasm"))]
    false // Placeholder for non-WASM implementation
}
