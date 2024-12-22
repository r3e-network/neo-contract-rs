// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub(crate) mod hash;

pub use hash::*;

use crate::{env, types::*};

#[inline(always)]
pub fn check_sign(public_key: PublicKey, sign: ByteString) -> bool {
    unsafe { env::syscall::system_crypto_check_sign(public_key, sign) }
}

#[inline(always)]
pub fn check_multi_signs(public_keys: Array<PublicKey>, signs: Array<ByteString>) -> bool {
    unsafe { env::syscall::system_crypto_check_multi_signs(public_keys, signs) }
}

#[inline(always)]
pub fn verify_ecdsa(
    message: ByteString,
    public_key: PublicKey,
    sign: ByteString,
    named_curve_hash: NamedCurveHash,
) -> bool {
    unsafe { env::crypto::verify_ecdsa(message, public_key, sign, named_curve_hash) }
}

#[inline(always)]
pub fn verify_ed25519(message: ByteString, public_key: PublicKey, sign: ByteString) -> bool {
    unsafe { env::crypto::verify_ed25519(message, public_key, sign) }
}
