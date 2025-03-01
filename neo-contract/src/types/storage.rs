// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::types::builtin::string::ByteString;
use crate::types::context::StorageContext;

/// Storage represents a storage item
#[derive(Debug, Clone)]
pub struct Storage {
    context: StorageContext,
    key: ByteString,
    value: ByteString,
}

impl Storage {
    /// Create a new storage item
    pub fn new() -> Self {
        Self {
            context: StorageContext::new(),
            key: ByteString::empty(),
            value: ByteString::empty(),
        }
    }

    /// Get the context
    pub fn context(&self) -> StorageContext {
        self.context.clone()
    }

    /// Get the key
    pub fn key(&self) -> ByteString {
        self.key.clone()
    }

    /// Get the value
    pub fn value(&self) -> ByteString {
        self.value.clone()
    }
}
