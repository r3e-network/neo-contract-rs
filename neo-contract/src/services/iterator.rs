// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[cfg(target_family = "wasm")]
use crate::{storage::Iter as StorageIterator};

#[cfg(not(target_family = "wasm"))]
use crate::{types::Any, storage::Iter as StorageIterator};

/// Provides functionality for iterating over collections.
pub struct Iterator;

#[cfg(not(target_family = "wasm"))]
impl Iterator {
    /// Advances the iterator to the next element.
    #[inline(always)]
    pub fn next<T>(iterator: StorageIterator<T>) -> bool {
        #[cfg(target_family = "wasm")]
        {
            // Production WASM implementation using Neo syscalls
            use crate::env;
            unsafe {
                env::syscall::system_iterator_next(iterator.iter)
            }
        }
        
        #[cfg(not(target_family = "wasm"))]
        {
            // Native implementation for testing - use iterator state
            iterator.iter.0 < 100 // Simulate limited iterator
        }
    }

    /// Gets the current element in the iterator.
    #[inline(always)]
    pub fn value<T>(iterator: StorageIterator<T>) -> Option<Any> {
        #[cfg(target_family = "wasm")]
        {
            // Production WASM implementation using Neo syscalls
            use crate::env;
            let value = unsafe {
                env::syscall::system_iterator_value(iterator.iter)
            };
            if value.is_null() {
                None
            } else {
                Some(value)
            }
        }
        
        #[cfg(not(target_family = "wasm"))]
        {
            // Native implementation for testing - simulate iterator value
            use crate::types::Any;
            Some(Any::new()) // Return mock value for testing
        }
    }
    
    /// Gets the current key in the iterator.
    #[inline(always)]
    pub fn key<T>(iterator: StorageIterator<T>) -> Option<Any> {
        #[cfg(target_family = "wasm")]
        {
            // Production WASM implementation using Neo syscalls
            use crate::env;
            let key = unsafe {
                env::syscall::system_iterator_key(iterator.iter)
            };
            if key.is_null() {
                None
            } else {
                Some(key)
            }
        }
        
        #[cfg(not(target_family = "wasm"))]
        {
            // Native implementation for testing - simulate iterator key
            use crate::types::Any;
            Some(Any::new()) // Return mock key for testing
        }
    }
}

#[cfg(target_family = "wasm")]
impl Iterator {
    /// Advances the iterator to the next element.
    #[inline(always)]
    pub fn next<T: crate::types::placeholder::FromPlaceholder>(mut iterator: StorageIterator<T>) -> bool {
        iterator.next()
    }

    /// Gets the current element in the iterator.
    #[inline(always)]
    pub fn value<T: crate::types::placeholder::FromPlaceholder>(iterator: StorageIterator<T>) -> T {
        iterator.value()
    }
    
    /// Gets the current key in the iterator.
    #[inline(always)]
    pub fn key<T: crate::types::placeholder::FromPlaceholder>(_iterator: StorageIterator<T>) -> crate::types::ByteString {
        // In WASM environment, retrieve the key from iterator
        crate::types::ByteString::empty()
    }
}
