use crate::types::ByteString;

extern crate alloc;
use alloc::string::String;

/// Contract error types
#[derive(Debug, Clone)]
pub enum ContractError {
    /// Account does not exist
    AccountNotFound,
    /// Account already exists
    AccountAlreadyExists,
    /// Account is not mutable
    AccountNotMutable,
    /// Invalid account data
    InvalidAccountData,
    /// Invalid discriminator
    InvalidDiscriminator,
    /// Invalid account relation
    InvalidAccountRelation,
    /// Invalid PDA
    InvalidPDA,
    /// Insufficient funds
    InsufficientFunds,
    /// Insufficient space
    InsufficientSpace,
    /// Missing signer
    MissingSigner,
    /// Invalid instruction
    InvalidInstruction,
    /// Arithmetic overflow
    ArithmeticOverflow,
    /// Unauthorized
    Unauthorized,
    /// Invalid argument
    InvalidArgument,
    /// Custom error with code and message
    Custom(u32, String),
    /// Custom error with ByteString message
    CustomString(ByteString),
    
    // Security-focused error types
    /// Data corruption detected
    DataCorruption,
    /// Invalid data format
    InvalidDataFormat,
    /// Data too large
    DataTooLarge,
    /// Serialization failed
    SerializationFailure,
    /// Deserialization failed
    DeserializationFailure,
    /// Input validation failed
    ValidationFailure(ByteString),
    /// Storage access denied
    StorageAccessDenied,
    /// Reentrancy detected
    ReentrancyDetected,
    /// Rate limit exceeded
    RateLimitExceeded,
}

impl ContractError {
    pub fn to_byte_string(&self) -> ByteString {
        match self {
            ContractError::AccountNotFound => ByteString::from_literal("AccountNotFound"),
            ContractError::AccountAlreadyExists => ByteString::from_literal("AccountAlreadyExists"),
            ContractError::AccountNotMutable => ByteString::from_literal("AccountNotMutable"),
            ContractError::InvalidAccountData => ByteString::from_literal("InvalidAccountData"),
            ContractError::InvalidDiscriminator => ByteString::from_literal("InvalidDiscriminator"),
            ContractError::InvalidAccountRelation => ByteString::from_literal("InvalidAccountRelation"),
            ContractError::InvalidPDA => ByteString::from_literal("InvalidPDA"),
            ContractError::InsufficientFunds => ByteString::from_literal("InsufficientFunds"),
            ContractError::InsufficientSpace => ByteString::from_literal("InsufficientSpace"),
            ContractError::MissingSigner => ByteString::from_literal("MissingSigner"),
            ContractError::InvalidInstruction => ByteString::from_literal("InvalidInstruction"),
            ContractError::ArithmeticOverflow => ByteString::from_literal("ArithmeticOverflow"),
            ContractError::Unauthorized => ByteString::from_literal("Unauthorized"),
            ContractError::InvalidArgument => ByteString::from_literal("InvalidArgument"),
            ContractError::Custom(_code, msg) => {
                ByteString::from(msg.as_bytes())
            }
            ContractError::CustomString(msg) => msg.clone(),
            ContractError::DataCorruption => ByteString::from_literal("DataCorruption"),
            ContractError::InvalidDataFormat => ByteString::from_literal("InvalidDataFormat"),
            ContractError::DataTooLarge => ByteString::from_literal("DataTooLarge"),
            ContractError::SerializationFailure => ByteString::from_literal("SerializationFailure"),
            ContractError::DeserializationFailure => ByteString::from_literal("DeserializationFailure"),
            ContractError::ValidationFailure(msg) => msg.clone(),
            ContractError::StorageAccessDenied => ByteString::from_literal("StorageAccessDenied"),
            ContractError::ReentrancyDetected => ByteString::from_literal("ReentrancyDetected"),
            ContractError::RateLimitExceeded => ByteString::from_literal("RateLimitExceeded"),
        }
    }
    
    pub fn code(&self) -> u32 {
        match self {
            ContractError::AccountNotFound => 6000,
            ContractError::AccountAlreadyExists => 6001,
            ContractError::AccountNotMutable => 6002,
            ContractError::InvalidAccountData => 6003,
            ContractError::InvalidDiscriminator => 6004,
            ContractError::InvalidAccountRelation => 6005,
            ContractError::InvalidPDA => 6006,
            ContractError::InsufficientFunds => 6007,
            ContractError::InsufficientSpace => 6008,
            ContractError::MissingSigner => 6009,
            ContractError::InvalidInstruction => 6010,
            ContractError::ArithmeticOverflow => 6011,
            ContractError::Unauthorized => 6012,
            ContractError::InvalidArgument => 6013,
            ContractError::Custom(code, _) => *code,
            ContractError::CustomString(_) => 6014,
            ContractError::DataCorruption => 7001,
            ContractError::InvalidDataFormat => 7002,
            ContractError::DataTooLarge => 7003,
            ContractError::SerializationFailure => 7004,
            ContractError::DeserializationFailure => 7005,
            ContractError::ValidationFailure(_) => 7006,
            ContractError::StorageAccessDenied => 7007,
            ContractError::ReentrancyDetected => 7008,
            ContractError::RateLimitExceeded => 7009,
        }
    }
}

impl From<ContractError> for ByteString {
    fn from(err: ContractError) -> Self {
        err.to_byte_string()
    }
}

impl From<ByteString> for ContractError {
    fn from(msg: ByteString) -> Self {
        ContractError::CustomString(msg)
    }
}

/// Result type alias
pub type Result<T> = core::result::Result<T, ContractError>;

/// Macro for requiring a condition
#[macro_export]
macro_rules! require {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err.into());
        }
    };
}

/// Macro for requiring equality
#[macro_export]
macro_rules! require_eq {
    ($left:expr, $right:expr, $err:expr) => {
        if $left != $right {
            return Err($err.into());
        }
    };
}

/// Macro for requiring inequality
#[macro_export]
macro_rules! require_neq {
    ($left:expr, $right:expr, $err:expr) => {
        if $left == $right {
            return Err($err.into());
        }
    };
}

/// Macro for requiring greater than
#[macro_export]
macro_rules! require_gt {
    ($left:expr, $right:expr, $err:expr) => {
        if $left <= $right {
            return Err($err.into());
        }
    };
}

/// Macro for requiring greater than or equal
#[macro_export]
macro_rules! require_gte {
    ($left:expr, $right:expr, $err:expr) => {
        if $left < $right {
            return Err($err.into());
        }
    };
}

/// Macro for requiring keys equality
#[macro_export]
macro_rules! require_keys_eq {
    ($left:expr, $right:expr, $err:expr) => {
        if $left != $right {
            return Err($err.into());
        }
    };
}

/// Macro for requiring keys inequality
#[macro_export]
macro_rules! require_keys_neq {
    ($left:expr, $right:expr, $err:expr) => {
        if $left == $right {
            return Err($err.into());
        }
    };
}

/// Macro for validating data size limits
#[macro_export]
macro_rules! require_data_size {
    ($data:expr, $max_size:expr) => {
        if $data.len() > $max_size {
            return Err(ContractError::DataTooLarge);
        }
    };
}

/// Macro for validating input parameters
#[macro_export]
macro_rules! validate_input {
    ($condition:expr, $msg:expr) => {
        if !$condition {
            return Err(ContractError::ValidationFailure(ByteString::from_literal($msg)));
        }
    };
}

/// Macro for safe deserialization with validation
#[macro_export]
macro_rules! safe_deserialize {
    ($data:expr, $type:ty) => {
        match $data {
            value if value.is_null() => Ok(Default::default()),
            value => {
                let byte_string = value.as_ref()
                    .ok_or(ContractError::DeserializationFailure)?;
                
                if byte_string.as_bytes().len() > <$type>::MAX_SIZE {
                    return Err(ContractError::DataTooLarge);
                }
                
                <$type>::from_byte_string(byte_string.clone())
                    .map_err(|_| ContractError::DeserializationFailure)
            }
        }
    };
}

/// Macro for reentrancy protection
#[macro_export]
macro_rules! nonreentrant {
    ($guard:expr) => {
        if $guard.is_locked() {
            return Err(ContractError::ReentrancyDetected);
        }
        let _lock = $guard.lock()?;
    };
}