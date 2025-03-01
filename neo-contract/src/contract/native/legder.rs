// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::types::builtin::h160::H160;
use crate::types::builtin::int256::Int256;

/// Ledger represents the Ledger native contract
pub struct Ledger;

impl Ledger {
    /// Get the contract hash
    pub fn hash() -> H160 {
        H160::hex_decode("0xda65b600f7124ce6c79950c1772a36403104f2be").expect("Invalid hash")
    }

    /// Get the current index
    pub fn current_index() -> u32 {
        0
    }

    /// Get the current hash
    pub fn current_hash() -> H160 {
        H160::zero()
    }

    /// Get the hash at the specified index
    pub fn hash_at(_index: u32) -> H160 {
        H160::zero()
    }
}
