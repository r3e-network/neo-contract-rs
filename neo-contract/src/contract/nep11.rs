// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::contract::{self, native::ContractManagement};
use crate::runtime;
use crate::serialize::*;
use crate::storage::{Iter, StorageMap};
use crate::types::{placeholder::*, *};

/// Default total supply key.
/// Do not change the default TOTAL_NEP11_SUPPLY_KEY value if really necessary.
pub const TOTAL_NEP11_SUPPLY_KEY: u8 = 0x00;

/// Default balance key prefix.
/// Do not change the default PREFIX_NEP11_BALANCE value if really necessary.
pub const PREFIX_NEP11_BALANCE: u8 = 0x01;

pub const PREFIX_TOKEN_ID: u8 = 0x02;
pub const PREFIX_TOKEN: u8 = 0x03;
pub const PREFIX_ACCOUNT_TOKEN: u8 = 0x04;

pub const MAX_TOKEN_ID_LENGTH: usize = 64;
pub const SCAN_TOKEN_OPTIONS: u32 = FindOptions::KeysOnly as u32 | FindOptions::RemovePrefix as u32;

pub trait Nep11TokenState {
    /// NOTE: must be #[inline(always)]
    fn token_name(&self) -> ByteString;

    /// NOTE: must be #[inline(always)]
    fn token_owner(&self) -> H160;

    /// NOTE: must be #[inline(always)]
    fn set_token_owner(&mut self, owner: H160);
}

// NOTE: neo-contract-proc-macros must be updated
// if any method definition changed(add, remove, modify) in this trait
pub trait Nep11Token<T: Nep11TokenState + FromPlaceholder + IntoPlaceholder + 'static> {
    #[inline(always)]
    fn _initialize() {}

    fn symbol() -> ByteString;

    fn decimals() -> u32;

    #[inline(always)]
    fn total_supply() -> Int256 { contract::token::total_supply::<TOTAL_NEP11_SUPPLY_KEY>() }

    #[inline(always)]
    fn balance_of(owner: H160) -> Int256 { contract::token::balance_of::<PREFIX_NEP11_BALANCE>(owner) }

    fn owner_of(token_id: ByteString) -> H160 {
        if token_id.len() > MAX_TOKEN_ID_LENGTH {
            runtime::throw(/* TODO: add message */);
            // return H160::zero(); // unreachable
        }

        let storage = StorageMap::new();
        let value = storage.get(token_id);
        if value.is_null() {
            runtime::throw(/* TODO: add message */);
            // return H160::zero(); // unreachable
        }

        let token_state = T::deserialize(unsafe { value.unwrap_unchecked() });
        token_state.token_owner()
    }

    fn properties(token_id: ByteString) -> Map<ByteString, Any> {
        let key = contract::token::prefixed_key::<PREFIX_TOKEN>(token_id);
        let storage = StorageMap::new();
        let value = storage.get(key);
        if value.is_null() {
            runtime::throw(/* TODO: add message */);
            // return Map::new(); // unreachable
        }

        let _token_state = T::deserialize(unsafe { value.unwrap_unchecked() });
        let map = Map::new();
        // map.put(ByteString::from_bytes(&[0x00]), token_state.token_name());
        map
    }

    fn tokens() -> Iter<T> {
        let prefix = ByteString::one_byte::<PREFIX_TOKEN>();
        let storage = StorageMap::new();
        storage.scan_prefix::<T, SCAN_TOKEN_OPTIONS>(prefix)
    }

    fn tokens_of(owner: H160) -> Iter<T> {
        // TODO: check `owner` is valid.
        let storage = StorageMap::new();
        let key = ByteString::one_byte::<PREFIX_ACCOUNT_TOKEN>().concat(owner.into_byte_string());
        storage.scan_prefix::<T, SCAN_TOKEN_OPTIONS>(key)
    }

    fn transfer(to: H160, token_id: ByteString) -> bool {
        // TODO: check `to` and `token_id` is valid.
        let mut storage = StorageMap::new();
        let key = contract::token::prefixed_key::<PREFIX_TOKEN>(token_id.clone());
        let value = storage.get(key.clone());
        if value.is_null() {
            runtime::throw(/* TODO: add message */);
            // return; // unreachable
        }

        let mut token_state = T::deserialize(unsafe { value.unwrap_unchecked() });
        let token_owner = token_state.token_owner();
        if runtime::check_witness_with_account(token_owner) {
            return false;
        }

        if token_owner != to {
            token_state.set_token_owner(to);
            storage.put(key, token_state.serialize());

            // TODO: optimzie duplicated storage context creation
            update_nep11_balance(token_owner, token_id.clone(), Int256::minus_one());
            update_nep11_balance(to, token_id.clone(), Int256::one());
        }

        // TODO: add `data` argument
        post_nep11_transfer(Nullable::new(token_owner), Nullable::new(to), token_id, Any::null());
        true
    }

    fn mint(token_id: ByteString, token_state: T) {
        let mut storage = StorageMap::new();
        let key = contract::token::prefixed_key::<PREFIX_TOKEN>(token_id.clone());

        let token_owner = token_state.token_owner();
        storage.put(key, token_state.serialize());

        // TODO: optimzie duplicated storage context creation
        update_nep11_balance(token_owner, token_id.clone(), Int256::one());
        update_nep11_total_supply::<TOTAL_NEP11_SUPPLY_KEY>(Int256::one());

        post_nep11_transfer(Nullable::null(), Nullable::new(token_owner), token_id, Any::null());
    }

    fn burn(token_id: ByteString) {
        let mut storage = StorageMap::new();
        let key = contract::token::prefixed_key::<PREFIX_TOKEN>(token_id.clone());
        let value = storage.get(key.clone());
        if value.is_null() {
            runtime::throw(/* TODO: add message */);
            // return; // unreachable
        }

        let token_state = T::deserialize(unsafe { value.unwrap_unchecked() });
        storage.delete(key);

        let owner = token_state.token_owner();
        update_nep11_balance(owner, token_id.clone(), Int256::minus_one());
        update_nep11_total_supply::<TOTAL_NEP11_SUPPLY_KEY>(Int256::minus_one());

        post_nep11_transfer(Nullable::new(owner), Nullable::null(), token_id, Any::null());
    }
}

pub fn update_nep11_balance(owner: H160, token_id: ByteString, increment: Int256) {
    let mut storage = StorageMap::new();
    let ok = contract::token::update_balance::<PREFIX_ACCOUNT_TOKEN>(&mut storage, owner, increment);
    if !ok {
        runtime::abort(/* TODO: add message */);
        // return; // unreachable
    }

    let key = owner.into_byte_string().concat(token_id);
    if increment.is_positive() {
        storage.put(key, Int256::zero().into_byte_string());
    } else {
        storage.delete(key);
    }
}

pub fn update_nep11_total_supply<const PREFIX: u8>(amount: Int256) {
    let mut storage = StorageMap::new();
    contract::token::update_total_supply::<TOTAL_NEP11_SUPPLY_KEY>(&mut storage, amount)
}

pub fn post_nep11_transfer(from: Nullable<H160>, to: Nullable<H160>, token_id: ByteString, data: Any) {
    let mut event_state = Array::new();
    event_state.push(from.clone().into_any());
    event_state.push(to.clone().into_any());
    event_state.push(Int256::one().into_any());
    event_state.push(token_id.clone().into_any()); // TODO: optimize with PACK
    contract::event::notify(ByteString::empty() /* TODO: 'OnTransfer' */, event_state);

    if to.is_null() {
        return;
    }

    let hash = unsafe { to.unwrap_unchecked() };
    if !ContractManagement::contract_of_hash(hash).is_null() {
        let mut args = Array::new();
        args.push(from.into_any());
        args.push(Int256::one().into_any());
        args.push(token_id.into_any());
        args.push(data);

        // TODO: 'onNEP11Payment'
        contract::call(hash, ByteString::empty(), CallFlags::All, args);
    }
}
