// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::types::builtin::h160::H160;
use crate::types::builtin::h256::H256;
use crate::types::builtin::int256::Int256;
use crate::types::tx::Transaction;

/// Block represents a Neo block
#[derive(Debug, Clone)]
pub struct Block {
    pub hash: H256,
    pub version: u8,
    pub previous_hash: H256,
    pub merkle_root: H256,
    pub timestamp: u64,
    pub index: u32,
    pub primary_index: u8,
    pub next_consensus: H160,
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Create a new block
    pub fn new() -> Self {
        Self {
            hash: H256::zero(),
            version: 0,
            previous_hash: H256::zero(),
            merkle_root: H256::zero(),
            timestamp: 0,
            index: 0,
            primary_index: 0,
            next_consensus: H160::zero(),
            transactions: Vec::new(),
        }
    }

    /// Get the block hash
    pub fn hash(&self) -> H256 {
        self.hash.clone()
    }

    /// Get the block version
    pub fn version(&self) -> u8 {
        self.version
    }

    /// Get the previous block hash
    pub fn previous_hash(&self) -> H256 {
        self.previous_hash.clone()
    }

    /// Get the merkle root
    pub fn merkle_root(&self) -> H256 {
        self.merkle_root.clone()
    }

    /// Get the timestamp
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Get the block index
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Get the primary index
    pub fn primary_index(&self) -> u8 {
        self.primary_index
    }

    /// Get the next consensus
    pub fn next_consensus(&self) -> H160 {
        self.next_consensus.clone()
    }

    /// Get the transactions
    pub fn transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }
}
