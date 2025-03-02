// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub mod native;

pub(crate) mod event;
pub(crate) mod nep;
pub(crate) mod nep11;
pub(crate) mod nep17;
pub(crate) mod token;

pub use {event::*, nep::*, nep11::*, nep17::*};

use crate::{env, types::*};

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
