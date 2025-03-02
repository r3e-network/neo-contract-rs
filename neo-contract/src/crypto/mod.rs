// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::types::builtin::array::Array;
use crate::types::builtin::string::ByteString;
use crate::types::builtin::h256::H256;
use crate::builtin::H160;

/// Crypto methods for Neo smart contracts
pub struct Crypto;

impl Crypto {
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
        /// KECCAK256 (Ethereum compatible)
        KECCAK256 = 4,
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
        let result = Self::hash(data_bs, Self::NamedCurveHash::SHA256);
        // Convert ByteString to H256
        let bytes: Vec<u8> = result.into();
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        H256::new(array)
    }
    
    /// Compute RIPEMD160 hash of the data
    pub fn ripemd160(data: &[u8]) -> [u8; 20] {
        let data_bs = ByteString::from_bytes(data);
        let result = Self::hash(data_bs, Self::NamedCurveHash::RIPEMD160);
        // Convert ByteString to [u8; 20]
        let bytes: Vec<u8> = result.into();
        let mut array = [0u8; 20];
        array.copy_from_slice(&bytes);
        array
    }
    
    /// Compute BLAKE2B hash of the data
    pub fn blake2b(data: &[u8]) -> H256 {
        let data_bs = ByteString::from_bytes(data);
        let result = Self::hash(data_bs, Self::NamedCurveHash::BLAKE2B);
        // Convert ByteString to H256
        let bytes: Vec<u8> = result.into();
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        H256::new(array)
    }
    
    /// Compute SHA3_256 hash of the data
    pub fn sha3_256(data: &[u8]) -> H256 {
        let data_bs = ByteString::from_bytes(data);
        let result = Self::hash(data_bs, Self::NamedCurveHash::SHA3_256);
        // Convert ByteString to H256
        let bytes: Vec<u8> = result.into();
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        H256::new(array)
    }
    
    /// Compute Keccak256 hash of the data (Ethereum compatible)
    pub fn keccak256(data: &[u8]) -> H256 {
        let data_bs = ByteString::from_bytes(data);
        let result = Self::hash(data_bs, Self::NamedCurveHash::KECCAK256);
        // Convert ByteString to H256
        let bytes: Vec<u8> = result.into();
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        H256::new(array)
    }
    
    /// ECDSA curve types
    pub enum ECDsaCurve {
        /// secp256r1 (P-256)
        SECP256R1 = 0,
        /// secp256k1 (used by Bitcoin and Ethereum)
        SECP256K1 = 1,
    }
    
    /// Verify an ECDSA signature
    pub fn verify_ecdsa(message: &[u8], signature: &[u8], public_key: &[u8], curve: ECDsaCurve) -> bool {
        let message_bs = ByteString::from_bytes(message);
        let signature_bs = ByteString::from_bytes(signature);
        let public_key_bs = ByteString::from_bytes(public_key);
        
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_crypto_verify_with_ecdsa(
                message_bs, signature_bs, public_key_bs, curve as u32
            )
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_crypto_verify_with_ecdsa(
                message_bs, signature_bs, public_key_bs, curve as u32
            )
        }
    }
    
    /// Check a signature with secp256r1
    pub fn check_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        Self::verify_ecdsa(message, signature, public_key, Self::ECDsaCurve::SECP256R1)
    }
    
    /// Check multiple signatures
    pub fn check_multisig(message: &[u8], signatures: &[&[u8]], public_keys: &[&[u8]]) -> bool {
        // Convert message to ByteString
        let message_bs = ByteString::from_bytes(message);
        
        // Convert signatures to Array<ByteString>
        let mut sigs_array = Array::<ByteString>::new();
        for sig in signatures {
            sigs_array.push(ByteString::from_bytes(sig));
        }
        
        // Convert public keys to Array<ByteString>
        let mut keys_array = Array::<ByteString>::new();
        for key in public_keys {
            keys_array.push(ByteString::from_bytes(key));
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_crypto_check_multisig(message_bs, sigs_array, keys_array)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_crypto_check_multisig(message_bs, sigs_array, keys_array)
        }
    }
    
    /// Convert a script hash to a Neo address (Base58 check encoding)
    pub fn to_address(script_hash: &H160) -> ByteString {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_crypto_to_address(script_hash.clone())
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_crypto_to_address(script_hash.clone())
        }
    }
    
    /// Convert a Neo address to a script hash
    pub fn to_script_hash(address: &ByteString) -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_crypto_to_script_hash(address.clone())
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_crypto_to_script_hash(address.clone())
        }
    }
    
    /// Generate a BLS signature for a message
    pub fn bls_generate(message: &[u8], private_key: &[u8]) -> ByteString {
        let message_bs = ByteString::from_bytes(message);
        let private_key_bs = ByteString::from_bytes(private_key);
        
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_crypto_bls_generate(message_bs, private_key_bs)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_crypto_bls_generate(message_bs, private_key_bs)
        }
    }
    
    /// Verify a BLS signature
    pub fn bls_verify(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        let message_bs = ByteString::from_bytes(message);
        let signature_bs = ByteString::from_bytes(signature);
        let public_key_bs = ByteString::from_bytes(public_key);
        
        #[cfg(not(target_arch = "wasm32"))]
        unsafe { 
            crate::env::syscall_non_wasm::system_crypto_bls_verify(message_bs, signature_bs, public_key_bs)
        }
        
        #[cfg(target_arch = "wasm32")]
        unsafe { 
            crate::env::syscall::system_crypto_bls_verify(message_bs, signature_bs, public_key_bs)
        }
    }
}

/// Backward compatibility functions

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
    Crypto::hash(data, match hash_type {
        NamedCurveHash::SHA256 => Crypto::NamedCurveHash::SHA256,
        NamedCurveHash::RIPEMD160 => Crypto::NamedCurveHash::RIPEMD160,
        NamedCurveHash::BLAKE2B => Crypto::NamedCurveHash::BLAKE2B,
        NamedCurveHash::SHA3_256 => Crypto::NamedCurveHash::SHA3_256,
    })
}

/// Compute SHA256 hash of the data
pub fn sha256(data: &[u8]) -> H256 {
    Crypto::sha256(data)
}

/// Compute RIPEMD160 hash of the data
pub fn ripemd160(data: &[u8]) -> [u8; 20] {
    Crypto::ripemd160(data)
}

/// Verify an ECDSA signature
pub fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    Crypto::verify_ecdsa(message, signature, public_key, Crypto::ECDsaCurve::SECP256R1)
}
