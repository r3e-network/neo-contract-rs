//! Storage Iterator for Neo Contract RS
//!
//! This module provides an iterator API for storage.

use alloc::vec::Vec;
use core::marker::PhantomData;
use crate::error::{Error, ErrorCode, Result};
use crate::find_options::FindOptions;
use super::context::Context;
use super::item::Codec;

/// An iterator over storage entries
pub struct StorageIter<K, V> {
    /// The items in the iterator
    items: Vec<(Vec<u8>, Vec<u8>)>,
    
    /// The current position in the iterator
    position: usize,
    
    /// The prefix to remove from keys
    prefix: Vec<u8>,
    
    /// The type of keys in this iterator
    _key_marker: PhantomData<K>,
    
    /// The type of values in this iterator
    _value_marker: PhantomData<V>,
}

impl<K, V> StorageIter<K, V>
where
    K: Codec,
    V: Codec,
{
    /// Creates a new storage iterator with the given items and prefix
    pub fn new(items: Vec<(Vec<u8>, Vec<u8>)>, prefix: impl AsRef<[u8]>) -> Self {
        StorageIter {
            items,
            position: 0,
            prefix: prefix.as_ref().to_vec(),
            _key_marker: PhantomData,
            _value_marker: PhantomData,
        }
    }
    
    /// Creates a storage iterator that finds entries with the given prefix and options
    pub fn find(prefix: impl AsRef<[u8]>, options: FindOptions) -> Self {
        let items = super::find(prefix.as_ref(), options);
        Self::new(items, prefix)
    }
    
    /// Creates a storage iterator that finds entries with the given prefix, context, and options
    pub fn find_with_context(prefix: impl AsRef<[u8]>, context: &Context, options: FindOptions) -> Self {
        let items = context.find(prefix.as_ref(), options);
        Self::new(items, prefix)
    }
    
    /// Gets the next entry in the iterator
    pub fn next(&mut self) -> Option<Result<(K, V)>> {
        if self.position >= self.items.len() {
            return None;
        }
        
        let (key, value) = &self.items[self.position];
        self.position += 1;
        
        // Skip the prefix in the key
        let key_bytes = &key[self.prefix.len()..];
        
        // Try to decode the key and value
        match (K::decode(key_bytes), V::decode(value)) {
            (Ok(k), Ok(v)) => Some(Ok((k, v))),
            (Err(_e), _) => Some(Err(Error::new(ErrorCode::DecodingError))),
            (_, Err(_e)) => Some(Err(Error::new(ErrorCode::DecodingError))),
        }
    }
    
    /// Gets all entries in the iterator
    pub fn collect(&mut self) -> Result<Vec<(K, V)>> {
        let mut result = Vec::new();
        
        while let Some(entry) = self.next() {
            result.push(entry?);
        }
        
        Ok(result)
    }
    
    /// Resets the iterator to the beginning
    pub fn reset(&mut self) {
        self.position = 0;
    }
    
    /// Gets the number of items in the iterator
    pub fn len(&self) -> usize {
        self.items.len()
    }
    
    /// Checks if the iterator is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    /// Gets the current position in the iterator
    pub fn position(&self) -> usize {
        self.position
    }
    
    /// Checks if the iterator has reached the end
    pub fn is_end(&self) -> bool {
        self.position >= self.items.len()
    }
}

/// An iterator adapter that implements the Iterator trait
pub struct StorageIterator<K, V> {
    /// The storage iterator
    iter: StorageIter<K, V>,
}

impl<K, V> StorageIterator<K, V>
where
    K: Codec,
    V: Codec,
{
    /// Creates a new storage iterator adapter
    pub fn new(iter: StorageIter<K, V>) -> Self {
        StorageIterator { iter }
    }
}

impl<K, V> Iterator for StorageIterator<K, V>
where
    K: Codec,
    V: Codec,
{
    type Item = Result<(K, V)>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
