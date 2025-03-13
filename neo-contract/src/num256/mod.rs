// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use core::fmt;

/// Int256 is a 256-bit integer
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Int256(pub i128);

impl Int256 {
    /// Create a new Int256 from an i128
    pub fn new(value: i128) -> Self { Int256(value) }

    /// Get the underlying i128 value
    pub fn value(&self) -> i128 { self.0 }
}

impl fmt::Display for Int256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}
