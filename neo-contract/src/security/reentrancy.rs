// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Reentrancy guard implementation for Neo smart contracts
//! This module provides protection against reentrancy attacks

use crate::storage::{StorageMap, Storable, StorageKey};
use crate::builtin::ByteString;
use crate::error::{Error, ErrorCode, Result};
use core::marker::PhantomData;

/// Reentrancy status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReentrancyStatus {
    /// Not entered
    NotEntered = 0,
    /// Entered
    Entered = 1,
}

impl Storable for ReentrancyStatus {
    fn to_bytes(&self) -> alloc::vec::Vec<u8> {
        alloc::vec![*self as u8]
    }
    
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes.get(0) {
            Some(0) => Some(ReentrancyStatus::NotEntered),
            Some(1) => Some(ReentrancyStatus::Entered),
            _ => None,
        }
    }
}

/// Reentrancy guard to protect against reentrancy attacks
pub struct ReentrancyGuard {
    /// Storage map to track reentrancy status
    status_map: StorageMap<ByteString, ReentrancyStatus>,
    /// Unique key for this guard
    key: ByteString,
}

impl ReentrancyGuard {
    /// Create a new reentrancy guard with a unique identifier
    pub fn new(identifier: &str) -> Self {
        let key = ByteString::from(identifier);
        let status_map = StorageMap::new(b"reentrancy_guard");
        
        // Initialize the status as NotEntered if it doesn't exist
        if !status_map.contains_key(&key) {
            status_map.put(&key, &ReentrancyStatus::NotEntered);
        }
        
        Self { status_map, key }
    }
    
    /// Enter the guarded section, returns an error if already entered
    pub fn enter(&self) -> Result<()> {
        match self.status_map.get(&self.key) {
            Some(ReentrancyStatus::Entered) => {
                Err(Error::new(ErrorCode::Reentrancy, "Reentrant call detected"))
            },
            _ => {
                self.status_map.put(&self.key, &ReentrancyStatus::Entered);
                Ok(())
            }
        }
    }
    
    /// Exit the guarded section
    pub fn exit(&self) {
        self.status_map.put(&self.key, &ReentrancyStatus::NotEntered);
    }
    
    /// Run a function with reentrancy protection
    pub fn with_guard<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> T,
    {
        self.enter()?;
        
        // Safe unwinding even if the function panics
        struct Guard<'a> {
            guard: &'a ReentrancyGuard,
        }
        
        impl<'a> Drop for Guard<'a> {
            fn drop(&mut self) {
                self.guard.exit();
            }
        }
        
        let guard = Guard { guard: self };
        let result = f();
        
        // Explicitly drop the guard
        core::mem::drop(guard);
        
        Ok(result)
    }
    
    /// Check if the guarded section is currently entered
    pub fn is_entered(&self) -> bool {
        matches!(self.status_map.get(&self.key), Some(ReentrancyStatus::Entered))
    }
}

/// A macro for creating a reentrancy guard with a default identifier
#[macro_export]
macro_rules! reentrancy_guard {
    () => {
        $crate::security::ReentrancyGuard::new("default")
    };
    
    ($identifier:expr) => {
        $crate::security::ReentrancyGuard::new($identifier)
    };
}

/// A macro for creating a function with reentrancy protection
#[macro_export]
macro_rules! no_reentrant_method {
    (fn $name:ident ($($arg:ident : $type:ty),*) -> $ret:ty $body:block) => {
        fn $name($($arg : $type),*) -> $ret {
            static mut GUARD: Option<$crate::security::ReentrancyGuard> = None;
            
            if unsafe { GUARD.is_none() } {
                unsafe {
                    GUARD = Some($crate::security::ReentrancyGuard::new(stringify!($name)));
                }
            }
            
            let guard = unsafe { GUARD.as_ref().unwrap() };
            guard.with_guard(|| $body).unwrap()
        }
    };
}
