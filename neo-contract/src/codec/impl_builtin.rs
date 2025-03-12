// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Codec implementations for builtin types

use crate::error::{Error, ErrorCode, Result};
use crate::storage::item::Codec;
use crate::types::builtin::h160::H160;
use crate::types::builtin::h256::H256;
use crate::types::builtin::int256::Int256;
use crate::types::builtin::string::ByteString;
use crate::types::builtin::array::Array;
use crate::types::builtin::any::Any;
use alloc::vec::Vec;

/// Codec implementation for ByteString
impl Codec for ByteString {
    fn encode(&self) -> Vec<u8> {
        // ByteString already contains Vec<u8>, so we can just clone it
        self.0.clone()
    }

    fn decode(bytes: &[u8]) -> Result<Self> {
        // Create a ByteString from the bytes
        Ok(ByteString::from(bytes))
    }
}

/// Codec implementation for H160
impl Codec for H160 {
    fn encode(&self) -> Vec<u8> {
        // H160 contains a 20-byte array, convert it to Vec<u8>
        self.0.to_vec()
    }

    fn decode(bytes: &[u8]) -> Result<Self> {
        // Ensure the bytes are the correct length for H160
        if bytes.len() != 20 {
            return Err(Error::with_message(ErrorCode::DecodingError, "Invalid H160 length"));
        }
        
        // Copy bytes into the H160 array
        let mut arr = [0u8; 20];
        arr.copy_from_slice(bytes);
        
        Ok(H160(arr))
    }
}

/// Codec implementation for H256
impl Codec for H256 {
    fn encode(&self) -> Vec<u8> {
        // H256 contains a 32-byte array, convert it to Vec<u8>
        self.0.to_vec()
    }

    fn decode(bytes: &[u8]) -> Result<Self> {
        // Ensure the bytes are the correct length for H256
        if bytes.len() != 32 {
            return Err(Error::with_message(ErrorCode::DecodingError, "Invalid H256 length"));
        }
        
        // Copy bytes into the H256 array
        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);
        
        Ok(H256(arr))
    }
}

/// Codec implementation for Int256
impl Codec for Int256 {
    fn encode(&self) -> Vec<u8> {
        // Int256 contains a 32-byte array, convert it to Vec<u8>
        self.0.to_vec()
    }

    fn decode(bytes: &[u8]) -> Result<Self> {
        // Ensure the bytes are the correct length for Int256
        if bytes.len() != 32 {
            return Err(Error::with_message(ErrorCode::DecodingError, "Invalid Int256 length"));
        }
        
        // Copy bytes into the Int256 array
        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);
        
        Ok(Int256(arr))
    }
}

/// Codec implementation for Array
impl Codec for Array {
    fn encode(&self) -> Vec<u8> {
        // For Array, we need to encode each Any element
        // This is a simplified version - a real implementation would need 
        // to handle all Any types properly
        let mut result = Vec::new();
        
        // Add the number of elements as a prefix (4 bytes)
        let len = self.0.len() as u32;
        result.extend_from_slice(&len.to_le_bytes());
        
        // Serialize each element (with type information)
        for item in &self.0 {
            match item {
                Any::Integer(int) => {
                    // Type byte: 1 for Integer
                    result.push(1);
                    // Append the Int256 bytes
                    result.extend_from_slice(&int.0);
                }
                Any::Boolean(b) => {
                    // Type byte: 2 for Boolean
                    result.push(2);
                    // Append the boolean as a byte
                    result.push(*b as u8);
                }
                Any::ByteString(bs) => {
                    // Type byte: 3 for ByteString
                    result.push(3);
                    // Append the length of the string (4 bytes)
                    let bs_len = bs.len() as u32;
                    result.extend_from_slice(&bs_len.to_le_bytes());
                    // Append the ByteString bytes
                    result.extend_from_slice(bs.as_bytes());
                }
                Any::Array(arr) => {
                    // Type byte: 4 for Array
                    result.push(4);
                    // Recursively encode the nested array
                    // This is a simplified approach and might not handle complex nested structures
                    let encoded = Array::new_with_data(arr.clone()).encode();
                    result.extend_from_slice(&encoded);
                }
                Any::Map(map) => {
                    // Type byte: 5 for Map
                    result.push(5);
                    // Length of the map
                    let map_len = map.len() as u32;
                    result.extend_from_slice(&map_len.to_le_bytes());
                    // For each key-value pair, encode key and value
                    // This is a simplified approach
                    for (k, v) in map {
                        // Recursively encode key and value (as Any)
                        let key_encoded = match k {
                            Any::ByteString(bs) => bs.encode(),
                            _ => Vec::new(), // simplified - would need full Any encoding
                        };
                        let val_encoded = match v {
                            Any::Integer(i) => i.encode(),
                            _ => Vec::new(), // simplified - would need full Any encoding
                        };
                        
                        // Add lengths and data
                        let key_len = key_encoded.len() as u32;
                        result.extend_from_slice(&key_len.to_le_bytes());
                        result.extend_from_slice(&key_encoded);
                        
                        let val_len = val_encoded.len() as u32;
                        result.extend_from_slice(&val_len.to_le_bytes());
                        result.extend_from_slice(&val_encoded);
                    }
                }
                Any::Null => {
                    // Type byte: 0 for Null
                    result.push(0);
                }
            }
        }
        
        result
    }

    fn decode(bytes: &[u8]) -> Result<Self> {
        // This is a simplified decoder - a real implementation would be more robust
        if bytes.len() < 4 {
            return Err(Error::with_message(ErrorCode::DecodingError, "Invalid Array encoding"));
        }
        
        // Read the number of elements
        let mut len_bytes = [0u8; 4];
        len_bytes.copy_from_slice(&bytes[0..4]);
        let len = u32::from_le_bytes(len_bytes) as usize;
        
        // Start with an empty array
        let mut array = Array::new();
        
        // Current position in the byte array
        let mut pos = 4;
        
        // Decode each element
        for _ in 0..len {
            if pos >= bytes.len() {
                return Err(Error::with_message(ErrorCode::DecodingError, "Unexpected end of data"));
            }
            
            // Read the type byte
            let type_byte = bytes[pos];
            pos += 1;
            
            match type_byte {
                0 => {
                    // Null
                    array.push(Any::null());
                }
                1 => {
                    // Integer
                    if pos + 32 > bytes.len() {
                        return Err(Error::with_message(ErrorCode::DecodingError, "Invalid Int256 data"));
                    }
                    let mut int_bytes = [0u8; 32];
                    int_bytes.copy_from_slice(&bytes[pos..pos+32]);
                    pos += 32;
                    array.push(Any::Integer(Int256(int_bytes)));
                }
                2 => {
                    // Boolean
                    if pos >= bytes.len() {
                        return Err(Error::with_message(ErrorCode::DecodingError, "Invalid Boolean data"));
                    }
                    let b = bytes[pos] != 0;
                    pos += 1;
                    array.push(Any::Boolean(b));
                }
                3 => {
                    // ByteString
                    if pos + 4 > bytes.len() {
                        return Err(Error::with_message(ErrorCode::DecodingError, "Invalid ByteString length"));
                    }
                    let mut len_bytes = [0u8; 4];
                    len_bytes.copy_from_slice(&bytes[pos..pos+4]);
                    pos += 4;
                    let bs_len = u32::from_le_bytes(len_bytes) as usize;
                    
                    if pos + bs_len > bytes.len() {
                        return Err(Error::with_message(ErrorCode::DecodingError, "Invalid ByteString data"));
                    }
                    let bs_bytes = &bytes[pos..pos+bs_len];
                    pos += bs_len;
                    array.push(Any::ByteString(ByteString::from(bs_bytes)));
                }
                // Additional cases for Array and Map would be more complex
                // and are omitted for this simplified example
                _ => {
                    return Err(Error::with_message(ErrorCode::DecodingError, "Unsupported type"));
                }
            }
        }
        
        Ok(array)
    }
}

// We'd also need implementations for Any, but that's more complex
// as it involves variant handling, so it's omitted here.

// Extension methods to make Array serialization easier
impl Array {
    /// Create a new Array with initial data
    pub fn new_with_data(data: alloc::vec::Vec<Any>) -> Self {
        // Create an empty array and then populate it using public methods
        let mut array = Self::new();
        for item in data {
            array.push(item);
        }
        array
    }
    
    /// Serialize an Array to bytes (for storage)
    pub fn serialize(&self) -> alloc::vec::Vec<u8> {
        self.encode()
    }
    
    /// Deserialize bytes to an Array (from storage)
    pub fn deserialize(bytes: &[u8]) -> Result<Self> {
        Self::decode(bytes)
    }
} 