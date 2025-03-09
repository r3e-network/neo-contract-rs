//! Array type for Neo Contract RS
//!
//! This module defines the Array type, which is used for dynamic arrays in Neo.

use core::fmt;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use alloc::vec::Vec;
use super::any::Any;

/// Array represents a dynamic array of values in Neo
#[derive(Clone, Default)]
pub struct Array(pub Vec<Any>);

impl Array {
    /// Creates a new empty Array
    pub fn new() -> Self {
        Array(Vec::new())
    }
    
    /// Creates an Array with the given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Array(Vec::with_capacity(capacity))
    }
    
    /// Creates an Array from a vector of Any values
    pub fn from_vec(vec: Vec<Any>) -> Self {
        Array(vec)
    }
    
    /// Returns the length of the Array
    pub fn len(&self) -> usize {
        self.0.len()
    }
    
    /// Checks if the Array is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    
    /// Pushes a value to the end of the Array
    pub fn push<T: Into<Any>>(&mut self, value: T) {
        self.0.push(value.into());
    }
    
    /// Pops a value from the end of the Array
    pub fn pop(&mut self) -> Option<Any> {
        self.0.pop()
    }
    
    /// Gets a reference to a value by index
    pub fn get(&self, index: usize) -> Option<&Any> {
        self.0.get(index)
    }
    
    /// Gets a mutable reference to a value by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Any> {
        self.0.get_mut(index)
    }
    
    /// Sets a value at the given index
    pub fn set<T: Into<Any>>(&mut self, index: usize, value: T) -> Result<(), &'static str> {
        if index < self.0.len() {
            self.0[index] = value.into();
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
    
    /// Removes a value at the given index
    pub fn remove(&mut self, index: usize) -> Any {
        self.0.remove(index)
    }
    
    /// Inserts a value at the given index
    pub fn insert<T: Into<Any>>(&mut self, index: usize, value: T) {
        self.0.insert(index, value.into());
    }
    
    /// Clears the Array
    pub fn clear(&mut self) {
        self.0.clear();
    }
    
    /// Returns an iterator over the Array
    pub fn iter(&self) -> impl Iterator<Item = &Any> {
        self.0.iter()
    }
    
    /// Returns a mutable iterator over the Array
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Any> {
        self.0.iter_mut()
    }
    
    /// Resizes the Array to the given length with the given value
    pub fn resize<T: Into<Any> + Clone>(&mut self, len: usize, value: T) {
        let value = value.into();
        if len > self.0.len() {
            let additional = len - self.0.len();
            for _ in 0..additional {
                self.0.push(value.clone());
            }
        } else if len < self.0.len() {
            self.0.truncate(len);
        }
    }
    
    /// Returns a reference to the underlying vector
    pub fn as_vec(&self) -> &Vec<Any> {
        &self.0
    }
    
    /// Converts the Array into a vector
    pub fn into_vec(self) -> Vec<Any> {
        self.0
    }
}

impl Deref for Array {
    type Target = Vec<Any>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Array {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<usize> for Array {
    type Output = Any;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Array {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<Vec<Any>> for Array {
    fn from(vec: Vec<Any>) -> Self {
        Array(vec)
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Array(")?;
        f.debug_list().entries(self.0.iter()).finish()?;
        write!(f, ")")
    }
}

impl IntoIterator for Array {
    type Item = Any;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Array {
    type Item = &'a Any;
    type IntoIter = core::slice::Iter<'a, Any>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Array {
    type Item = &'a mut Any;
    type IntoIter = core::slice::IterMut<'a, Any>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
