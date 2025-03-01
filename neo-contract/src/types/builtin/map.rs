// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use core::fmt;
use crate::types::builtin::string::ByteString;

/// Map represents a map of key-value pairs
#[derive(Debug, Clone)]
pub struct Map<K, V> {
    /// Keys
    pub keys: Vec<K>,
    /// Values
    pub values: Vec<V>,
}

impl<K, V> Map<K, V> {
    /// Create a new map
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Get the number of key-value pairs
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Put a key-value pair in the map
    pub fn put(&mut self, key: K, value: V) where K: PartialEq {
        if let Some(index) = self.keys.iter().position(|k| k == &key) {
            self.values[index] = value;
        } else {
            self.keys.push(key);
            self.values.push(value);
        }
    }

    /// Delete a key-value pair from the map
    pub fn delete(&mut self, key: &K) where K: PartialEq {
        if let Some(index) = self.keys.iter().position(|k| k == key) {
            self.keys.remove(index);
            self.values.remove(index);
        }
    }
}

impl<K, V> Default for Map<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: fmt::Display, V: fmt::Display> fmt::Display for Map<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Map({} items)", self.len())
    }
}
