//! Map type for Neo Contract RS
//!
//! This module defines the Map type, which is used for key-value dictionaries in Neo.

use super::any::Any;
use alloc::vec::Vec;
use core::fmt;

/// Map represents a key-value dictionary in Neo
///
/// K and V are phantom type parameters that help with type safety
/// while maintaining compatibility with Neo N3 VM representation
#[derive(Clone, Default)]
pub struct Map<K = Any, V = Any>(pub Vec<(Any, Any)>, core::marker::PhantomData<(K, V)>);

impl<K, V> Map<K, V> {
    /// Creates a new empty Map
    pub fn new() -> Self { Map(Vec::new(), core::marker::PhantomData) }

    /// Creates a Map with the given capacity
    pub fn with_capacity(capacity: usize) -> Self { Map(Vec::with_capacity(capacity), core::marker::PhantomData) }

    /// Creates a Map from a vector of key-value pairs
    pub fn from_vec(vec: Vec<(Any, Any)>) -> Self { Map(vec, core::marker::PhantomData) }

    /// Returns the number of key-value pairs in the Map
    pub fn len(&self) -> usize { self.0.len() }

    /// Checks if the Map is empty
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Gets a reference to a value by key
    pub fn get(&self, key: &Any) -> Option<&Any> {
        for (k, v) in &self.0 {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    /// Gets a mutable reference to a value by key
    pub fn get_mut(&mut self, key: &Any) -> Option<&mut Any> {
        for (k, v) in &mut self.0 {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    /// Sets a value for the given key
    pub fn set<KeyType: Into<Any>, ValueType: Into<Any>>(&mut self, key: KeyType, value: ValueType) {
        let key = key.into();
        let value = value.into();

        // Check if the key already exists
        for entry in &mut self.0 {
            if entry.0 == key {
                entry.1 = value;
                return;
            }
        }

        // Key doesn't exist, add a new entry
        self.0.push((key, value));
    }

    /// Removes a key-value pair by key
    pub fn remove(&mut self, key: &Any) -> Option<Any> {
        let index = self.0.iter().position(|(k, _)| k == key)?;
        Some(self.0.remove(index).1)
    }

    /// Checks if the Map contains the given key
    pub fn contains_key(&self, key: &Any) -> bool { self.0.iter().any(|(k, _)| k == key) }

    /// Clears the Map
    pub fn clear(&mut self) { self.0.clear(); }

    /// Returns an iterator over the key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = &(Any, Any)> { self.0.iter() }

    /// Returns a mutable iterator over the key-value pairs
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (Any, Any)> { self.0.iter_mut() }

    /// Returns an iterator over the keys
    pub fn keys(&self) -> impl Iterator<Item = &Any> { self.0.iter().map(|(k, _)| k) }

    /// Returns an iterator over the values
    pub fn values(&self) -> impl Iterator<Item = &Any> { self.0.iter().map(|(_, v)| v) }

    /// Returns a reference to the underlying vector
    pub fn as_vec(&self) -> &Vec<(Any, Any)> { &self.0 }

    /// Converts the Map into a vector
    pub fn into_vec(self) -> Vec<(Any, Any)> { self.0 }
}

impl<K, V> From<Vec<(Any, Any)>> for Map<K, V> {
    fn from(vec: Vec<(Any, Any)>) -> Self { Map(vec, core::marker::PhantomData) }
}

impl<K1, V1, K2: Into<Any>, V2: Into<Any>> FromIterator<(K2, V2)> for Map<K1, V1> {
    fn from_iter<T: IntoIterator<Item = (K2, V2)>>(iter: T) -> Self {
        let vec = iter.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        Map(vec, core::marker::PhantomData)
    }
}

impl<K, V> fmt::Debug for Map<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Map({{")?;
        for (i, (k, v)) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}: {:?}", k, v)?;
        }
        write!(f, "}})")
    }
}

impl IntoIterator for Map {
    type Item = (Any, Any);
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a> IntoIterator for &'a Map {
    type Item = &'a (Any, Any);
    type IntoIter = core::slice::Iter<'a, (Any, Any)>;

    fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

impl<'a> IntoIterator for &'a mut Map {
    type Item = &'a mut (Any, Any);
    type IntoIter = core::slice::IterMut<'a, (Any, Any)>;

    fn into_iter(self) -> Self::IntoIter { self.0.iter_mut() }
}
