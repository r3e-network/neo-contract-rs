// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use core::marker::PhantomData;
use crate::builtin::ByteString;
use crate::types::context::StorageContext;

/// Iterator for basic storage values
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

/// Native storage iterator that uses Neo's built-in storage iterator
pub struct StorageIterator {
    /// Iterator ID returned by storage.find
    id: i32,
}

impl StorageIterator {
    /// Create a new storage iterator for the given prefix
    pub fn new(context: StorageContext, prefix: ByteString) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let id = unsafe { crate::env::syscall_non_wasm::system_storage_find(context, prefix) };
        
        #[cfg(target_arch = "wasm32")]
        let id = unsafe { crate::env::syscall::system_storage_find(context, prefix) };
        
        Self { id }
    }
    
    /// Check if the iterator has a next element
    pub fn has_next(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        let result = unsafe { 
            crate::env::syscall_non_wasm::system_iterator_next(self.id)
        };
        
        #[cfg(target_arch = "wasm32")]
        let result = unsafe { 
            crate::env::syscall::system_iterator_next(self.id)
        };
        
        result
    }
    
    /// Get the next key from the iterator
    pub fn next_key(&self) -> Option<ByteString> {
        if !self.has_next() {
            return None;
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        let key = unsafe { 
            crate::env::syscall_non_wasm::system_iterator_key(self.id)
        };
        
        #[cfg(target_arch = "wasm32")]
        let key = unsafe { 
            crate::env::syscall::system_iterator_key(self.id)
        };
        
        Some(key)
    }
    
    /// Get the next value from the iterator
    pub fn next_value(&self) -> Option<ByteString> {
        if !self.has_next() {
            return None;
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        let value = unsafe { 
            crate::env::syscall_non_wasm::system_iterator_value(self.id)
        };
        
        #[cfg(target_arch = "wasm32")]
        let value = unsafe { 
            crate::env::syscall::system_iterator_value(self.id)
        };
        
        Some(value)
    }
    
    /// Get the next key-value pair from the iterator
    pub fn next(&self) -> Option<(ByteString, ByteString)> {
        if !self.has_next() {
            return None;
        }
        
        let key = self.next_key()?;
        let value = self.next_value()?;
        
        Some((key, value))
    }
}

impl Drop for StorageIterator {
    fn drop(&mut self) {
        // Clean up the iterator when it goes out of scope
        if self.id >= 0 {
            #[cfg(not(target_arch = "wasm32"))]
            unsafe { 
                crate::env::syscall_non_wasm::system_iterator_value(self.id);
            }
            
            #[cfg(target_arch = "wasm32")]
            unsafe { 
                crate::env::syscall::system_iterator_value(self.id);
            }
        }
    }
}

/// Typed storage iterator that converts raw storage values to specific types
pub struct TypedStorageIterator<K, V> {
    /// The underlying storage iterator
    iterator: StorageIterator,
    /// Marker for the key type
    _key_type: PhantomData<K>,
    /// Marker for the value type
    _value_type: PhantomData<V>,
}

impl<K, V> TypedStorageIterator<K, V> 
where
    K: TryFrom<ByteString>,
    V: TryFrom<ByteString>,
{
    /// Create a new typed storage iterator
    pub fn new(context: StorageContext, prefix: ByteString) -> Self {
        Self {
            iterator: StorageIterator::new(context, prefix),
            _key_type: PhantomData,
            _value_type: PhantomData,
        }
    }
    
    /// Check if the iterator has a next element
    pub fn has_next(&self) -> bool {
        self.iterator.has_next()
    }
    
    /// Get the next element from the iterator
    pub fn next(&self) -> Option<(K, V)> {
        let (key_bs, value_bs) = self.iterator.next()?;
        
        let key = K::try_from(key_bs).ok()?;
        let value = V::try_from(value_bs).ok()?;
        
        Some((key, value))
    }
}
