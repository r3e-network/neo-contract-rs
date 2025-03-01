// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

/// StorageContext represents a storage context
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageContext {
    /// The ID of the storage context
    pub id: u32,
}

/// ReadOnlyStorageContext represents a read-only storage context
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadOnlyStorageContext {
    /// The ID of the read-only storage context
    pub id: u32,
}

impl StorageContext {
    /// Create a new storage context
    pub fn new() -> Self {
        Self { id: 0 }
    }

    /// Get the current storage context
    pub fn current() -> Self {
        Self::get_context()
    }

    /// Get the storage context
    pub fn get_context() -> Self {
        unsafe { crate::env::syscall::system_storage_get_context() }
    }

    /// Convert to a read-only storage context
    pub fn as_read_only(&self) -> ReadOnlyStorageContext {
        unsafe { crate::env::syscall::system_storage_as_readonly(self.clone()) }
    }
}

impl ReadOnlyStorageContext {
    /// Create a new read-only storage context
    pub fn new() -> Self {
        Self { id: 0 }
    }

    /// Get the current read-only storage context
    pub fn current() -> Self {
        Self::get_context()
    }

    /// Get the read-only storage context
    pub fn get_context() -> Self {
        unsafe { crate::env::syscall::system_storage_get_read_only_context() }
    }
}
