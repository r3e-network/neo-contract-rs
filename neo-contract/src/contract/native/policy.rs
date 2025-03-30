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

    #[inline(always)]
    pub fn get_fee_per_byte() -> Int256 { unsafe { env::contract::native_policy_get_fee_per_byte() } }

    #[inline(always)]
    pub fn get_exec_fee_factor() -> Int256 { unsafe { env::contract::native_policy_get_exec_fee_factor() } }

    #[inline(always)]
    pub fn get_storage_price() -> Int256 { unsafe { env::contract::native_policy_get_storage_price() } }

    #[inline(always)]
    pub fn is_blocked(account: H160) -> bool { unsafe { env::contract::native_policy_is_blocked(account) } }

    #[inline(always)]
    pub fn get_attr_fee(attr_type: TxAttrType) -> Int256 {
        unsafe { env::contract::native_policy_get_attr_fee(attr_type) }
    }

    #[inline(always)]
    pub fn set_attr_fee(attr_type: TxAttrType, fee: Int256) {
        unsafe { env::contract::native_policy_set_attr_fee(attr_type, fee) }
    }
}
