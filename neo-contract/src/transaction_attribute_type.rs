//! Transaction Attribute Types for Neo Contract RS
//!
//! This module defines the transaction attribute types used in Neo transactions.

use core::fmt;

/// Transaction attribute types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TransactionAttributeType {
    /// High priority - indicates the transaction should have high priority
    HighPriority = 0x01,

    /// Oracle response - used for oracle responses
    OracleResponse = 0x11,

    /// Not valid before - specifies a timestamp before which the transaction is not valid
    NotValidBefore = 0x20,

    /// Conflicts - specifies conflicting transactions
    Conflicts = 0x21,

    /// Additional script - additional script to execute
    AdditionalScript = 0x42,

    /// Network ID - ID of the network where the transaction is valid
    NetworkID = 0x4C,
}

impl TransactionAttributeType {
    /// Returns the byte value of the transaction attribute type
    pub fn value(&self) -> u8 { *self as u8 }

    /// Tries to convert a byte value to a TransactionAttributeType
    pub fn from_value(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(TransactionAttributeType::HighPriority),
            0x11 => Some(TransactionAttributeType::OracleResponse),
            0x20 => Some(TransactionAttributeType::NotValidBefore),
            0x21 => Some(TransactionAttributeType::Conflicts),
            0x42 => Some(TransactionAttributeType::AdditionalScript),
            0x4C => Some(TransactionAttributeType::NetworkID),
            _ => None,
        }
    }

    /// Returns a human-readable name for the transaction attribute type
    pub fn name(&self) -> &'static str {
        match self {
            TransactionAttributeType::HighPriority => "High Priority",
            TransactionAttributeType::OracleResponse => "Oracle Response",
            TransactionAttributeType::NotValidBefore => "Not Valid Before",
            TransactionAttributeType::Conflicts => "Conflicts",
            TransactionAttributeType::AdditionalScript => "Additional Script",
            TransactionAttributeType::NetworkID => "Network ID",
        }
    }
}

impl fmt::Display for TransactionAttributeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.name()) }
}

/// Converts a TransactionAttributeType to a u8
impl From<TransactionAttributeType> for u8 {
    fn from(attribute_type: TransactionAttributeType) -> Self { attribute_type.value() }
}

/// Tries to convert a u8 to a TransactionAttributeType
impl TryFrom<u8> for TransactionAttributeType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> { TransactionAttributeType::from_value(value).ok_or(()) }
}
