// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::env::contract;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use crate::types::Any;
use crate::types::Array;
use crate::prelude::*;

/// StdLib native contract for Neo N3
/// 
/// This contract provides standard library functions for Neo N3 smart contracts,
/// such as serialization, string operations, and data manipulation.
/// 
/// Contract Hash: 0xacce6fd80d44e1796aa0c2c625e9e4e0ce39efc0
#[allow(non_snake_case)]
pub struct StdLib;

impl StdLib {
    /// Returns the contract hash for the StdLib native contract
    pub fn hash() -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            contract::std_lib_contract_hash()
        }
        #[cfg(target_arch = "wasm32")]
        {
            contract::native_std_lib_contract_hash()
        }
    }

    /// Serializes an object to a byte array
    /// 
    /// # Arguments
    /// 
    /// * `item` - The object to serialize
    /// 
    /// # Returns
    /// 
    /// The serialized object as a byte array
    #[safe]
    pub fn serialize(item: &Any) -> Vec<u8> {
        let method = ByteString::from("serialize");
        let mut args = Array::<Any>::new();
        args.push(item.clone());
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        match result.try_into::<Vec<u8>>() {
            Ok(data) => data,
            Err(_) => Vec::new(),
        }
    }

    /// Deserializes a byte array to an object
    /// 
    /// # Arguments
    /// 
    /// * `data` - The byte array to deserialize
    /// 
    /// # Returns
    /// 
    /// The deserialized object as Any
    #[safe]
    pub fn deserialize(data: &[u8]) -> Any {
        let method = ByteString::from("deserialize");
        let mut args = Array::<Any>::new();
        args.push(Any::from(data));
        
        Runtime::call_contract(&Self::hash(), &method, &args)
    }

    /// Converts a string to bytes
    /// 
    /// # Arguments
    /// 
    /// * `str` - The string to convert
    /// 
    /// # Returns
    /// 
    /// The string as a byte array
    #[safe]
    pub fn string_to_bytes(str: &ByteString) -> Vec<u8> {
        let method = ByteString::from("atoi");
        let mut args = Array::<Any>::new();
        args.push(Any::from(str.clone()));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        match result.try_into::<Vec<u8>>() {
            Ok(data) => data,
            Err(_) => Vec::new(),
        }
    }

    /// Converts a byte array to a string
    /// 
    /// # Arguments
    /// 
    /// * `bytes` - The byte array to convert
    /// 
    /// # Returns
    /// 
    /// The byte array as a string
    #[safe]
    pub fn bytes_to_string(bytes: &[u8]) -> ByteString {
        let method = ByteString::from("itoa");
        let mut args = Array::<Any>::new();
        args.push(Any::from(bytes));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or_else(|_| ByteString::from(""))
    }

    /// Converts a base64 string to a byte array
    /// 
    /// # Arguments
    /// 
    /// * `base64_str` - The base64 string to decode
    /// 
    /// # Returns
    /// 
    /// The decoded byte array
    #[safe]
    pub fn base64_decode(base64_str: &ByteString) -> Vec<u8> {
        let method = ByteString::from("base64Decode");
        let mut args = Array::<Any>::new();
        args.push(Any::from(base64_str.clone()));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        match result.try_into::<Vec<u8>>() {
            Ok(data) => data,
            Err(_) => Vec::new(),
        }
    }

    /// Converts a byte array to a base64 string
    /// 
    /// # Arguments
    /// 
    /// * `data` - The byte array to encode
    /// 
    /// # Returns
    /// 
    /// The base64 encoded string
    #[safe]
    pub fn base64_encode(data: &[u8]) -> ByteString {
        let method = ByteString::from("base64Encode");
        let mut args = Array::<Any>::new();
        args.push(Any::from(data));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or_else(|_| ByteString::from(""))
    }

    /// Encodes a byte array to a hexadecimal string
    /// 
    /// # Arguments
    /// 
    /// * `data` - The byte array to encode
    /// 
    /// # Returns
    /// 
    /// The hexadecimal encoded string
    #[safe]
    pub fn hex_encode(data: &[u8]) -> ByteString {
        let method = ByteString::from("binaryToHex");
        let mut args = Array::<Any>::new();
        args.push(Any::from(data));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or_else(|_| ByteString::from(""))
    }

    /// Decodes a hexadecimal string to a byte array
    /// 
    /// # Arguments
    /// 
    /// * `hex_str` - The hexadecimal string to decode
    /// 
    /// # Returns
    /// 
    /// The decoded byte array
    #[safe]
    pub fn hex_decode(hex_str: &ByteString) -> Vec<u8> {
        let method = ByteString::from("hexToBinary");
        let mut args = Array::<Any>::new();
        args.push(Any::from(hex_str.clone()));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        match result.try_into::<Vec<u8>>() {
            Ok(data) => data,
            Err(_) => Vec::new(),
        }
    }
}
