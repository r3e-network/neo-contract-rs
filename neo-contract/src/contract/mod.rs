// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub mod native;
pub(crate) mod nep;
pub(crate) mod nep11;
pub(crate) mod nep17;

pub use {nep11::*, nep17::*};

use crate::{env, types::*};

pub trait SmartContract {
    #[inline(always)]
    fn _initialize() {}
}

pub trait TokenContract: SmartContract {
    fn symbol() -> ByteString;

    fn decimals() -> u32;

    fn total_supply() -> Int256;

    fn balance_of(account: H160) -> Int256;
}

pub trait Balancing {
    fn update_balance(account: H160, amount: Int256) -> bool;
}

impl<T: TokenContract> Balancing for T {
    fn update_balance(account: H160, amount: Int256) -> bool {
        false
    }
}

#[inline(always)]
pub fn call(contract: H160, method: ByteString, call_flags: CallFlags, args: Array<Any>) -> Any {
    unsafe { env::syscall::system_contract_call(contract, method, call_flags, args) }
}

#[inline(always)]
pub fn get_call_flags() -> CallFlags {
    unsafe { env::syscall::system_contract_get_call_flags() }
}

#[inline(always)]
pub fn create_standard_account(public_key: PublicKey) -> H160 {
    unsafe { env::syscall::system_contract_create_standard_account(public_key) }
}

#[inline(always)]
pub fn create_multi_signs_account(min_signers: u32, public_keys: Array<PublicKey>) -> H160 {
    unsafe { env::syscall::system_contract_create_multi_signs_account(min_signers, public_keys) }
}
