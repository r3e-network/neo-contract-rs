// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[repr(C)]
pub struct Block {
    hash: H256,
    version: u32,
    prev_hash: H256,
    merkle_root: H256,
    timestamp: u64,
    nonce: u64,
    index: u32, // current block index
    primary_index: u32,
    next_consensus: H160,
    tx_count: u32,
}

impl Block {
    #[inline(always)]
    pub fn hash(&self) -> H256 {
        self.hash
    }

    #[inline(always)]
    pub fn version(&self) -> u32 {
        self.version
    }

    #[inline(always)]
    pub fn prev_hash(&self) -> H256 {
        self.prev_hash
    }

    #[inline(always)]
    pub fn merkle_root(&self) -> H256 {
        self.merkle_root
    }

    #[inline(always)]
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    #[inline(always)]
    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    #[inline(always)]
    pub fn index(&self) -> u32 {
        self.index
    }

    #[inline(always)]
    pub fn primary_index(&self) -> u32 {
        self.primary_index
    }

    #[inline(always)]
    pub fn next_consensus(&self) -> H160 {
        self.next_consensus
    }

    #[inline(always)]
    pub fn tx_count(&self) -> u32 {
        self.tx_count
    }
}
