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
    #[rustfmt::skip]
    pub fn total_supply() -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_gas_total_supply() }

        #[cfg(not(target_family = "wasm"))]
        unsafe { crate::env::contract_non_wasm::native_gas_total_supply() }
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
    #[rustfmt::skip]
    pub fn balance_of(account: H160) -> Int256 {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_gas_balance_of(account) }

        #[cfg(not(target_family = "wasm"))]
        unsafe { crate::env::contract_non_wasm::native_gas_balance_of(account) }
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        #[cfg(target_family = "wasm")]
        unsafe { env::contract::native_gas_transfer(from, to, amount) }

        #[cfg(not(target_family = "wasm"))]
        unsafe { crate::env::contract_non_wasm::native_gas_transfer(from, to, amount) }
    }
}
