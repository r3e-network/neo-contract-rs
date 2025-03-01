// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;

/// Iterator for storage values
pub struct Iter<T> {
    data: Vec<T>,
    index: usize,
}

impl<T> Iter<T> {
    /// Create a new iterator from a vector
    pub fn new(data: Vec<T>) -> Self {
        Iter { data, index: 0 }
    }

    /// Check if the iterator has a next element
    pub fn has_next(&self) -> bool {
        self.index < self.data.len()
    }

    /// Get the next element from the iterator
    pub fn next(&mut self) -> Option<&T> {
        if self.has_next() {
            let result = &self.data[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}
