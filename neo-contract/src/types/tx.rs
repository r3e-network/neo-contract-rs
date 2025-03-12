// Copyright @ 2024 - present, R3E Network
// All Rights Reserved


use crate::types::builtin::h256::H256;
use crate::types::builtin::string::ByteString;

/// Transaction represents a transaction in the blockchain
#[derive(Debug, Clone)]
pub struct Tx {
    /// Hash of the transaction
    pub hash: H256,
    /// Version of the transaction
    pub version: u32,
    /// Nonce
    pub nonce: u32,
    /// Sender
    pub sender: ByteString,
    /// System fee
    pub system_fee: i64,
    /// Network fee
    pub network_fee: i64,
    /// Valid until block
    pub valid_until_block: u32,
    /// Script
    pub script: ByteString,
}

impl Tx {
    /// Create a new transaction
    pub fn new() -> Self {
        Self {
            hash: H256::zero(),
            version: 0,
            nonce: 0,
            sender: ByteString::empty(),
            system_fee: 0,
            network_fee: 0,
            valid_until_block: 0,
            script: ByteString::empty(),
        }
    }
}

impl Default for Tx {
    fn default() -> Self {
        Self::new()
    }
}
