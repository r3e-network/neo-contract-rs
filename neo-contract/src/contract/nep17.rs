// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::contract::{self, native::ContractManagement};
use crate::runtime;
use crate::storage::StorageMap;
use crate::types::{builtin::IntoAny, *};

/// Default total supply key.
/// Do not change the default TOTAL_NEP17_SUPPLY_KEY value if really necessary.
pub const TOTAL_NEP17_SUPPLY_KEY: u8 = 0x00;

/// Default balance key prefix.
/// Do not change the default PREFIX_NEP17_BALANCE value if really necessary.
pub const PREFIX_NEP17_BALANCE: u8 = 0x01;

// NOTE: neo-contract-proc-macros must be updated
//if any method definition changed(add, remove, modify) in this trait
pub trait Nep17Token {
    #[inline(always)]
    fn _initialize() {}

    fn symbol() -> ByteString;

    fn decimals() -> u32;

    #[inline(always)]
    fn total_supply() -> Int256 { contract::token::total_supply::<TOTAL_NEP17_SUPPLY_KEY>() }

    #[inline(always)]
    fn balance_of(owner: H160) -> Int256 { contract::token::balance_of::<PREFIX_NEP17_BALANCE>(owner) }

    fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        if amount.is_negative() {
            runtime::throw(/* TODO: add message */);
            // return false; // unreachable
        }

        if runtime::check_witness_with_account(from) {
            return false;
        }

        if amount.is_positive() {
            if !update_nep17_balance::<PREFIX_NEP17_BALANCE>(from, amount.checked_neg()) {
                return false;
            }
            let _ = update_nep17_balance::<PREFIX_NEP17_BALANCE>(to, amount);
        }

        // TODO: add `data` argument
        post_nep17_transfer(Nullable::new(from), Nullable::new(to), amount, Any::null());
        true
    }

    // fn transfer_with_data(from: H160, to: H160, amount: Int256, data: Any) -> bool;

    fn mint(account: H160, amount: Int256) {
        if amount.is_negative() {
            runtime::throw(/* TODO: add message */);
            // return; // unreachable
        }

        if amount.is_zero() {
            return;
        }

        let _ = update_nep17_balance::<PREFIX_NEP17_BALANCE>(account, amount);
        update_nep17_total_supply::<TOTAL_NEP17_SUPPLY_KEY>(amount);

        post_nep17_transfer(Nullable::null(), Nullable::new(account), amount, Any::null());
    }

    fn burn(account: H160, amount: Int256) {
        if amount.is_negative() {
            runtime::throw(/* TODO: add message */);
            // return; // unreachable
        }

        if amount.is_zero() {
            return;
        }

        let burned = amount.checked_neg();
        let _ = update_nep17_balance::<PREFIX_NEP17_BALANCE>(account, burned);
        update_nep17_total_supply::<TOTAL_NEP17_SUPPLY_KEY>(burned);
        post_nep17_transfer(Nullable::new(account), Nullable::null(), amount, Any::null());
    }
}

pub fn update_nep17_balance<const PREFIX: u8>(account: H160, amount: Int256) -> bool {
    let mut storage = StorageMap::new();
    contract::token::update_balance::<PREFIX>(&mut storage, account, amount)
}

pub fn update_nep17_total_supply<const PREFIX: u8>(amount: Int256) {
    let mut storage = StorageMap::new();
    contract::token::update_total_supply::<TOTAL_NEP17_SUPPLY_KEY>(&mut storage, amount)
}

pub fn post_nep17_transfer(from: Nullable<H160>, to: Nullable<H160>, amount: Int256, data: Any) {
    let mut event_state = Array::new();
    event_state.push(from.into_any());
    event_state.push(to.clone().into_any());
    event_state.push(amount.into_any()); // TODO: optimize with PACK
    contract::event::notify(ByteString::empty() /* TODO: 'OnTransfer' */, event_state);

    if to.is_null() {
        return;
    }

    let hash = unsafe { to.unwrap_unchecked() };
    if !ContractManagement::contract_of_hash(hash).is_null() {
        let mut args = Array::new();
        args.push(data);

        // TODO: 'onNEP17Payment'
        contract::call(hash, ByteString::empty(), CallFlags::All, args);
    }
}
