// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};

pub struct Neo;

impl Neo {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_neo_contract_hash() }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn symbol() -> ByteString {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_neo_symbol() }

        #[cfg(not(target_family = "wasm"))]
        ByteString::new("NEO".into())
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn decimals() -> u32 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_neo_decimals() }

        #[cfg(not(target_family = "wasm"))]
        0
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn total_supply() -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_neo_total_supply() }

        #[cfg(not(target_family = "wasm"))]
        Int256::new(1_0000_0000)
    }

    #[inline(always)]
    pub fn balance_of(account: H160) -> Int256 {
        unsafe { env::contract::native_neo_balance_of(account) }
    }

    #[inline(always)]
    pub fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        unsafe { env::contract::native_neo_transfer(from, to, amount) }
    }
}
