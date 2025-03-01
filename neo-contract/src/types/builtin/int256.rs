// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::format;
use alloc::vec::Vec;
use alloc::string::ToString;
use crate::types::builtin::string::ByteString;
use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Int256 represents a 256-bit integer
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
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

    /// Get the value
    pub fn value(&self) -> i128 {
        self.0
    }
    
    /// Convert to u8
    pub fn to_u8(&self) -> u8 {
        self.0 as u8
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

impl From<u64> for Int256 {
    fn from(value: u64) -> Self {
        Int256(value as i128)
    }
}

impl From<u32> for Int256 {
    fn from(value: u32) -> Self {
        Int256(value as i128)
    }
}

impl From<Int256> for i128 {
    fn from(value: Int256) -> Self {
        value.0
    }
}

impl TryFrom<ByteString> for Int256 {
    type Error = ();

    fn try_from(value: ByteString) -> Result<Self, Self::Error> {
        let s = String::from_utf8_lossy(&value.0);
        match s.parse::<i128>() {
            Ok(val) => Ok(Int256(val)),
            Err(_) => Err(()),
        }
    }
}

impl From<Int256> for ByteString {
    fn from(value: Int256) -> Self {
        ByteString::from(value.0.to_string())
    }
}

impl Add for Int256 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Int256(self.0 + other.0)
    }
}

impl AddAssign for Int256 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for Int256 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Int256(self.0 - other.0)
    }
}

impl SubAssign for Int256 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl Mul for Int256 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Int256(self.0 * other.0)
    }
}

impl MulAssign for Int256 {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}

impl Div for Int256 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Int256(self.0 / other.0)
    }
}

impl DivAssign for Int256 {
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
    }
}

impl Neg for Int256 {
    type Output = Self;

    fn neg(self) -> Self {
        Int256(-self.0)
    }
}

impl fmt::Display for Int256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
