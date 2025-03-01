// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use alloc::string::String;
use core::marker::PhantomData;
use crate::storage::{delete, get, put, find};
use crate::types::builtin::string::ByteString;
use crate::types::context::StorageContext;

/// StorageMap represents a map in storage
pub struct StorageMap<K, V> {
    context: StorageContext,
    prefix: Vec<u8>,
    _key: PhantomData<K>,
    _value: PhantomData<V>,
}

impl<K, V> StorageMap<K, V> {
    /// Create a new storage map
    pub fn new(context: StorageContext, prefix: Vec<u8>) -> Self {
        Self {
            context,
            prefix,
            _key: PhantomData,
            _value: PhantomData,
        }
    }

    /// Get the context
    pub fn context(&self) -> StorageContext {
        self.context.clone()
    }

    /// Get the prefix
    pub fn prefix(&self) -> Vec<u8> {
        self.prefix.clone()
    }
}

impl<K, V> StorageMap<K, V>
where
    K: AsRef<[u8]>,
    V: TryFrom<ByteString> + Into<ByteString>,
{
    /// Find values in the map
    pub fn find(&self) -> i32 {
        find(self.context.clone(), ByteString::from(self.prefix.clone()))
    }

    /// Get a value from the map
    pub fn get(&self, key: K) -> Option<V> {
        let mut full_key = self.prefix.clone();
        full_key.extend_from_slice(key.as_ref());
        let value = get(self.context.clone(), ByteString::from(full_key));
        V::try_from(value).ok()
    }

    /// Put a value in the map
    pub fn put(&self, key: K, value: V) {
        let mut full_key = self.prefix.clone();
        full_key.extend_from_slice(key.as_ref());
        put(self.context.clone(), ByteString::from(full_key), value.into());
    }

    /// Delete a value from the map
    pub fn delete(&self, key: K) {
        let mut full_key = self.prefix.clone();
        full_key.extend_from_slice(key.as_ref());
        delete(self.context.clone(), ByteString::from(full_key));
    }
}

impl<V> StorageMap<ByteString, V>
where
    V: TryFrom<ByteString> + Into<ByteString>,
{
    /// Get a value from the map
    pub fn get_string(&self, key: &str) -> Option<V> {
        self.get(ByteString::from(key))
    }

    /// Put a value in the map
    pub fn put_string(&self, key: &str, value: V) {
        self.put(ByteString::from(key), value);
    }

    /// Delete a value from the map
    pub fn delete_string(&self, key: &str) {
        self.delete(ByteString::from(key));
    }
}

use crate::types::builtin::int256::Int256;

impl<K> StorageMap<K, Int256>
where
    K: AsRef<[u8]>,
{
    /// Get an integer value from the map
    pub fn get_int(&self, key: K) -> Int256 {
        match self.get(key) {
            Some(value) => value,
            None => Int256::zero()
        }
    }
}
