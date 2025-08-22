// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::{
    contract::token,
    error::{ContractError, Result},
    runtime,
    storage::StorageMap,
    types::{
        builtin::{
            array::Array,
            h160::H160,
            int256::Int256,
            string::{ByteString, FromByteString, IntoByteString},
            any::IntoAny,
        },
        Any,
    },
};

/// Default total supply key.
/// Do not change the default TOTAL_SUPPLY_KEY value if really necessary.
pub const TOTAL_SUPPLY_KEY: u8 = 0x00;

/// Default balance key prefix.
/// Do not change the default PREFIX_BALANCE value if really necessary.
pub const PREFIX_BALANCE: u8 = 0x01;

// NOTE: neo-contract-proc-macros must be updated
//if any method definition changed(add, remove, modify) in this trait
pub trait Nep17Token {
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

    fn transfer(from: H160, to: H160, amount: Int256, _data: Array<Any>) -> bool {
        if amount.is_negative() {
            runtime::abort();
            return false;
        }

        if !runtime::check_witness_with_account(from) {
            return false;
        }

        if amount.is_positive() {
            if !update_nep17_balance::<PREFIX_BALANCE>(from, amount.checked_neg()) {
                return false;
            }
            let _ = update_nep17_balance::<PREFIX_BALANCE>(to, amount);

            // Emit transfer event
            let mut event_data = Array::<Any>::new();
            event_data.push(from.into_any());
            event_data.push(to.into_any());
            event_data.push(amount.into_any());
            runtime::notify(ByteString::from_literal("Transfer"), event_data);
        }

        return true;
    }

    // fn transfer_with_data(from: H160, to: H160, amount: Int256, data: Any) -> bool;

    fn mint(account: H160, amount: Int256) {
        if amount.is_negative() {
            runtime::abort();
            return;
        }

        if amount.is_zero() {
            return;
        }

        let _ = update_nep17_balance::<PREFIX_BALANCE>(account, amount);
        update_nep17_total_supply::<TOTAL_SUPPLY_KEY>(amount);

        // Emit mint event (transfer from zero address)
        let mut event_data = Array::<Any>::new();
        event_data.push(H160::zero().into_any());
        event_data.push(account.into_any());
        event_data.push(amount.into_any());
        runtime::notify(ByteString::from_literal("Transfer"), event_data);
    }

    fn burn(account: H160, amount: Int256) {
        if amount.is_negative() {
            runtime::abort();
            return;
        }

        if amount.is_zero() {
            return;
        }

        let burned = amount.checked_neg();
        let _ = update_nep17_balance::<PREFIX_BALANCE>(account, burned);
        update_nep17_total_supply::<TOTAL_SUPPLY_KEY>(burned);

        // Emit burn event (transfer to zero address)
        let mut event_data = Array::<Any>::new();
        event_data.push(account.into_any());
        event_data.push(H160::zero().into_any());
        event_data.push(amount.into_any());
        runtime::notify(ByteString::from_literal("Transfer"), event_data);
    }
}

pub fn update_nep17_balance<const PREFIX: u8>(account: H160, amount: Int256) -> bool {
    let mut storage = StorageMap::new();
    token::update_balance::<PREFIX>(&mut storage, account, amount)
}

pub fn update_nep17_total_supply<const KEY: u8>(amount: Int256) {
    #[cfg(target_family = "wasm")]
    let key = ByteString::from_bytes(&[KEY]);

    #[cfg(not(target_family = "wasm"))]
    let key = ByteString::from_bytes(&[KEY]);

    let mut storage = StorageMap::new();
    let value = storage.get(key.clone());
    let total_supply = if value.is_null() {
        Int256::zero()
    } else {
        // Safe deserialization with error handling
        match try_safe_from_byte_string(value) {
            Ok(amount) => amount,
            Err(_) => {
                // Log warning and use zero for corrupted total supply
                #[cfg(debug_assertions)]
                crate::runtime::log(ByteString::from_literal("Warning: Corrupted total supply data"));
                Int256::zero()
            }
        }
    };

    let new_total_supply = total_supply.checked_add(&amount);
    if new_total_supply.is_negative() {
        runtime::abort();
        return;
    }

    storage.put(key, new_total_supply.into_byte_string());
}

/// Safe ByteString to Int256 conversion with error handling
fn try_safe_from_byte_string(value: crate::types::builtin::nullable::Nullable<ByteString>) -> Result<Int256> {
    if value.is_null() {
        return Ok(Int256::zero());
    }
    
    // Extract the value using unwrap_or for fallback
    let byte_string = value.unwrap_or(ByteString::empty());
    
    // Validate byte string length
    if byte_string.as_bytes().len() > Int256::SIZE {
        return Err(ContractError::InvalidArgument);
    }
    
    // Handle empty case
    if byte_string.is_empty() {
        return Ok(Int256::zero());
    }
    
    // Safe conversion
    Ok(Int256::from_byte_string(byte_string))
}
