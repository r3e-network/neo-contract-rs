// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub mod native;
pub(crate) mod nep;
pub mod nep5;
pub mod nep11;
pub mod nep17;

pub use {nep5::*, nep11::*, nep17::*};

use crate::{storage::StorageMap, types::{*, consts::DEFAULT_BALANCE_KEY}};

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
        if amount.is_zero() {
            return true;
        }
        
        let balance = Self::balance_of(account);
        
        if amount.is_negative() && balance < -amount {
            return false;
        }
        
        let storage = StorageMap::new();
        
        // Create key directly
        let mut key = Vec::with_capacity(account.as_bytes().len() + 1);
        key.push(DEFAULT_BALANCE_KEY);
        key.extend_from_slice(account.as_bytes());
        
        let new_balance = balance + amount;
        
        if new_balance.is_zero() {
            storage.delete(&key);
        } else {
            storage.put(&key, &new_balance);
        }
        
        return true;
    }
}

#[inline(always)]
pub fn call(contract: H160, method: ByteString, call_flags: CallFlags, args: Array<Any>) -> Any {
    #[cfg(target_family = "wasm")]
    unsafe { env::syscall::system_contract_call(contract, method, call_flags, args) }

    #[cfg(not(target_family = "wasm"))]
    unsafe { crate::env::syscall_non_wasm::system_contract_call(contract, method, call_flags, args) }
}

#[inline(always)]
pub fn get_call_flags() -> CallFlags {
    #[cfg(target_family = "wasm")]
    unsafe { env::syscall::system_contract_get_call_flags() }

    #[cfg(not(target_family = "wasm"))]
    unsafe { crate::env::syscall_non_wasm::system_contract_get_call_flags() }
}

#[inline(always)]
pub fn create_standard_account(public_key: PublicKey) -> H160 {
    #[cfg(target_family = "wasm")]
    unsafe { env::syscall::system_contract_create_standard_account(public_key) }

    #[cfg(not(target_family = "wasm"))]
    unsafe { crate::env::syscall_non_wasm::system_contract_create_standard_account(public_key) }
}

#[inline(always)]
pub fn create_multi_signs_account(min_signers: u32, public_keys: Array<PublicKey>) -> H160 {
    #[cfg(target_family = "wasm")]
    unsafe { env::syscall::system_contract_create_multi_signs_account(min_signers, public_keys) }

    #[cfg(not(target_family = "wasm"))]
    unsafe { crate::env::syscall_non_wasm::system_contract_create_multi_signs_account(min_signers, public_keys) }
}
