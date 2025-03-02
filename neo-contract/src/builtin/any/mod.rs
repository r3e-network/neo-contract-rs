// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use alloc::vec;
use core::convert::TryFrom;

use crate::builtin::{ByteString, H160, H256, Int256, Array};

/// Any type that can hold any value
#[derive(Debug, Clone)]
pub struct Any {
    /// The value
    value: Vec<u8>,
}

impl Any {
    /// Create a new empty Any
    pub fn new() -> Self {
        Self { value: Vec::new() }
    }
    
    /// Get the value as bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.value
    }
}

impl Default for Any {
    fn default() -> Self {
        Self::new()
    }
}

impl From<i64> for Any {
    fn from(value: i64) -> Self {
        Self { value: value.to_le_bytes().to_vec() }
    }
}

impl From<u64> for Any {
    fn from(value: u64) -> Self {
        Self { value: value.to_le_bytes().to_vec() }
    }
}

impl From<i32> for Any {
    fn from(value: i32) -> Self {
        Self { value: value.to_le_bytes().to_vec() }
    }
}

impl From<u32> for Any {
    fn from(value: u32) -> Self {
        Self { value: value.to_le_bytes().to_vec() }
    }
}

impl From<u8> for Any {
    fn from(value: u8) -> Self {
        Self { value: vec![value] }
    }
}

impl From<bool> for Any {
    fn from(value: bool) -> Self {
        Self { value: vec![if value { 1 } else { 0 }] }
    }
}

impl From<ByteString> for Any {
    fn from(value: ByteString) -> Self {
        Self { value: value.to_vec() }
    }
}

impl From<H160> for Any {
    fn from(value: H160) -> Self {
        Self { value: value.to_vec() }
    }
}

impl From<H256> for Any {
    fn from(value: H256) -> Self {
        Self { value: value.to_vec() }
    }
}

impl From<Int256> for Any {
    fn from(value: Int256) -> Self {
        Self { value: value.to_vec() }
    }
}

impl<T> From<Array<T>> for Any {
    fn from(value: Array<T>) -> Self {
        Self { value: value.to_vec() }
    }
}

impl TryFrom<Any> for i64 {
    type Error = ();
    
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        if value.value.len() == 8 {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&value.value);
            Ok(i64::from_le_bytes(bytes))
        } else {
            Err(())
        }
    }
}

impl TryFrom<Any> for u64 {
    type Error = ();
    
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        if value.value.len() == 8 {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&value.value);
            Ok(u64::from_le_bytes(bytes))
        } else {
            Err(())
        }
    }
}

impl TryFrom<Any> for i32 {
    type Error = ();
    
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        if value.value.len() == 4 {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&value.value);
            Ok(i32::from_le_bytes(bytes))
        } else {
            Err(())
        }
    }
}

impl TryFrom<Any> for u32 {
    type Error = ();
    
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        if value.value.len() == 4 {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&value.value);
            Ok(u32::from_le_bytes(bytes))
        } else {
            Err(())
        }
    }
}

impl TryFrom<Any> for u8 {
    type Error = ();
    
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        if value.value.len() == 1 {
            Ok(value.value[0])
        } else {
            Err(())
        }
    }
}

impl TryFrom<Any> for bool {
    type Error = ();
    
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        if value.value.len() == 1 {
            match value.value[0] {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

impl TryFrom<Any> for ByteString {
    type Error = ();
    
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        Ok(ByteString::from(value.value))
    }
}

impl TryFrom<Any> for H160 {
    type Error = ();
    
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        if value.value.len() == 20 {
            let mut bytes = [0u8; 20];
            bytes.copy_from_slice(&value.value);
            Ok(H160::from(bytes))
        } else {
            Err(())
        }
    }
}

impl TryFrom<Any> for H256 {
    type Error = ();
    
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        if value.value.len() == 32 {
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(&value.value);
            Ok(H256::from(bytes))
        } else {
            Err(())
        }
    }
}

impl TryFrom<Any> for Int256 {
    type Error = ();
    
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        if value.value.len() <= 32 {
            Ok(Int256::from_bytes(&value.value))
        } else {
            Err(())
        }
    }
}

impl<T> TryFrom<Any> for Array<T> {
    type Error = ();
    
    fn try_from(value: Any) -> Result<Self, Self::Error> {
        Ok(Array::from_bytes(&value.value))
    }
}
