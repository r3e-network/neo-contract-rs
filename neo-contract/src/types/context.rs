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
