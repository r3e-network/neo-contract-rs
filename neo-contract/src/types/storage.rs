// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Storage types for Neo N3 smart contracts
//!
//! This module provides types for interacting with smart contract storage,
//! including StorageItem and StorageMap.

use crate::prelude::*;
use crate::types::builtin::string::ByteString;
use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;

/// Trait for encoding and decoding values to/from storage
pub trait Codec: Sized {
    /// Encode the value to bytes
    fn encode(&self) -> Vec<u8>;

    /// Decode the value from bytes
    fn decode(bytes: &[u8]) -> Option<Self>;
}

/// Storage context for Neo N3 contract storage
pub struct StorageContext {
    prefix: Vec<u8>,
}

impl StorageContext {
    /// Create a new storage context with the given prefix
    pub fn new(prefix: &[u8]) -> Self { Self { prefix: prefix.to_vec() } }

    /// Get the prefix as bytes
    pub fn as_bytes(&self) -> &[u8] { &self.prefix }
}

/// A single item in smart contract storage
pub struct StorageItem<T: Codec> {
    key: Vec<u8>,
    phantom: PhantomData<T>,
}

impl<T: Codec> StorageItem<T> {
    /// Create a new storage item with the given key
    pub fn new(key: &[u8]) -> Self { Self { key: key.to_vec(), phantom: PhantomData } }

    /// Get the value from storage
    pub fn get(&self) -> Option<T> {
        let context = StorageContext::new(b"");
        let value_bytes = crate::env::syscall::system_storage_get(&context, &self.key);

        if value_bytes.is_empty() {
            None
        } else {
            T::decode(&value_bytes)
        }
    }

    /// Set the value in storage
    pub fn set(&self, value: &T) {
        let context = StorageContext::new(b"");
        let value_bytes = value.encode();
        crate::env::syscall::system_storage_put(&context, &self.key, &value_bytes);
    }

    /// Delete the value from storage
    pub fn delete(&self) {
        let context = StorageContext::new(b"");
        crate::env::syscall::system_storage_delete(&context, &self.key);
    }
}

/// A map in smart contract storage
pub struct StorageMap<K: Codec, V: Codec> {
    prefix: Vec<u8>,
    phantom_k: PhantomData<K>,
    phantom_v: PhantomData<V>,
}

impl<K: Codec, V: Codec> StorageMap<K, V> {
    /// Create a new storage map with the given prefix
    pub fn new(prefix: &[u8]) -> Self {
        Self {
            prefix: prefix.to_vec(),
            phantom_k: PhantomData,
            phantom_v: PhantomData,
        }
    }

    /// Create a full key from the prefix and key
    fn make_key(&self, key: &K) -> Vec<u8> {
        let mut full_key = self.prefix.clone();
        full_key.extend_from_slice(&key.encode());
        full_key
    }

    /// Get a value from the map
    pub fn get(&self, key: &K) -> Option<V> {
        let context = StorageContext::new(b"");
        let full_key = self.make_key(key);
        let value_bytes = crate::env::syscall::system_storage_get(&context, &full_key);

        if value_bytes.is_empty() {
            None
        } else {
            V::decode(&value_bytes)
        }
    }

    /// Set a value in the map
    pub fn set(&self, key: &K, value: &V) {
        let context = StorageContext::new(b"");
        let full_key = self.make_key(key);
        let value_bytes = value.encode();
        crate::env::syscall::system_storage_put(&context, &full_key, &value_bytes);
    }

    /// Delete a value from the map
    pub fn delete(&self, key: &K) {
        let context = StorageContext::new(b"");
        let full_key = self.make_key(key);
        crate::env::syscall::system_storage_delete(&context, &full_key);
    }
}

// Implement Codec for common types
impl Codec for u32 {
    fn encode(&self) -> Vec<u8> { self.to_be_bytes().to_vec() }

    fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() == 4 {
            let mut array = [0u8; 4];
            array.copy_from_slice(bytes);
            Some(u32::from_be_bytes(array))
        } else {
            None
        }
    }
}

impl Codec for u64 {
    fn encode(&self) -> Vec<u8> { self.to_be_bytes().to_vec() }

    fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() == 8 {
            let mut array = [0u8; 8];
            array.copy_from_slice(bytes);
            Some(u64::from_be_bytes(array))
        } else {
            None
        }
    }
}

impl Codec for u8 {
    fn encode(&self) -> Vec<u8> { vec![*self] }

    fn decode(bytes: &[u8]) -> Option<Self> { bytes.first().copied() }
}

impl Codec for bool {
    fn encode(&self) -> Vec<u8> { vec![if *self { 1 } else { 0 }] }

    fn decode(bytes: &[u8]) -> Option<Self> { bytes.first().map(|&b| b != 0) }
}

impl Codec for H160 {
    fn encode(&self) -> Vec<u8> { self.as_bytes().to_vec() }

    fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() == 20 {
            Some(H160::from_slice(bytes))
        } else {
            None
        }
    }
}

impl Codec for ByteString {
    fn encode(&self) -> Vec<u8> { self.as_bytes().to_vec() }

    fn decode(bytes: &[u8]) -> Option<Self> { Some(ByteString::from(bytes)) }
}

impl Codec for String {
    fn encode(&self) -> Vec<u8> { self.as_bytes().to_vec() }

    fn decode(bytes: &[u8]) -> Option<Self> { core::str::from_utf8(bytes).ok().map(String::from) }
}

impl<T: Codec> Codec for Vec<T> {
    fn encode(&self) -> Vec<u8> {
        // For simplicity, we'll just concatenate the encoded values
        // In a real implementation, you'd want to store length information
        let mut result = Vec::new();
        for item in self {
            let encoded = item.encode();
            result.extend_from_slice(&encoded);
        }
        result
    }

    fn decode(bytes: &[u8]) -> Option<Self> {
        // This is a very simplified implementation
        // In a real implementation, you'd need more sophisticated deserialization
        Some(Vec::new())
    }
}
