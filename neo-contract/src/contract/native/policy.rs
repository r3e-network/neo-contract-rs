// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};

pub struct Policy;

impl Policy {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_policy_contract_hash() }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0xcc5e4edd9f5f8dba8bb65734541df7a1c081c67b")
    }
}
