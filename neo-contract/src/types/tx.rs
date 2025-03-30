// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[repr(C)]
#[crate::inner_structs]
pub struct Tx {
    #[get(pub)]
    hash: H256,

    /// an u8 value
    #[get(pub)]
    version: Int256,

    /// an u32 value
    #[get(pub)]
    nonce: Int256,

    #[get(pub)]
    sender: H160,

    /// an u64 value
    #[get(pub)]
    system_fee: Int256,

    /// an u64 value
    #[get(pub)]
    network_fee: Int256,

    /// a u32 value
    #[get(pub)]
    valid_until_block: Int256,

    #[get(pub)]
    script: ByteString,
}
