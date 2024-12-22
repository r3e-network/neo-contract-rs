// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{contract::*, env, types::*};

pub struct Gas;

impl Gas {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_gas_contract_hash() }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0xd2a4cff31913016155e38e474a2c06d08be276cf")
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn symbol() -> ByteString {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_gas_symbol() }

        #[cfg(not(target_family = "wasm"))]
        ByteString::new("GAS".into())
    }

    #[inline(always)]
    pub fn total_supply() -> Int256 {
        unsafe { env::contract::native_gas_total_supply() }
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn decimals() -> u32 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_gas_decimals() }
    
        #[cfg(not(target_family = "wasm"))]
        8
    }

    #[inline(always)]
    pub fn balance_of(account: H160) -> Int256 {
        unsafe { env::contract::native_gas_balance_of(account) }
    }

    #[inline(always)]
    pub fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        unsafe { env::contract::native_gas_transfer(from, to, amount) }
    }
}
