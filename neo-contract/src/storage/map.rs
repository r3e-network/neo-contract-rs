//! Storage Map for Neo Contract RS
//!
//! This module provides a map-like storage API for Neo smart contracts.

use alloc::vec::Vec;
use core::marker::PhantomData;
use crate::error::{Error, ErrorCode, Result};
use crate::find_options::FindOptions;
use super::context::Context;
use super::item::{Item, Codec};

/// A high-level storage map
pub struct Map<K, V> {
    /// The prefix used for keys in this map
    prefix: Vec<u8>,
    
    /// The storage context
    context: Option<Context>,
    
    /// The type of keys in this map
    _key_marker: PhantomData<K>,
    
    /// The type of values in this map
    _value_marker: PhantomData<V>,
}

impl<K, V> Map<K, V>
where
    K: Codec,
    V: Codec,
{
    /// Creates a new storage map with the given prefix
    pub fn new(prefix: impl AsRef<[u8]>) -> Self {
        Map {
            prefix: prefix.as_ref().to_vec(),
            context: None,
            _key_marker: PhantomData,
            _value_marker: PhantomData,
        }
    }
    
    /// Creates a new storage map with the given prefix and context
    pub fn with_context(prefix: impl AsRef<[u8]>, context: Context) -> Self {
        Map {
            prefix: prefix.as_ref().to_vec(),
            context: Some(context),
            _key_marker: PhantomData,
            _value_marker: PhantomData,
        }
    }
    
    /// Creates a storage key for the given map key
    fn create_key(&self, key: &K) -> Vec<u8> {
        let mut storage_key = self.prefix.clone();
        storage_key.extend_from_slice(&key.encode());
        storage_key
    }
    
    /// Gets a value from the map
    pub fn get(&self, key: &K) -> Result<Option<V>> {
        let storage_key = self.create_key(key);
        
        let value = if let Some(context) = &self.context {
            context.get(&storage_key)
        } else {
            super::get(&storage_key)
        };
        
        match value {
            Some(bytes) => Ok(Some(V::decode(&bytes)?)),
            None => Ok(None),
        }
    }
    
    /// Sets a value in the map
    pub fn set(&self, key: &K, value: &V) -> Result<()> {
        let storage_key = self.create_key(key);
        let bytes = value.encode();
        
        if let Some(context) = &self.context {
            context.put(&storage_key, &bytes);
        } else {
            super::put(&storage_key, &bytes);
        }
        
        Ok(())
    }
    
    /// Stores a value in the map
    pub fn put(&self, key: &K, value: &V) -> Result<()> {
        self.set(key, value)
    }
    
    /// Removes a value from the map
    pub fn delete(&self, key: &K) -> Result<()> {
        let storage_key = self.create_key(key);
        
        if let Some(context) = &self.context {
            context.delete(&storage_key);
        } else {
            super::delete(&storage_key);
        }
        
        Ok(())
    }
    
    /// Removes a value from the map
    pub fn remove(&self, key: &K) -> Result<()> {
        self.delete(key)
    }
    
    /// Checks if a key exists in the map
    pub fn has(&self, key: &K) -> bool {
        let storage_key = self.create_key(key);
        
        if let Some(context) = &self.context {
            context.has(&storage_key)
        } else {
            super::has(&storage_key)
        }
    }
    
    /// Finds entries in the map with the given options
    pub fn find(&self, options: FindOptions) -> Vec<(K, V)> {
        let items = if let Some(context) = &self.context {
            context.find(&self.prefix, options)
        } else {
            super::find(&self.prefix, options)
        };
        
        let mut result = Vec::new();
        
        for (key, value) in items {
            // Skip the prefix in the key
            let key_bytes = &key[self.prefix.len()..];
            
            // Try to decode the key and value
            match (K::decode(key_bytes), V::decode(&value)) {
                (Ok(k), Ok(v)) => result.push((k, v)),
                _ => continue, // Skip entries that can't be decoded
            }
        }
        
        result
    }
    
    /// Gets the prefix of this map
    pub fn prefix(&self) -> &[u8] {
        &self.prefix
    }
    
    /// Gets the context of this map
    pub fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }
    
    /// Sets the context for this map
    pub fn set_context(&mut self, context: Context) {
        self.context = Some(context);
    }
    
    /// Clears the context for this map
    pub fn clear_context(&mut self) {
        self.context = None;
    }
    
    /// Clears all entries in the map
    pub fn clear(&self) -> Result<()> {
        // Find all keys with this prefix
        let items = if let Some(context) = &self.context {
            context.find(&self.prefix, FindOptions::KEYS_ONLY)
        } else {
            super::find(&self.prefix, FindOptions::KEYS_ONLY)
        };
        
        // Delete each key
        for (key, _) in items {
            if let Some(context) = &self.context {
                context.delete(&key);
            } else {
                super::delete(&key);
            }
        }
        
        Ok(())
    }
}
