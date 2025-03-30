// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[repr(C)]
#[crate::inner_structs]
pub struct Block {
    #[get(pub)]
    hash: H256,

    /// an u32 value
    #[get(pub)]
    version: Int256,

    #[get(pub)]
    prev_hash: H256,

    #[get(pub)]
    merkle_root: H256,

    /// an u64 value
    #[get(pub)]
    timestamp: Int256,

    /// an u64 value
    #[get(pub)]
    nonce: Int256,

    /// an u32 value, i.e. current block index
    #[get(pub)]
    index: Int256,

    /// an u8 value
    #[get(pub)]
    primary_index: Int256,

    /// an H160 value
    #[get(pub)]
    next_consensus: H160,

    /// an u32 value
    #[get(pub)]
    tx_count: Int256,
}
