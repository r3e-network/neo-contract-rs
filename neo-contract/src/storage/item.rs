//! Storage Item for Neo Contract RS
//!
//! This module provides a high-level API for contract storage.

use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;
use crate::error::{Error, ErrorCode, Result};
use super::context::Context;

/// A trait for types that can be serialized and deserialized
pub trait Codec {
    /// Serializes the value into bytes
    fn encode(&self) -> Vec<u8>;
    
    /// Deserializes bytes into a value
    fn decode(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

/// A high-level storage item
pub struct Item<T> {
    /// The key used to store the value in storage
    key: Vec<u8>,
    
    /// The storage context
    context: Option<Context>,
    
    /// The type of value stored in this item
    _marker: PhantomData<T>,
}

impl<T> Item<T>
where
    T: Codec,
{
    /// Creates a new storage item with the given key
    pub fn new(key: impl AsRef<[u8]>) -> Self {
        Item {
            key: key.as_ref().to_vec(),
            context: None,
            _marker: PhantomData,
        }
    }
    
    /// Creates a new storage item with the given key and context
    pub fn with_context(key: impl AsRef<[u8]>, context: Context) -> Self {
        Item {
            key: key.as_ref().to_vec(),
            context: Some(context),
            _marker: PhantomData,
        }
    }
    
    /// Gets the value from storage
    pub fn get(&self) -> Result<Option<T>> {
        let value = if let Some(context) = &self.context {
            context.get(&self.key)
        } else {
            super::get(&self.key)
        };
        
        match value {
            Some(bytes) => Ok(Some(T::decode(&bytes)?)),
            None => Ok(None),
        }
    }
    
    /// Sets the value in storage
    pub fn set(&self, value: &T) -> Result<()> {
        let bytes = value.encode();
        
        if let Some(context) = &self.context {
            context.put(&self.key, &bytes);
        } else {
            super::put(&self.key, &bytes);
        }
        
        Ok(())
    }
    
    /// Removes the value from storage
    pub fn clear(&self) -> Result<()> {
        if let Some(context) = &self.context {
            context.delete(&self.key);
        } else {
            super::delete(&self.key);
        }
        
        Ok(())
    }
    
    /// Checks if the item exists in storage
    pub fn exists(&self) -> bool {
        if let Some(context) = &self.context {
            context.has(&self.key)
        } else {
            super::has(&self.key)
        }
    }
    
    /// Gets the key of this item
    pub fn key(&self) -> &[u8] {
        &self.key
    }
    
    /// Gets the context of this item
    pub fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }
    
    /// Sets the context for this item
    pub fn set_context(&mut self, context: Context) {
        self.context = Some(context);
    }
    
    /// Clears the context for this item
    pub fn clear_context(&mut self) {
        self.context = None;
    }
}

/// Implementations of Codec for common types

impl Codec for u8 {
    fn encode(&self) -> Vec<u8> {
        vec![*self]
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 1 {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        Ok(bytes[0])
    }
}

impl Codec for u16 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 2 {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        let mut buf = [0u8; 2];
        buf.copy_from_slice(bytes);
        Ok(u16::from_le_bytes(buf))
    }
}

impl Codec for u32 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 4 {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        let mut buf = [0u8; 4];
        buf.copy_from_slice(bytes);
        Ok(u32::from_le_bytes(buf))
    }
}

impl Codec for u64 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 8 {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        let mut buf = [0u8; 8];
        buf.copy_from_slice(bytes);
        Ok(u64::from_le_bytes(buf))
    }
}

impl Codec for i8 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 1 {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        let mut buf = [0u8; 1];
        buf.copy_from_slice(bytes);
        Ok(i8::from_le_bytes(buf))
    }
}

impl Codec for i16 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 2 {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        let mut buf = [0u8; 2];
        buf.copy_from_slice(bytes);
        Ok(i16::from_le_bytes(buf))
    }
}

impl Codec for i32 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 4 {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        let mut buf = [0u8; 4];
        buf.copy_from_slice(bytes);
        Ok(i32::from_le_bytes(buf))
    }
}

impl Codec for i64 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 8 {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        let mut buf = [0u8; 8];
        buf.copy_from_slice(bytes);
        Ok(i64::from_le_bytes(buf))
    }
}

impl Codec for bool {
    fn encode(&self) -> Vec<u8> {
        vec![if *self { 1 } else { 0 }]
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 1 {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        Ok(bytes[0] != 0)
    }
}

impl Codec for Vec<u8> {
    fn encode(&self) -> Vec<u8> {
        self.clone()
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        Ok(bytes.to_vec())
    }
}