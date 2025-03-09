//! Error handling for Neo Contract RS
//!
//! This module provides error handling functionality for Neo smart contracts.

use alloc::string::String;
use core::fmt;

/// Error codes for Neo Contract RS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ErrorCode {
    /// Unknown error
    Unknown = 0,
    
    /// Invalid argument
    InvalidArgument = 1,
    
    /// Invalid format
    InvalidFormat = 2,
    
    /// Invalid state
    InvalidState = 3,
    
    /// Not found
    NotFound = 4,
    
    /// Unauthorized
    Unauthorized = 5,
    
    /// Insufficient funds
    InsufficientFunds = 6,
    
    /// Overflow error
    OverflowError = 7,
    
    /// Underflow error
    UnderflowError = 8,
    
    /// Encoding error
    EncodingError = 9,
    
    /// Decoding error
    DecodingError = 10,
    
    /// Execution error
    ExecutionError = 11,
    
    /// Storage error
    StorageError = 12,
    
    /// VM error
    VMError = 13,
    
    /// Contract error
    ContractError = 14,
    
    /// System error
    SystemError = 15,
    
    /// Reentrancy error
    ReentrancyError = 16,
    
    /// Validation error
    ValidationError = 17,
    
    /// Permission denied
    PermissionDenied = 18,
    
    /// Assertion error
    AssertionError = 19,
}

impl ErrorCode {
    /// Returns a human-readable name for the error code
    pub fn name(&self) -> &'static str {
        match self {
            ErrorCode::Unknown => "Unknown",
            ErrorCode::InvalidArgument => "InvalidArgument",
            ErrorCode::InvalidFormat => "InvalidFormat",
            ErrorCode::InvalidState => "InvalidState",
            ErrorCode::NotFound => "NotFound",
            ErrorCode::Unauthorized => "Unauthorized",
            ErrorCode::InsufficientFunds => "InsufficientFunds",
            ErrorCode::OverflowError => "OverflowError",
            ErrorCode::UnderflowError => "UnderflowError",
            ErrorCode::EncodingError => "EncodingError",
            ErrorCode::DecodingError => "DecodingError",
            ErrorCode::ExecutionError => "ExecutionError",
            ErrorCode::StorageError => "StorageError",
            ErrorCode::VMError => "VMError",
            ErrorCode::ContractError => "ContractError",
            ErrorCode::SystemError => "SystemError",
            ErrorCode::ReentrancyError => "ReentrancyError",
            ErrorCode::ValidationError => "ValidationError",
            ErrorCode::PermissionDenied => "PermissionDenied",
            ErrorCode::AssertionError => "AssertionError",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Error type for Neo Contract RS
#[derive(Debug, Clone)]
pub struct Error {
    /// The error code
    code: ErrorCode,
    
    /// The error message
    message: Option<String>,
}

impl Error {
    /// Creates a new error with the given code
    pub fn new(code: ErrorCode) -> Self {
        Error {
            code,
            message: None,
        }
    }
    
    /// Creates a new error with the given code and message
    pub fn with_message(code: ErrorCode, message: impl Into<String>) -> Self {
        Error {
            code,
            message: Some(message.into()),
        }
    }
    
    /// Gets the error code
    pub fn code(&self) -> ErrorCode {
        self.code
    }
    
    /// Gets the error message
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
    
    /// Sets the error message
    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = Some(message.into());
    }
    
    /// Clears the error message
    pub fn clear_message(&mut self) {
        self.message = None;
    }
    
    /// Creates an unknown error
    pub fn unknown() -> Self {
        Self::new(ErrorCode::Unknown)
    }
    
    /// Creates an invalid argument error
    pub fn invalid_argument() -> Self {
        Self::new(ErrorCode::InvalidArgument)
    }
    
    /// Creates an invalid format error
    pub fn invalid_format() -> Self {
        Self::new(ErrorCode::InvalidFormat)
    }
    
    /// Creates an invalid state error
    pub fn invalid_state() -> Self {
        Self::new(ErrorCode::InvalidState)
    }
    
    /// Creates a not found error
    pub fn not_found() -> Self {
        Self::new(ErrorCode::NotFound)
    }
    
    /// Creates an unauthorized error
    pub fn unauthorized() -> Self {
        Self::new(ErrorCode::Unauthorized)
    }
    
    /// Creates an insufficient funds error
    pub fn insufficient_funds() -> Self {
        Self::new(ErrorCode::InsufficientFunds)
    }
    
    /// Creates an overflow error
    pub fn overflow() -> Self {
        Self::new(ErrorCode::OverflowError)
    }
    
    /// Creates an underflow error
    pub fn underflow() -> Self {
        Self::new(ErrorCode::UnderflowError)
    }
    
    /// Creates an encoding error
    pub fn encoding() -> Self {
        Self::new(ErrorCode::EncodingError)
    }
    
    /// Creates a decoding error
    pub fn decoding() -> Self {
        Self::new(ErrorCode::DecodingError)
    }
    
    /// Creates an execution error
    pub fn execution() -> Self {
        Self::new(ErrorCode::ExecutionError)
    }
    
    /// Creates a storage error
    pub fn storage() -> Self {
        Self::new(ErrorCode::StorageError)
    }
    
    /// Creates a VM error
    pub fn vm() -> Self {
        Self::new(ErrorCode::VMError)
    }
    
    /// Creates a contract error
    pub fn contract() -> Self {
        Self::new(ErrorCode::ContractError)
    }
    
    /// Creates a system error
    pub fn system() -> Self {
        Self::new(ErrorCode::SystemError)
    }
    
    /// Creates a reentrancy error
    pub fn reentrancy() -> Self {
        Self::new(ErrorCode::ReentrancyError)
    }
    
    /// Creates a validation error
    pub fn validation() -> Self {
        Self::new(ErrorCode::ValidationError)
    }
    
    /// Creates a permission denied error
    pub fn permission_denied() -> Self {
        Self::new(ErrorCode::PermissionDenied)
    }
    
    /// Creates an assertion error
    pub fn assertion() -> Self {
        Self::new(ErrorCode::AssertionError)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.message {
            Some(message) => write!(f, "{}: {}", self.code, message),
            None => write!(f, "{}", self.code),
        }
    }
}

/// Result type for Neo Contract RS
pub type Result<T> = core::result::Result<T, Error>;
