// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use core::fmt;
use crate::utils::hex;

/// Key type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    /// ECDSA key
    ECDSA = 0x12,
    /// NEO ECDSA key
    NeoEcdsa = 0x21,
}

/// Named curve
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamedCurve {
    /// secp256k1
    Secp256k1 = 0,
    /// secp256r1
    Secp256r1 = 1,
}

/// Named curve hash
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamedCurveHash {
    /// SHA256
    SHA256 = 0,
    /// RIPEMD160
    RIPEMD160 = 1,
}

/// PublicKey represents a public key
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey(pub [u8; 33]);

impl PublicKey {
    /// Create a new public key
    pub fn new(bytes: [u8; 33]) -> Self {
        PublicKey(bytes)
    }

    /// Get the bytes of the public key
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Decode a hex string into a public key
    pub fn hex_decode(hex: &str) -> Option<Self> {
        let hex = if hex.starts_with("0x") {
            &hex[2..]
        } else {
            hex
        };

        if hex.len() != 66 {
            return None;
        }

        match hex::decode(hex) {
            Ok(bytes) => {
                let mut result = [0u8; 33];
                result.copy_from_slice(&bytes);
                Some(PublicKey(result))
            },
            Err(_) => None
        }
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(&self.0))
    }
}
