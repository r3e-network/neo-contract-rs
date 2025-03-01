// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use core::fmt;
use alloc::collections::BTreeMap;
use core::ops::{Deref, DerefMut};
use crate::types::builtin::any::Any;

/// Map represents a map
#[derive(Debug, Clone, PartialEq)]
pub struct Map<K, V> {
    items: BTreeMap<K, V>,
}

impl<K, V> Map<K, V>
where
    K: Ord,
{
    /// Create a new empty map
    pub fn new() -> Self {
        Map {
            items: BTreeMap::new(),
        }
    }

    /// Get the length of the map
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get a reference to the value for the given key
    pub fn get(&self, key: &K) -> Option<&V> {
        self.items.get(key)
    }

    /// Get a mutable reference to the value for the given key
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.items.get_mut(key)
    }

    /// Insert a key-value pair into the map
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.items.insert(key, value)
    }

    /// Remove a key-value pair from the map
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.items.remove(key)
    }

    /// Clear the map
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get the keys of the map
    pub fn keys(&self) -> Vec<&K> {
        self.items.keys().collect()
    }

    /// Get the values of the map
    pub fn values(&self) -> Vec<&V> {
        self.items.values().collect()
    }
}

impl<K, V> Deref for Map<K, V>
where
    K: Ord,
{
    type Target = BTreeMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<K, V> DerefMut for Map<K, V>
where
    K: Ord,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl<K, V> From<BTreeMap<K, V>> for Map<K, V>
where
    K: Ord,
{
    fn from(items: BTreeMap<K, V>) -> Self {
        Map { items }
    }
}

impl<K, V> From<Map<K, V>> for BTreeMap<K, V>
where
    K: Ord,
{
    fn from(map: Map<K, V>) -> Self {
        map.items
    }
}

impl<K, V> Default for Map<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Map::new()
    }
}

impl<K, V> fmt::Display for Map<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
    K: Ord,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for (i, (key, value)) in self.items.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", key, value)?;
        }
        write!(f, "}}")
    }
}

impl<K: Clone + Ord + 'static, V: Clone + 'static> TryFrom<Any> for Map<K, V> {
    type Error = ();

    fn try_from(any: Any) -> Result<Self, Self::Error> {
        if let Some(map) = any.cast::<Self>() {
            Ok(map.clone())
        } else {
            Err(())
        }
    }
}
