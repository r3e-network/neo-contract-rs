// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Error handling for Neo smart contracts
//! This module provides standardized error types and helper functions
//! for handling errors in Neo smart contracts.

use alloc::string::{String, ToString};
use core::fmt;

/// Standard error codes for Neo smart contracts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ErrorCode {
    /// No error
    None = 0,
    
    /// Invalid argument
    InvalidArgument = 1,
    
    /// Unauthorized operation
    Unauthorized = 2,
    
    /// Insufficient funds
    InsufficientFunds = 3,
    
    /// Storage error
    StorageError = 4,
    
    /// Contract call error
    ContractCallError = 5,
    
    /// Invalid state
    InvalidState = 6,
    
    /// Overflow
    Overflow = 7,
    
    /// No signature
    NoSignature = 8,
    
    /// Reentrancy
    Reentrancy = 9,
    
    /// Custom error
    Custom = 10000,
}

/// Result type for Neo contract operations
pub type Result<T> = core::result::Result<T, Error>;

/// Error type for Neo contract operations
#[derive(Debug, Clone)]
pub struct Error {
    /// Error code
    pub code: ErrorCode,
    
    /// Error message
    pub message: String,
}

impl Error {
    /// Create a new error
    pub fn new(code: ErrorCode, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
    
    /// Create a standard unauthorized error
    pub fn unauthorized() -> Self {
        Self::new(ErrorCode::Unauthorized, "Unauthorized operation")
    }
    
    /// Create a standard insufficient funds error
    pub fn insufficient_funds() -> Self {
        Self::new(ErrorCode::InsufficientFunds, "Insufficient funds")
    }
    
    /// Create a standard invalid argument error
    pub fn invalid_argument(msg: &str) -> Self {
        Self::new(ErrorCode::InvalidArgument, msg)
    }
    
    /// Create a storage error
    pub fn storage_error() -> Self {
        Self::new(ErrorCode::StorageError, "Storage operation failed")
    }
    
    /// Create a contract call error
    pub fn contract_call_error(msg: &str) -> Self {
        Self::new(ErrorCode::ContractCallError, msg)
    }
    
    /// Create a custom error
    pub fn custom(msg: &str) -> Self {
        Self::new(ErrorCode::Custom, msg)
    }
    
    /// Assert a condition or return an error
    pub fn assert(condition: bool, code: ErrorCode, message: &str) -> Result<()> {
        if condition {
            Ok(())
        } else {
            Err(Self::new(code, message))
        }
    }
    
    /// Check witness or return unauthorized error
    pub fn check_witness(address: &crate::builtin::H160) -> Result<()> {
        if crate::Runtime::check_witness(address.clone()) {
            Ok(())
        } else {
            Err(Self::unauthorized())
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}: {}", self.code as u32, self.message)
    }
}

// Helper macros for error handling
#[macro_export]
macro_rules! ensure {
    ($condition:expr, $error:expr) => {
        if !($condition) {
            return Err($error);
        }
    };
    
    ($condition:expr, $error_code:expr, $message:expr) => {
        if !($condition) {
            return Err($crate::error::Error::new($error_code, $message));
        }
    };
}

#[macro_export]
macro_rules! require_witness {
    ($address:expr) => {
        $crate::error::Error::check_witness(&$address)?;
    };
}

/// Revert execution with an error
pub fn revert(code: ErrorCode, message: &str) -> ! {
    // Log the error
    let error = Error::new(code, message);
    // In a real contract, this would revert the execution
    panic!("{}", error);
}
