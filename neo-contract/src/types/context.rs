// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::types::builtin::h160::H160;

/// StorageContext represents a storage context
#[derive(Debug, Clone)]
pub struct StorageContext {
    id: u32,
    read_only: bool,
}

/// ReadOnlyStorageContext represents a read-only storage context
pub type ReadOnlyStorageContext = StorageContext;

/// FindOptions represents options for finding storage items
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindOptions {
    /// None
    None = 0,
    /// KeysOnly
    KeysOnly = 1,
    /// RemovePrefix
    RemovePrefix = 2,
    /// ValuesOnly
    ValuesOnly = 4,
    /// DeserializeValues
    DeserializeValues = 8,
    /// PickField0
    PickField0 = 16,
    /// PickField1
    PickField1 = 32,
}

impl StorageContext {
    /// Create a new storage context
    pub fn new() -> Self {
        Self {
            id: 0,
            read_only: false,
        }
    }

    /// Get the storage context
    pub fn get_context() -> Self {
        unsafe { crate::env::syscall_non_wasm::system_storage_get_context() }
    }

    /// Convert to a read-only storage context
    pub fn as_read_only(&self) -> Self {
        unsafe { crate::env::syscall_non_wasm::system_storage_as_readonly(self.clone()) }
    }

    /// Get the id
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Check if the context is read-only
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// Get a read-only storage context
    pub fn get_read_only_context() -> Self {
        unsafe { crate::env::syscall_non_wasm::system_storage_get_read_only_context() }
    }
}
