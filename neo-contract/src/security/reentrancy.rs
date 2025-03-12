// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Reentrancy guard implementation for Neo smart contracts
//! This module provides protection against reentrancy attacks

// use crate::storage::{StorageMap, Storable, StorageKey};
// use crate::builtin::ByteString;
use crate::prelude::ByteString;
use crate::error::{Error, ErrorCode, Result};


/// Reentrancy status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReentrancyStatus {
    /// Not entered
    NotEntered,
    /// Entered
    Entered,
}

/// Reentrancy guard to protect against reentrancy attacks
pub struct ReentrancyGuard {
    /// The key used to store the status
    #[allow(dead_code)]
    key: ByteString,
    /// Status flag (in-memory only for this simplified implementation)
    status: ReentrancyStatus,
}

impl ReentrancyGuard {
    /// Create a new reentrancy guard with a unique identifier
    pub fn new(identifier: &str) -> Self {
        let key = ByteString::from(identifier);
        Self { 
            key,
            status: ReentrancyStatus::NotEntered
        }
    }
    
    /// Enter the guarded section, returns an error if already entered
    pub fn enter(&mut self) -> Result<()> {
        if self.status == ReentrancyStatus::Entered {
            return Err(Error::with_message(ErrorCode::InvalidState, "Reentrant call detected"));
        }
        
        self.status = ReentrancyStatus::Entered;
        Ok(())
    }
    
    /// Exit the guarded section
    pub fn exit(&mut self) {
        self.status = ReentrancyStatus::NotEntered;
    }
    
    /// Check if the guard is currently entered
    pub fn is_entered(&self) -> bool {
        self.status == ReentrancyStatus::Entered
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
