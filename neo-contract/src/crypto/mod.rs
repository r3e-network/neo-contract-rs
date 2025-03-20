// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::env::syscall_non_wasm::{
    system_crypto_bls_generate, system_crypto_bls_verify, system_crypto_check_multi_signs,
    system_crypto_check_multisig, system_crypto_check_sign, system_crypto_hash, system_crypto_to_address,
    system_crypto_to_script_hash, system_crypto_verify_with_ecdsa,
};
use crate::prelude::{Array, ByteString, H160, H256};

/// Named curve hash type for cryptographic functions
#[derive(Debug, Clone, Copy)]
pub enum NamedCurveHash {
    /// SHA256
    SHA256 = 0,
    /// RIPEMD160
    RIPEMD160 = 1,
    /// BLAKE2B
    BLAKE2B = 2,
    /// SHA3_256
    SHA3_256 = 3,
    /// KECCAK256 (Ethereum compatible)
    KECCAK256 = 4,
}

/// ECDSA curve types
#[derive(Debug, Clone, Copy)]
pub enum ECDsaCurve {
    /// secp256r1 (P-256)
    SECP256R1 = 0,
    /// secp256k1 (used by Bitcoin and Ethereum)
    SECP256K1 = 1,
}

/// Crypto methods for Neo smart contracts
pub struct Crypto;

impl Crypto {
    /// Hash a byte array using the specified algorithm
    pub fn hash(data: ByteString, hash_type: u32) -> ByteString {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            system_crypto_hash(data, hash_type)
        }

        #[cfg(target_arch = "wasm32")]
        unsafe {
            system_crypto_hash(data, hash_type)
        }
    }

    /// Compute SHA256 hash of the data
    pub fn sha256(data: &[u8]) -> H256 {
        let data_bs = ByteString::from_bytes(data);
        let result = Self::hash(data_bs, NamedCurveHash::SHA256 as u32);

        // Manually convert ByteString to bytes and then to H256
        let bytes = result.as_bytes();
        if bytes.len() == 32 {
            H256::from_slice(bytes)
        } else {
            H256::zero()
        }
    }

    /// Compute RIPEMD160 hash of the data
    pub fn ripemd160(data: &[u8]) -> [u8; 20] {
        let data_bs = ByteString::from_bytes(data);
        let result = Self::hash(data_bs, NamedCurveHash::RIPEMD160 as u32);

        // Manually convert ByteString to [u8; 20]
        let bytes = result.as_bytes();
        let mut array = [0u8; 20];
        if bytes.len() >= 20 {
            array.copy_from_slice(&bytes[..20]);
        }
        array
    }

    /// Compute BLAKE2B hash of the data
    pub fn blake2b(data: &[u8]) -> H256 {
        let data_bs = ByteString::from_bytes(data);
        let result = Self::hash(data_bs, NamedCurveHash::BLAKE2B as u32);

        // Manually convert ByteString to H256
        let bytes = result.as_bytes();
        if bytes.len() == 32 {
            H256::from_slice(bytes)
        } else {
            H256::zero()
        }
    }

    /// Compute SHA3_256 hash of the data
    pub fn sha3_256(data: &[u8]) -> H256 {
        let data_bs = ByteString::from_bytes(data);
        let result = Self::hash(data_bs, NamedCurveHash::SHA3_256 as u32);

        // Manually convert ByteString to H256
        let bytes = result.as_bytes();
        if bytes.len() == 32 {
            H256::from_slice(bytes)
        } else {
            H256::zero()
        }
    }

    /// Compute Keccak256 hash of the data (Ethereum compatible)
    pub fn keccak256(data: &[u8]) -> H256 {
        let data_bs = ByteString::from_bytes(data);
        let result = Self::hash(data_bs, NamedCurveHash::KECCAK256 as u32);

        // Manually convert ByteString to H256
        let bytes = result.as_bytes();
        if bytes.len() == 32 {
            H256::from_slice(bytes)
        } else {
            H256::zero()
        }
    }

    /// Verify an ECDSA signature
    pub fn verify_ecdsa(message: &[u8], signature: &[u8], public_key: &[u8], curve: u32) -> bool {
        let message_bs = ByteString::from_bytes(message);
        let signature_bs = ByteString::from_bytes(signature);
        let public_key_bs = ByteString::from_bytes(public_key);

        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            system_crypto_verify_with_ecdsa(message_bs, signature_bs, public_key_bs, curve)
        }

        #[cfg(target_arch = "wasm32")]
        unsafe {
            system_crypto_verify_with_ecdsa(message_bs, signature_bs, public_key_bs, curve)
        }
    }

    /// Check a signature with secp256r1
    pub fn check_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        Self::verify_ecdsa(message, signature, public_key, ECDsaCurve::SECP256R1 as u32)
    }

    /// Check multiple signatures
    pub fn check_multisig(message: &[u8], signatures: &[&[u8]], public_keys: &[&[u8]]) -> bool {
        // Convert message to ByteString
        let message_bs = ByteString::from_bytes(message);

        // Convert signatures to Array
        let mut sigs_array = Array::new();
        for sig in signatures {
            sigs_array.push(ByteString::from_bytes(sig));
        }

        // Convert public keys to Array
        let mut keys_array = Array::new();
        for key in public_keys {
            keys_array.push(ByteString::from_bytes(key));
        }

        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            system_crypto_check_multisig(message_bs, sigs_array, keys_array)
        }

        #[cfg(target_arch = "wasm32")]
        unsafe {
            system_crypto_check_multisig(message_bs, sigs_array, keys_array)
        }
    }

    /// Convert a script hash to a Neo address (Base58 check encoding)
    pub fn to_address(script_hash: &H160) -> ByteString {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            system_crypto_to_address(script_hash.clone())
        }

        #[cfg(target_arch = "wasm32")]
        unsafe {
            system_crypto_to_address(script_hash.clone())
        }
    }

    /// Convert a Neo address to a script hash
    pub fn to_script_hash(address: &ByteString) -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            system_crypto_to_script_hash(address.clone())
        }

        #[cfg(target_arch = "wasm32")]
        unsafe {
            system_crypto_to_script_hash(address.clone())
        }
    }

    /// Generate a BLS signature for a message
    pub fn bls_generate(message: &[u8], private_key: &[u8]) -> ByteString {
        let message_bs = ByteString::from_bytes(message);
        let private_key_bs = ByteString::from_bytes(private_key);

        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            system_crypto_bls_generate(message_bs, private_key_bs)
        }

        #[cfg(target_arch = "wasm32")]
        unsafe {
            system_crypto_bls_generate(message_bs, private_key_bs)
        }
    }

    /// Verify a BLS signature
    pub fn bls_verify(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        let message_bs = ByteString::from_bytes(message);
        let signature_bs = ByteString::from_bytes(signature);
        let public_key_bs = ByteString::from_bytes(public_key);

        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            system_crypto_bls_verify(message_bs, signature_bs, public_key_bs)
        }

        #[cfg(target_arch = "wasm32")]
        unsafe {
            system_crypto_bls_verify(message_bs, signature_bs, public_key_bs)
        }
    }
}

/// Backward compatibility functions

/// Check a signature
pub fn check_sign(public_key: ByteString, sign: ByteString) -> bool {
    unsafe { system_crypto_check_sign(public_key, sign) }
}

/// Check multiple signatures
pub fn check_multi_signs(_public_keys: Array, _signs: Array) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    unsafe {
        // Convert public keys to ByteString array
        let public_keys_converted = Array::new();
        // TODO: Implement proper conversion between public_keys and public_keys_converted

        // Convert signatures to ByteString array
        let signs_converted = Array::new();
        // TODO: Implement proper conversion between signs and signs_converted

        system_crypto_check_multi_signs(public_keys_converted, signs_converted)
    }

    #[cfg(target_arch = "wasm32")]
    unsafe {
        // Convert public keys to ByteString array
        let mut public_keys_converted = Array::new();
        // TODO: Implement proper conversion

        // Convert signatures to ByteString array
        let mut signs_converted = Array::new();
        // TODO: Implement proper conversion

        system_crypto_check_multi_signs(public_keys_converted, signs_converted)
    }
}

/// Hash data using the specified algorithm
pub fn hash(data: ByteString, hash_type: NamedCurveHash) -> ByteString { Crypto::hash(data, hash_type as u32) }

/// Compute SHA256 hash of the data
pub fn sha256(data: &[u8]) -> H256 { Crypto::sha256(data) }

/// Compute RIPEMD160 hash of the data
pub fn ripemd160(data: &[u8]) -> [u8; 20] { Crypto::ripemd160(data) }

/// Verify an ECDSA signature
pub fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    Crypto::verify_ecdsa(message, signature, public_key, 0) // 0 is SECP256R1
}
