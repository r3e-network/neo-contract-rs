// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::types::builtin::{Any, FromAny, IntoAny};
use core::ops::{Deref, DerefMut};

/// Buffer type for Neo smart contracts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buffer(alloc::vec::Vec<u8>);

impl Buffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        Self(alloc::vec::Vec::new())
    }

    /// Create a new buffer with the given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self(alloc::vec::Vec::with_capacity(capacity))
    }

    /// Create a new buffer with the given size, filled with zeros
    pub fn new_with_size(size: usize) -> Self {
        Self(alloc::vec![0; size])
    }

    /// Get the length of the buffer
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Push a byte to the buffer
    pub fn push(&mut self, byte: u8) {
        self.0.push(byte);
    }

    /// Pop a byte from the buffer
    pub fn pop(&mut self) -> Option<u8> {
        self.0.pop()
    }

    /// Get a reference to a byte at the given index
    pub fn get(&self, index: usize) -> Option<&u8> {
        self.0.get(index)
    }

    /// Get a mutable reference to a byte at the given index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut u8> {
        self.0.get_mut(index)
    }

    /// Remove a byte at the given index
    pub fn remove(&mut self, index: usize) -> u8 {
        self.0.remove(index)
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&[u8]> for Buffer {
    fn from(slice: &[u8]) -> Self {
        Self(slice.to_vec())
    }
}

impl From<alloc::vec::Vec<u8>> for Buffer {
    fn from(vec: alloc::vec::Vec<u8>) -> Self {
        Self(vec)
    }
}

impl Into<alloc::vec::Vec<u8>> for Buffer {
    fn into(self) -> alloc::vec::Vec<u8> {
        self.0
    }
}

impl IntoAny for Buffer {
    fn into_any(self) -> Any {
        Any::from(self)
    }
}

impl FromAny for Buffer {
    fn from_any(any: Any) -> Option<Self> {
        any.cast::<Self>().cloned()
    }
}
