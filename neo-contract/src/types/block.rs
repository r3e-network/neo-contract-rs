// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::types::builtin::h160::H160;
use crate::types::builtin::h256::H256;
use crate::types::tx::Tx;

/// Block represents a block in the blockchain
#[derive(Debug, Clone)]
pub struct Block {
    /// Hash of the block
    pub hash: H256,
    /// Version of the block
    pub version: u32,
    /// Previous block hash
    pub prev_hash: H256,
    /// Merkle root
    pub merkle_root: H256,
    /// Timestamp
    pub timestamp: u64,
    /// Index
    pub index: u32,
    /// Next consensus
    pub next_consensus: H160,
    /// Transactions
    pub transactions: Vec<Tx>,
}

impl Block {
    /// Create a new block
    pub fn new() -> Self {
        Self {
            hash: H256::zero(),
            version: 0,
            prev_hash: H256::zero(),
            merkle_root: H256::zero(),
            timestamp: 0,
            index: 0,
            next_consensus: H160::zero(),
            transactions: Vec::new(),
        }
    }
}
