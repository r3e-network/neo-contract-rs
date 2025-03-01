// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use core::fmt;
use core::ops::{Deref, DerefMut};

/// Array represents a dynamic array of values
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Array<T> {
    items: Vec<T>,
}

impl<T> Array<T> {
    /// Create a new empty array
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Create an array from a vector
    pub fn from_vec(items: Vec<T>) -> Self {
        Self { items }
    }

    /// Get the length of the array
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the array is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Push an item to the array
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Pop an item from the array
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Get an item from the array
    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    /// Get a mutable reference to an item in the array
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }

    /// Set an item in the array
    pub fn set(&mut self, index: usize, item: T) -> bool {
        if index < self.items.len() {
            self.items[index] = item;
            true
        } else {
            false
        }
    }

    /// Remove an item from the array
    pub fn remove(&mut self, index: usize) -> T {
        self.items.remove(index)
    }

    /// Clear the array
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl<T> From<Vec<T>> for Array<T> {
    fn from(items: Vec<T>) -> Self {
        Self { items }
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

impl<T: fmt::Display> fmt::Display for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
