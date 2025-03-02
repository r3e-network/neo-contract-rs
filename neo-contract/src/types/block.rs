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
