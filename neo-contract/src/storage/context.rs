// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use core::fmt;
use crate::types::context::StorageContext;

/// Storage context wrapper
#[derive(Debug, Clone)]
pub struct Context {
    /// Inner storage context
    pub inner: StorageContext,
}

impl Context {
    /// Create a new storage context
    pub fn new() -> Self {
        Self {
            inner: unsafe { crate::env::syscall_non_wasm::system_storage_get_context() },
        }
    }

    /// Convert to a read-only storage context
    pub fn as_readonly(&self) -> Self {
        Self {
            inner: unsafe { crate::env::syscall_non_wasm::system_storage_as_readonly(self.inner.clone()) },
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

/// Read-only storage context wrapper
#[derive(Debug, Clone)]
pub struct ReadOnlyContext {
    /// Inner storage context
    pub inner: StorageContext,
}

impl ReadOnlyContext {
    /// Create a new read-only storage context
    pub fn new() -> Self {
        Self {
            inner: unsafe { crate::env::syscall_non_wasm::system_storage_get_read_only_context() },
        }
    }
}

impl Default for ReadOnlyContext {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Context({})", self.inner)
    }
}

impl fmt::Display for ReadOnlyContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ReadOnlyContext({})", self.inner)
    }
}
