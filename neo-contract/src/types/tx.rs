// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::types::builtin::h160::H160;
use crate::types::builtin::h256::H256;
use crate::types::builtin::int256::Int256;
use crate::types::builtin::string::ByteString;

/// Transaction represents a Neo transaction
#[derive(Debug, Clone)]
pub struct Transaction {
    hash: H256,
    version: u8,
    nonce: u32,
    sender: H160,
    system_fee: Int256,
    network_fee: Int256,
    valid_until_block: u32,
    script: Vec<u8>,
}

/// Alias for Transaction
pub type Tx = Transaction;

/// TriggerType represents a Neo trigger type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerType {
    /// OnPersist
    OnPersist = 0x01,
    /// PostPersist
    PostPersist = 0x02,
    /// Application
    Application = 0x10,
    /// Verification
    Verification = 0x20,
    /// All
    All = 0x33,
}

/// TxAttrType represents a Neo transaction attribute type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxAttrType {
    /// HighPriority
    HighPriority = 0x01,
    /// OracleResponse
    OracleResponse = 0x11,
}

/// VmState represents a Neo VM state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmState {
    /// None
    None = 0,
    /// Halt
    Halt = 1,
    /// Fault
    Fault = 2,
    /// Break
    Break = 4,
}

impl Transaction {
    /// Create a new transaction
    pub fn new() -> Self {
        Self {
            hash: H256::zero(),
            version: 0,
            nonce: 0,
            sender: H160::zero(),
            system_fee: Int256::new(0),
            network_fee: Int256::new(0),
            valid_until_block: 0,
            script: Vec::new(),
        }
    }

    /// Get the transaction hash
    pub fn hash(&self) -> H256 {
        self.hash.clone()
    }

    /// Get the transaction version
    pub fn version(&self) -> u8 {
        self.version
    }

    /// Get the transaction nonce
    pub fn nonce(&self) -> u32 {
        self.nonce
    }

    /// Get the transaction sender
    pub fn sender(&self) -> H160 {
        self.sender.clone()
    }

    /// Get the transaction system fee
    pub fn system_fee(&self) -> Int256 {
        self.system_fee.clone()
    }

    /// Get the transaction network fee
    pub fn network_fee(&self) -> Int256 {
        self.network_fee.clone()
    }

    /// Get the transaction valid until block
    pub fn valid_until_block(&self) -> u32 {
        self.valid_until_block
    }

    /// Get the transaction script
    pub fn script(&self) -> Vec<u8> {
        self.script.clone()
    }
}
