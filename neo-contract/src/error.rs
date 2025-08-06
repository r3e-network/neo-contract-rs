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
            ContractError::Custom(_code, _msg) => {
                ByteString::from_literal("CustomError")
            }
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
        }
    }
}

impl From<ContractError> for ByteString {
    fn from(err: ContractError) -> Self {
        err.to_byte_string()
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