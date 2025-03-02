// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::{
    contract::token,
    runtime,
    storage::{Iter, StorageMap},
    types::{placeholder::*, *},
};

pub const PREFIX_TOKEN_ID: u8 = 0x02;
pub const PREFIX_TOKEN: u8 = 0x03;
pub const PREFIX_ACCOUNT_TOKEN: u8 = 0x04;

pub trait TokenState {
    fn name() -> ByteString;

    fn owner() -> H160;
}

// NOTE: neo-contract-proc-macros must be updated
// if any method definition changed(add, remove, modify) in this trait
pub trait Nep11Token<T: TokenState + FromPlaceholder> {
    #[inline(always)]
    fn _initialize() {}

    fn symbol() -> ByteString;

    fn decimals() -> u32;

    #[inline(always)]
    fn total_supply() -> Int256 {
        token::total_supply()
    }

    #[inline(always)]
    fn balance_of(owner: H160) -> Int256 {
        token::balance_of(owner)
    }

    fn owner_of(token_id: ByteString) -> H160 {
        if token_id.len() > 64 {
            runtime::abort(); // TODO: add message
            return H160::zero(); // unreachable
        }

        let storage = StorageMap::new();
        let value = storage.get(token_id);
        if value.is_null() {
            runtime::abort(); // TODO: add message
            return H160::zero(); // unreachable
        }

        return H160::zero(); // TODO: return value
    }

    fn properties(token_id: ByteString) -> Map<ByteString, Any>;

    fn tokens() -> Iter<T>;

    fn tokens_of(owner: H160) -> Iter<T>;

    fn transfer(to: H160, token_id: ByteString);

    fn mint(token_id: ByteString, token_state: T);

    fn burn(token_id: ByteString);
}

pub fn update_nep11_balance(owner: H160, token_id: ByteString, increment: Int256) {
    let mut storage = StorageMap::new();
    let ok = token::update_balance::<PREFIX_ACCOUNT_TOKEN>(&mut storage, owner, increment);
    if !ok {
        runtime::abort(); // TODO: add message
        return; // unreachable
    }

    let key = owner.into_byte_string().concat(&token_id);
    if increment.is_positive() {
        storage.put(key, Int256::zero().into_byte_string());
    } else {
        storage.delete(key);
    }
}
