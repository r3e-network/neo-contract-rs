// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::vec::Vec;
use core::marker::PhantomData;
use crate::builtin::{H160, ByteString, Int256};
use crate::types::context::StorageContext;
use crate::Runtime;
use crate::storage::iter::{StorageIterator, TypedStorageIterator};

/// Trait for types that can be serialized to storage
pub trait Storable {
    /// Serialize the value to bytes
    fn to_bytes(&self) -> Vec<u8>;
    
    /// Deserialize the value from bytes
    fn from_bytes(bytes: &[u8]) -> Option<Self> where Self: Sized;
}

// Implement Storable for common types
impl Storable for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
    
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.get(0).copied()
    }
}

impl Storable for u32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= 4 {
            let mut array = [0u8; 4];
            array.copy_from_slice(&bytes[0..4]);
            Some(u32::from_le_bytes(array))
        } else {
            None
        }
    }
}

impl Storable for u64 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= 8 {
            let mut array = [0u8; 8];
            array.copy_from_slice(&bytes[0..8]);
            Some(u64::from_le_bytes(array))
        } else {
            None
        }
    }
}

impl Storable for bool {
    fn to_bytes(&self) -> Vec<u8> {
        vec![if *self { 1 } else { 0 }]
    }
    
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.get(0).map(|&b| b != 0)
    }
}

impl Storable for H160 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
    
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= 20 {
            let mut array = [0u8; 20];
            array.copy_from_slice(&bytes[0..20]);
            Some(H160::new(array))
        } else {
            None
        }
    }
}

impl Storable for ByteString {
    fn to_bytes(&self) -> Vec<u8> {
        self.clone().into()
    }
    
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Some(ByteString::from_bytes(bytes))
    }
}

impl Storable for Int256 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
    
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Some(Int256::from_bytes(bytes))
    }
}

impl Storable for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
    
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        String::from_utf8(bytes.to_vec()).ok()
    }
}

/// Serializes a key for storage
pub trait StorageKey {
    /// Get the serialized key
    fn serialized_key(&self) -> ByteString;
}

impl<T: Storable> StorageKey for T {
    fn serialized_key(&self) -> ByteString {
        ByteString::from_bytes(&self.to_bytes())
    }
}

/// Storage map entry representing a key in the map
pub struct StorageMapEntry<'a, K, V> {
    /// The map this entry belongs to
    map: &'a StorageMap<K, V>,
    /// The key for this entry
    key: K,
}

impl<'a, K: StorageKey + Clone, V: Storable> StorageMapEntry<'a, K, V> {
    /// Create a new storage map entry
    fn new(map: &'a StorageMap<K, V>, key: K) -> Self {
        Self { map, key }
    }
    
    /// Check if the entry exists
    pub fn exists(&self) -> bool {
        self.get().is_some()
    }
    
    /// Get the value of this entry
    pub fn get(&self) -> Option<V> {
        let key_bs = self.key.serialized_key();
        let storage_context = self.map.context();
        
        let bytes = Runtime::storage_get(storage_context, key_bs.as_bytes())?;
        V::from_bytes(&bytes)
    }
    
    /// Set the value of this entry
    pub fn put(&self, value: &V) {
        let key_bs = self.key.serialized_key();
        let value_bytes = value.to_bytes();
        let storage_context = self.map.context();
        
        Runtime::storage_put(storage_context, key_bs.as_bytes(), &value_bytes);
    }
    
    /// Delete this entry
    pub fn delete(&self) {
        let key_bs = self.key.serialized_key();
        let storage_context = self.map.context();
        
        Runtime::storage_delete(storage_context, key_bs.as_bytes());
    }
}

/// StorageMap represents a key-value storage map
#[derive(Debug, Clone)]
pub struct StorageMap<K, V> {
    context: StorageContext,
    prefix: ByteString,
    _marker_k: PhantomData<K>,
    _marker_v: PhantomData<V>,
}

impl<K: StorageKey + Clone, V: Storable> StorageMap<K, V> {
    /// Create a new storage map with a prefix
    pub fn new(prefix: &[u8]) -> Self {
        Self {
            context: Runtime::storage_context(),
            prefix: ByteString::from_bytes(prefix),
            _marker_k: PhantomData,
            _marker_v: PhantomData,
        }
    }
    
    /// Create a new storage map with the default context
    pub fn new_default() -> Self {
        Self {
            context: Runtime::storage_context(),
            prefix: ByteString::empty(),
            _marker_k: PhantomData,
            _marker_v: PhantomData,
        }
    }

    /// Get the context
    pub fn context(&self) -> StorageContext {
        self.context
    }
    
    /// Get the prefix
    pub fn prefix(&self) -> ByteString {
        self.prefix.clone()
    }
    
    /// Get an entry for a key
    pub fn entry(&self, key: K) -> StorageMapEntry<K, V> {
        StorageMapEntry::new(self, key)
    }
    
    /// Check if a key exists
    pub fn contains_key(&self, key: &K) -> bool {
        self.entry(key.clone()).exists()
    }

    /// Get a value from the map
    pub fn get(&self, key: &K) -> Option<V> {
        self.entry(key.clone()).get()
    }

    /// Put a value in the map
    pub fn put(&self, key: &K, value: &V) {
        self.entry(key.clone()).put(value);
    }

    /// Delete a value from the map
    pub fn delete(&self, key: &K) {
        self.entry(key.clone()).delete();
    }
    
    /// Get an iterator over the map with a specific prefix
    pub fn iterate_with_prefix(&self, prefix: &[u8]) -> StorageIterator {
        let context = self.context();
        let prefix_bs = ByteString::from_bytes(prefix);
        StorageIterator::new(context, prefix_bs)
    }
    
    /// Get a typed iterator over the map with a specific prefix
    pub fn iterate_typed_with_prefix(&self, prefix: &[u8]) -> TypedStorageIterator<K, V> 
    where
        K: TryFrom<ByteString>,
        V: TryFrom<ByteString>,
    {
        let context = self.context();
        let prefix_bs = ByteString::from_bytes(prefix);
        TypedStorageIterator::new(context, prefix_bs)
    }
    
    /// Get an iterator over the entire map
    pub fn iterate(&self) -> StorageIterator {
        self.iterate_with_prefix(&self.prefix().into())
    }
    
    /// Clear all entries with a specific prefix
    pub fn clear_prefix(&self, prefix: &[u8]) {
        let iter = self.iterate_with_prefix(prefix);
        while iter.has_next() {
            if let Some((key, _)) = iter.next() {
                Runtime::storage_delete(self.context(), key.as_bytes());
            }
        }
    }
    
    /// Clear all entries in the map
    pub fn clear(&self) {
        self.clear_prefix(&self.prefix().into());
    }
}
