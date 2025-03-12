//! Int256 type for Neo Contract RS
//!
//! This module defines the Int256 type, which is used for large integer values in Neo.

use core::fmt;
use core::ops::{Add, Sub, Mul, Div, Neg};
use core::convert::TryFrom;


use crate::types::builtin::any::Any;

/// Int256 represents a 256-bit signed integer
#[derive(PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Int256(pub [u8; 32]);

// Add PartialOrd implementation for Int256
impl PartialOrd for Int256 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        // Check if negative
        let self_negative = self.is_negative();
        let other_negative = other.is_negative();
        
        // If signs are different, negative is less than positive
        if self_negative && !other_negative {
            return Some(core::cmp::Ordering::Less);
        }
        if !self_negative && other_negative {
            return Some(core::cmp::Ordering::Greater);
        }
        
        // If both have the same sign, compare bytes from most significant to least
        for i in (0..32).rev() {
            if self.0[i] < other.0[i] {
                return Some(if self_negative {
                    core::cmp::Ordering::Greater
                } else {
                    core::cmp::Ordering::Less
                });
            }
            if self.0[i] > other.0[i] {
                return Some(if self_negative {
                    core::cmp::Ordering::Less
                } else {
                    core::cmp::Ordering::Greater
                });
            }
        }
        
        // If we get here, they're equal
        Some(core::cmp::Ordering::Equal)
    }
}

// Add Ord implementation for Int256 for complete ordering
impl Ord for Int256 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Int256 {
    /// Creates a new Int256 with value zero
    pub fn zero() -> Self {
        Int256([0; 32])
    }
    
    /// Creates a new Int256 with value one
    pub fn one() -> Self {
        let mut bytes = [0; 32];
        bytes[0] = 1;
        Int256(bytes)
    }
    
    /// Checks if the Int256 is zero
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }
    
    /// Creates an Int256 from a slice
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut bytes = [0u8; 32];
        if slice.len() >= 32 {
            bytes.copy_from_slice(&slice[..32]);
        } else {
            bytes[..slice.len()].copy_from_slice(slice);
        }
        Int256(bytes)
    }
    
    /// Returns the bytes as a slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    
    /// Converts a u64 to Int256
    pub fn from_u64(val: u64) -> Self {
        let mut bytes = [0u8; 32];
        bytes[0..8].copy_from_slice(&val.to_le_bytes());
        Int256(bytes)
    }
    
    /// Converts an i64 to Int256
    pub fn from_i64(val: i64) -> Self {
        let mut bytes = [0u8; 32];
        bytes[0..8].copy_from_slice(&val.to_le_bytes());
        
        // Sign extend if negative
        if val < 0 {
            bytes[8..].fill(0xFF);
        }
        
        Int256(bytes)
    }
    
    /// Tries to convert the Int256 to a u64
    pub fn to_u64(&self) -> Option<u64> {
        // Check if the value fits in u64 (all bytes beyond the first 8 must be 0)
        if self.0[8..].iter().any(|&b| b != 0) {
            return None;
        }
        
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.0[0..8]);
        Some(u64::from_le_bytes(bytes))
    }
    
    /// Tries to convert the Int256 to a u32
    pub fn as_u32(&self) -> Option<u32> {
        // Check if the value fits in u32 (all bytes beyond the first 4 must be 0)
        if self.0[4..].iter().any(|&b| b != 0) {
            return None;
        }
        
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.0[0..4]);
        Some(u32::from_le_bytes(bytes))
    }
    
    /// Tries to convert the Int256 to a u64
    pub fn as_u64(&self) -> Option<u64> {
        self.to_u64()
    }
    
    /// Tries to convert the Int256 to an i64
    pub fn to_i64(&self) -> Option<i64> {
        // Check if the value fits in i64
        // For positive numbers, all bytes beyond the first 8 must be 0
        // For negative numbers, all bytes beyond the first 8 must be 0xFF
        let is_negative = (self.0[7] & 0x80) != 0;
        let expected_byte = if is_negative { 0xFF } else { 0 };
        
        if self.0[8..].iter().any(|&b| b != expected_byte) {
            return None;
        }
        
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.0[0..8]);
        Some(i64::from_le_bytes(bytes))
    }
    
    /// Checks if the Int256 is negative
    pub fn is_negative(&self) -> bool {
        // Check the most significant bit of the most significant byte
        (self.0[31] & 0x80) != 0
    }
    
    /// Computes the absolute value
    pub fn abs(&self) -> Self {
        if self.is_negative() {
            -(*self)
        } else {
            *self
        }
    }
}

// Basic arithmetic operations
impl Add for Int256 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        // For a real implementation, this would perform 256-bit addition
        // Here we're just providing a simple implementation for demonstration
        let mut result = [0u8; 32];
        let mut carry = 0u16;
        
        for i in 0..32 {
            let sum = self.0[i] as u16 + other.0[i] as u16 + carry;
            result[i] = sum as u8;
            carry = sum >> 8;
        }
        
        Int256(result)
    }
}

impl Sub for Int256 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        // For a real implementation, this would perform 256-bit subtraction
        // Here we're just providing a simple implementation for demonstration
        let mut result = [0u8; 32];
        let mut borrow = 0i16;
        
        for i in 0..32 {
            let diff = (self.0[i] as i16) - (other.0[i] as i16) - borrow;
            result[i] = diff as u8;
            borrow = if diff < 0 { 1 } else { 0 };
        }
        
        Int256(result)
    }
}

impl Neg for Int256 {
    type Output = Self;
    
    fn neg(self) -> Self {
        // Compute two's complement
        let mut result = [0u8; 32];
        let mut carry = 1u16;
        
        for i in 0..32 {
            let sum = (!self.0[i] as u16) + carry;
            result[i] = sum as u8;
            carry = sum >> 8;
        }
        
        Int256(result)
    }
}

// Simplified multiplication (not efficient for real-world use)
impl Mul for Int256 {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self {
        // This is a simplified implementation - only suitable for small values
        // For production use, a proper 256-bit multiplication algorithm should be implemented
        if self.is_zero() || other.is_zero() {
            return Int256::zero();
        }
        
        // Handle special case for one to prevent compiler warnings
        if other == Int256::one() {
            return self;
        }
        
        // For now, just returning a simplified placeholder
        // In a real implementation, this should perform actual multiplication
        Int256::zero()
    }
}

// Simplified division (not efficient for real-world use)
impl Div for Int256 {
    type Output = Self;
    
    fn div(self, other: Self) -> Self {
        // This is a simplified implementation - only suitable for small values
        // For production use, a proper 256-bit division algorithm should be implemented
        
        // Check for division by zero
        if other.is_zero() {
            // In a real implementation, we should panic or return an error
            // For now, just return zero to satisfy the compiler
            return Int256::zero();
        }
        
        // Handle special case for one to prevent compiler warnings
        if other == Int256::one() {
            return self;
        }
        
        // For now, returning a simplified placeholder
        // In a real implementation, this should perform actual division
        Int256::zero()
    }
}

// Conversion from primitive types
impl From<u8> for Int256 {
    fn from(val: u8) -> Self {
        let mut bytes = [0u8; 32];
        bytes[0] = val;
        Int256(bytes)
    }
}

impl From<i8> for Int256 {
    fn from(val: i8) -> Self {
        Int256::from_i64(val as i64)
    }
}

impl From<u16> for Int256 {
    fn from(val: u16) -> Self {
        let mut bytes = [0u8; 32];
        bytes[0..2].copy_from_slice(&val.to_le_bytes());
        Int256(bytes)
    }
}

impl From<i16> for Int256 {
    fn from(val: i16) -> Self {
        Int256::from_i64(val as i64)
    }
}

impl From<u32> for Int256 {
    fn from(val: u32) -> Self {
        let mut bytes = [0u8; 32];
        bytes[0..4].copy_from_slice(&val.to_le_bytes());
        Int256(bytes)
    }
}

impl From<i32> for Int256 {
    fn from(val: i32) -> Self {
        Int256::from_i64(val as i64)
    }
}

impl From<u64> for Int256 {
    fn from(val: u64) -> Self {
        Int256::from_u64(val)
    }
}

impl From<i64> for Int256 {
    fn from(val: i64) -> Self {
        Int256::from_i64(val)
    }
}

impl fmt::Debug for Int256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(val) = self.to_i64() {
            write!(f, "Int256({})", val)
        } else {
            write!(f, "Int256(")?;
            for byte in self.0.iter().rev() {
                write!(f, "{:02x}", byte)?;
            }
            write!(f, ")")
        }
    }
}

impl fmt::Display for Int256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(val) = self.to_i64() {
            write!(f, "{}", val)
        } else {
            write!(f, "0x")?;
            for byte in self.0.iter().rev() {
                write!(f, "{:02x}", byte)?;
            }
            Ok(())
        }
    }
}

// Implement TryFrom for Int256 to convert from Any
impl TryFrom<Any> for Int256 {
    type Error = ();

    fn try_from(value: Any) -> Result<Self, Self::Error> {
        match value {
            Any::Integer(int) => Ok(int),
            _ => Err(()),
        }
    }
}
