// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

/// Utility functions for Neo smart contracts
pub mod hex {
    use crate::types::*;

    /// Encodes bytes to a hexadecimal string
    pub fn encode(bytes: &[u8]) -> ByteString {
        let mut result = String::with_capacity(bytes.len() * 2);
        for &byte in bytes {
            result.push_str(&format!("{:02x}", byte));
        }
        ByteString::new(result)
    }

    /// Decodes a hexadecimal string to bytes
    pub fn decode(hex: &str) -> Option<Vec<u8>> {
        // Check if the string has a valid length
        if hex.len() % 2 != 0 {
            return None;
        }

        let mut result = Vec::with_capacity(hex.len() / 2);
        let mut chars = hex.chars();

        while let (Some(a), Some(b)) = (chars.next(), chars.next()) {
            let byte = match (a.to_digit(16), b.to_digit(16)) {
                (Some(high), Some(low)) => (high << 4 | low) as u8,
                _ => return None,
            };
            result.push(byte);
        }

        Some(result)
    }
}

/// Utility functions for working with addresses
pub mod address {
    use crate::types::*;

    /// Converts a script hash (H160) to an address string
    pub fn script_hash_to_address(script_hash: H160) -> ByteString {
        #[cfg(target_family = "wasm")]
        unsafe { crate::env::extension::h160_to_address(script_hash) }

        #[cfg(not(target_family = "wasm"))]
        {
            // Convert bytes to hex string
            let hex_str = hex::encode(script_hash.as_bytes());
            // Create address with "N" prefix
            let address = format!("N{}", hex_str);
            ByteString::new(address)
        }
    }

    /// Converts an address string to a script hash (H160)
    pub fn address_to_script_hash(address: ByteString) -> Option<H160> {
        #[cfg(target_family = "wasm")]
        unsafe { Some(crate::env::extension::address_to_h160(address)) }

        #[cfg(not(target_family = "wasm"))]
        {
            // Convert ByteString to String safely
            let bytes = address.as_bytes();
            let address_str = String::from_utf8_lossy(bytes);
            if !address_str.starts_with("N") {
                return None;
            }
            
            let hex_str = &address_str[1..];
            match super::hex::decode(hex_str) {
                Some(bytes) => {
                    if bytes.len() != 20 {
                        return None;
                    }
                    
                    let mut buf = [0u8; 20];
                    buf.copy_from_slice(&bytes);
                    Some(H160::from_bytes(buf))
                },
                None => None,
            }
        }
    }
}

/// Utility functions for serialization
pub mod serialization {
    use crate::types::*;

    /// Serializes a value to bytes
    pub fn serialize<T: Into<Any>>(_value: T) -> Vec<u8> {
        #[cfg(target_family = "wasm")]
        unsafe { crate::env::extension::serialize(_value.into()) }

        #[cfg(not(target_family = "wasm"))]
        Vec::new() // Placeholder for non-WASM implementation
    }

    /// Deserializes bytes to a value
    pub fn deserialize<T: crate::utils::FromAny>(_bytes: &[u8]) -> Option<T> {
        #[cfg(target_family = "wasm")]
        unsafe { 
            let any = crate::env::extension::deserialize(_bytes);
            T::from_any(any)
        }

        #[cfg(not(target_family = "wasm"))]
        None // Placeholder for non-WASM implementation
    }
}

/// Trait for converting from Any to a concrete type
pub trait FromAny {
    fn from_any(any: Any) -> Option<Self> where Self: Sized;
}

// Simplified implementations for non-WASM targets
#[cfg(not(target_family = "wasm"))]
impl FromAny for Int256 {
    fn from_any(_any: Any) -> Option<Self> {
        Some(Int256::zero())
    }
}

#[cfg(not(target_family = "wasm"))]
impl FromAny for ByteString {
    fn from_any(_any: Any) -> Option<Self> {
        Some(ByteString::new(String::new()))
    }
}

#[cfg(not(target_family = "wasm"))]
impl FromAny for H160 {
    fn from_any(_any: Any) -> Option<Self> {
        Some(H160::default())
    }
}

#[cfg(not(target_family = "wasm"))]
impl FromAny for H256 {
    fn from_any(_any: Any) -> Option<Self> {
        Some(H256::default())
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T: FromAny + Clone> FromAny for Array<T> {
    fn from_any(_any: Any) -> Option<Self> {
        Some(Array::new())
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K: FromAny + crate::types::builtin::Primitive + std::hash::Hash, V: FromAny> FromAny for Map<K, V> {
    fn from_any(_any: Any) -> Option<Self> {
        Some(Map::new())
    }
}

// WASM implementations
#[cfg(target_family = "wasm")]
impl FromAny for Int256 {
    fn from_any(any: Any) -> Option<Self> {
        any.as_int256()
    }
}

#[cfg(target_family = "wasm")]
impl FromAny for ByteString {
    fn from_any(any: Any) -> Option<Self> {
        any.as_byte_string()
    }
}

#[cfg(target_family = "wasm")]
impl FromAny for H160 {
    fn from_any(any: Any) -> Option<Self> {
        any.as_h160()
    }
}

#[cfg(target_family = "wasm")]
impl FromAny for H256 {
    fn from_any(any: Any) -> Option<Self> {
        any.as_h256()
    }
}

#[cfg(target_family = "wasm")]
impl<T: FromAny + Clone> FromAny for Array<T> {
    fn from_any(any: Any) -> Option<Self> {
        any.as_array().and_then(|arr| {
            let mut result = Array::new();
            for item in arr.iter() {
                match T::from_any(item) {
                    Some(value) => result.push(value),
                    None => return None,
                }
            }
            Some(result)
        })
    }
}

#[cfg(target_family = "wasm")]
impl<K: FromAny + crate::types::builtin::Primitive + std::hash::Hash, V: FromAny> FromAny for Map<K, V> {
    fn from_any(any: Any) -> Option<Self> {
        any.as_map().and_then(|map| {
            let mut result = Map::new();
            for (k, v) in map.iter() {
                match (K::from_any(k), V::from_any(v)) {
                    (Some(key), Some(value)) => {
                        result.insert(key, value);
                    },
                    _ => return None,
                }
            }
            Some(result)
        })
    }
}
