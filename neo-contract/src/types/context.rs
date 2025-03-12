// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use core::fmt;


/// Storage context
#[derive(Debug, Clone)]
pub struct StorageContext {
    /// ID
    pub id: i32,
    /// Read only
    pub read_only: bool,
}

impl StorageContext {
    /// Create a new storage context
    pub fn new() -> Self {
        Self {
            id: 0,
            read_only: false,
        }
    }

    /// Create a read-only storage context
    pub fn new_readonly() -> Self {
        Self {
            id: 0,
            read_only: true,
        }
    }

    /// Convert the storage context to a byte array
    pub fn as_bytes(&self) -> alloc::vec::Vec<u8> {
        let mut bytes = alloc::vec::Vec::with_capacity(8);
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.push(if self.read_only { 1 } else { 0 });
        bytes
    }
    
    /// Get the length of the byte representation
    pub fn len(&self) -> usize {
        // 4 bytes for id + 1 byte for read_only flag
        5
    }
    
    /// Get the pointer to the raw bytes
    pub fn as_ptr(&self) -> *const u8 {
        // This is a placeholder implementation
        // In a real implementation, this would return a pointer to the raw bytes
        // For now, we'll use the id's pointer
        &self.id as *const i32 as *const u8
    }
    
    /// Get the current storage context
    pub fn current() -> Self {
        Self {
            id: 0,
            read_only: false,
        }
    }
}

impl Default for StorageContext {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StorageContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StorageContext(id: {}, read_only: {})", self.id, self.read_only)
    }
}
