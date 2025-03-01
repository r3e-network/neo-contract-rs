// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::format;
use core::fmt;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use crate::types::builtin::string::ByteString;

/// Int256 represents a 256-bit integer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Int256(pub i128);

impl Int256 {
    /// Create a new Int256
    pub fn new(value: i128) -> Self {
        Int256(value)
    }

    /// Create a zero Int256
    pub fn zero() -> Self {
        Int256(0)
    }

    /// Check if the Int256 is zero
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Get the value as i128
    pub fn value(&self) -> i128 {
        self.0
    }

    /// Convert to string
    pub fn to_string(&self) -> String {
        format!("{}", self.0)
    }
    
    /// Convert to i64
    pub fn to_i64(&self) -> i64 {
        self.0 as i64
    }
}

impl Add for Int256 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Int256(self.0 + other.0)
    }
}

impl Sub for Int256 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Int256(self.0 - other.0)
    }
}

impl Mul for Int256 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Int256(self.0 * other.0)
    }
}

impl Div for Int256 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Int256(self.0 / other.0)
    }
}

impl Rem for Int256 {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Int256(self.0 % other.0)
    }
}

impl Neg for Int256 {
    type Output = Self;

    fn neg(self) -> Self {
        Int256(-self.0)
    }
}

impl From<i128> for Int256 {
    fn from(value: i128) -> Self {
        Int256(value)
    }
}

impl From<i64> for Int256 {
    fn from(value: i64) -> Self {
        Int256(value as i128)
    }
}

impl From<i32> for Int256 {
    fn from(value: i32) -> Self {
        Int256(value as i128)
    }
}

impl From<Int256> for i128 {
    fn from(value: Int256) -> Self {
        value.0
    }
}

impl From<ByteString> for Int256 {
    fn from(value: ByteString) -> Self {
        // Try to parse the ByteString as a string first
        if let Ok(s) = String::from_utf8(value.0.clone()) {
            if let Ok(i) = s.parse::<i128>() {
                return Int256(i);
            }
        }
        
        // Default to zero if parsing fails
        Int256::zero()
    }
}

impl From<Int256> for ByteString {
    fn from(value: Int256) -> Self {
        ByteString::from(value.to_string().as_str())
    }
}

impl fmt::Display for Int256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
