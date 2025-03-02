// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

/// Represents the type of a transaction attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionAttributeType {
    /// Indicates that the transaction is of high priority.
    HighPriority = 0x01,

    /// Indicates that the transaction is an oracle response.
    OracleResponse = 0x11,

    /// Indicates that the transaction is not valid before a certain height.
    NotValidBefore = 0x20,

    /// Indicates that the transaction conflicts with another transaction.
    Conflicts = 0x21,

    /// Indicates that the transaction is aimed to service notary request with a number of keys.
    NotaryAssisted = 0x22,
}

impl From<TransactionAttributeType> for u8 {
    fn from(value: TransactionAttributeType) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for TransactionAttributeType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(TransactionAttributeType::HighPriority),
            0x11 => Ok(TransactionAttributeType::OracleResponse),
            0x20 => Ok(TransactionAttributeType::NotValidBefore),
            0x21 => Ok(TransactionAttributeType::Conflicts),
            0x22 => Ok(TransactionAttributeType::NotaryAssisted),
            _ => Err(()),
        }
    }
}
