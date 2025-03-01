// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use core::fmt;
use core::ops::{Deref, DerefMut};
use crate::types::builtin::any::Any;

/// Array represents an array
#[derive(Debug, Clone, PartialEq)]
pub struct Array<T> {
    items: Vec<T>,
}

impl<T> Array<T> {
    /// Create a new empty array
    pub fn new() -> Self {
        Array { items: Vec::new() }
    }

    /// Create a new array with the given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Array {
            items: Vec::with_capacity(capacity),
        }
    }

    /// Create a new array from a vector
    pub fn from_vec(items: Vec<T>) -> Self {
        Array { items }
    }

    /// Get the length of the array
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the array is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get a reference to the item at the given index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    /// Get a mutable reference to the item at the given index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }

    /// Push an item to the end of the array
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Pop an item from the end of the array
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Insert an item at the given index
    pub fn insert(&mut self, index: usize, item: T) {
        self.items.insert(index, item);
    }

    /// Remove an item at the given index
    pub fn remove(&mut self, index: usize) -> T {
        self.items.remove(index)
    }

    /// Clear the array
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get the underlying vector
    pub fn into_vec(self) -> Vec<T> {
        self.items
    }
}

impl<T> Deref for Array<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<T> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl<T> From<Vec<T>> for Array<T> {
    fn from(items: Vec<T>) -> Self {
        Array { items }
    }
}

impl<T> From<Array<T>> for Vec<T> {
    fn from(array: Array<T>) -> Self {
        array.items
    }
}

impl<T> Default for Array<T> {
    fn default() -> Self {
        Array::new()
    }
}

impl<T> fmt::Display for Array<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

impl<T: Clone + 'static> TryFrom<Any> for Array<T> {
    type Error = ();

    fn try_from(any: Any) -> Result<Self, Self::Error> {
        if let Some(array) = any.cast::<Self>() {
            Ok(array.clone())
        } else {
            Err(())
        }
    }
}
