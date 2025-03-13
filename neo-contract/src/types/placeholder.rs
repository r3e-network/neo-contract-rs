// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use core::fmt;

/// Placeholder represents a placeholder for a value
#[derive(Debug, Clone)]
pub struct Placeholder {
    data: Vec<u8>,
}

impl Placeholder {
    /// Create a new placeholder
    pub fn new() -> Self { Self { data: Vec::new() } }

    /// Get the data
    pub fn data(&self) -> &[u8] { &self.data }
}

impl fmt::Display for Placeholder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "Placeholder") }
}
