// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::env::contract;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use crate::types::Any;
use crate::types::Array;
use crate::prelude::*;

/// CryptoLib native contract for Neo N3
/// 
/// This contract provides cryptographic utility functions for Neo N3 smart contracts
/// 
/// Contract Hash: 0x726cb6e0cd8628a1350a611384688911ab75f51b
#[allow(non_snake_case)]
pub struct CryptoLib;

/// Named curve types for cryptographic operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamedCurve {
    /// SECP256R1 curve (Default for Neo)
    Secp256r1 = 0,
    /// SECP256K1 curve (Used by Bitcoin and Ethereum)
    Secp256k1 = 1,
}

impl CryptoLib {
    /// Returns the contract hash for the CryptoLib native contract
    pub fn hash() -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            contract::crypto_lib_contract_hash()
        }
        #[cfg(target_arch = "wasm32")]
        {
            contract::native_crypto_lib_contract_hash()
        }
    }

    /// Calculates the SHA256 hash of the input data
    /// 
    /// # Arguments
    /// 
    /// * `data` - The data to hash
    /// 
    /// # Returns
    /// 
    /// The SHA256 hash as a byte array
    #[safe]
    pub fn sha256(data: &[u8]) -> Vec<u8> {
        let method = ByteString::from("sha256");
        let mut args = Array::<Any>::new();
        args.push(Any::from(data));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        match result.try_into::<Vec<u8>>() {
            Ok(hash) => hash,
            Err(_) => Vec::new(),
        }
    }

    /// Calculates the RIPEMD160 hash of the input data
    /// 
    /// # Arguments
    /// 
    /// * `data` - The data to hash
    /// 
    /// # Returns
    /// 
    /// The RIPEMD160 hash as a byte array
    #[safe]
    pub fn ripemd160(data: &[u8]) -> Vec<u8> {
        let method = ByteString::from("ripemd160");
        let mut args = Array::<Any>::new();
        args.push(Any::from(data));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        match result.try_into::<Vec<u8>>() {
            Ok(hash) => hash,
            Err(_) => Vec::new(),
        }
    }

    /// Verifies a signature using the specified elliptic curve
    /// 
    /// # Arguments
    /// 
    /// * `message` - The original message that was signed
    /// * `signature` - The signature to verify
    /// * `public_key` - The public key to use for verification
    /// * `curve` - The named curve to use (default is Secp256r1)
    /// 
    /// # Returns
    /// 
    /// True if the signature is valid, false otherwise
    #[safe]
    pub fn verify_with_ecdsasecp256r1(
        message: &[u8], 
        signature: &[u8], 
        public_key: &[u8],
        curve: NamedCurve,
    ) -> bool {
        let method = ByteString::from("verifyWithECDsaSecp256r1");
        let mut args = Array::<Any>::new();
        args.push(Any::from(message));
        args.push(Any::from(signature));
        args.push(Any::from(public_key));
        args.push(Any::from(curve as u8));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(false)
    }

    /// Verifies a signature using the Secp256r1 curve (Neo's default)
    /// 
    /// # Arguments
    /// 
    /// * `message` - The original message that was signed
    /// * `signature` - The signature to verify
    /// * `public_key` - The public key to use for verification
    /// 
    /// # Returns
    /// 
    /// True if the signature is valid, false otherwise
    #[safe]
    pub fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        Self::verify_with_ecdsasecp256r1(message, signature, public_key, NamedCurve::Secp256r1)
    }

    /// Gets the script hash from a public key
    /// 
    /// # Arguments
    /// 
    /// * `public_key` - The public key to derive the script hash from
    /// 
    /// # Returns
    /// 
    /// The script hash as a H160
    #[safe]
    pub fn get_script_hash_from_public_key(public_key: &[u8]) -> H160 {
        let method = ByteString::from("getScriptHashFromPublicKey");
        let mut args = Array::<Any>::new();
        args.push(Any::from(public_key));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or_else(|_| H160::zero())
    }

    /// Recovers the public key from a signature
    /// 
    /// # Arguments
    /// 
    /// * `message` - The original message that was signed
    /// * `signature` - The signature
    /// * `recovery_code` - The recovery code from the signature
    /// * `curve` - The named curve to use (default is Secp256r1)
    /// 
    /// # Returns
    /// 
    /// The recovered public key as a byte array
    #[safe]
    pub fn recover_public_key(
        message: &[u8], 
        signature: &[u8], 
        recovery_code: u8,
        curve: NamedCurve,
    ) -> Vec<u8> {
        let method = ByteString::from("recoverPublicKey");
        let mut args = Array::<Any>::new();
        args.push(Any::from(message));
        args.push(Any::from(signature));
        args.push(Any::from(recovery_code));
        args.push(Any::from(curve as u8));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        match result.try_into::<Vec<u8>>() {
            Ok(public_key) => public_key,
            Err(_) => Vec::new(),
        }
    }

    /// Gets the script hash from a standard account, given an address string
    /// 
    /// # Arguments
    /// 
    /// * `address` - The Neo3 address in ByteString format
    /// 
    /// # Returns
    /// 
    /// The script hash as a H160
    #[safe]
    pub fn address_to_script_hash(address: &ByteString) -> H160 {
        let method = ByteString::from("addressToScriptHash");
        let mut args = Array::<Any>::new();
        args.push(Any::from(address.clone()));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or_else(|_| H160::zero())
    }

    /// Converts a script hash to a standard Neo3 address
    /// 
    /// # Arguments
    /// 
    /// * `script_hash` - The script hash to convert
    /// 
    /// # Returns
    /// 
    /// The Neo3 address as a ByteString
    #[safe]
    pub fn script_hash_to_address(script_hash: &H160) -> ByteString {
        let method = ByteString::from("scriptHashToAddress");
        let mut args = Array::<Any>::new();
        args.push(Any::from(script_hash.clone()));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or_else(|_| ByteString::from(""))
    }
}
