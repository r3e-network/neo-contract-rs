// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};

pub struct Ledger;

impl Ledger {
    #[inline(always)]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_ledger_contract_hash() }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0xda65b600f7124ce6c79950c1772a36403104f2be")
    }

    #[inline(always)]
    pub fn current_block_index() -> u32 {
        unsafe { env::contract::native_ledger_current_block_index() }
    }

    #[inline(always)]
    pub fn current_block_hash() -> H256 {
        unsafe { env::contract::native_ledger_current_block_hash() }
    }

    #[inline(always)]
    pub fn block_of_index(index: u32) -> Block {
        unsafe { env::contract::native_ledger_block_of_index(index) }
    }

    #[inline(always)]
    pub fn block_of_hash(hash: H256) -> Block {
        unsafe { env::contract::native_ledger_block_of_hash(hash) }
    }
}
