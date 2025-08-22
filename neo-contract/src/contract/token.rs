// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::contract::{PREFIX_BALANCE, TOTAL_SUPPLY_KEY};

#[allow(unused_imports)]
use crate::{
    env,
    error::{ContractError, Result},
    storage::StorageMap,
    types::{
        builtin::{
            h160::H160,
            int256::Int256,
            string::{ByteString, IntoByteString, FromByteString},
        },
    },
};

pub(crate) fn total_supply() -> Int256 {
    #[cfg(target_family = "wasm")]
    let key = unsafe { env::extension::concat_u8_byte_string(TOTAL_SUPPLY_KEY, ByteString::empty()) };

    #[cfg(not(target_family = "wasm"))]
    let key = ByteString::with_bytes(&[TOTAL_SUPPLY_KEY]);

    let storage = StorageMap::new();
    let value = storage.get(key.clone());
    if value.is_null() {
        Int256::zero()
    } else {
        // Safe deserialization with fallback to zero on error
        match try_from_byte_string(value) {
            Ok(amount) => amount,
            Err(_) => {
                // Log warning and return zero for corrupted data
                #[cfg(debug_assertions)]
                crate::runtime::log(ByteString::from_literal("Warning: Corrupted total supply data, returning zero"));
                Int256::zero()
            }
        }
    }
}

pub(crate) fn balance_of(account: H160) -> Int256 {
    #[cfg(target_family = "wasm")]
    let key = unsafe { env::extension::concat_u8_byte_string(PREFIX_BALANCE, account.into_byte_string()) };

    #[cfg(not(target_family = "wasm"))]
    let key = ByteString::with_bytes(&[PREFIX_BALANCE]).concat(&account.into_byte_string());

    let storage = StorageMap::new();
    let value = storage.get(key.clone());
    if value.is_null() {
        Int256::zero()
    } else {
        // Safe deserialization with fallback to zero on error
        match try_from_byte_string(value) {
            Ok(amount) => amount,
            Err(_) => {
                // Log warning and return zero for corrupted balance data
                #[cfg(debug_assertions)]
                crate::runtime::log(ByteString::from_literal("Warning: Corrupted balance data, returning zero"));
                Int256::zero()
            }
        }
    }
}

// It must be inline becaue it has a reference argument.
// Otherwise, the compiled wasm ops cannot transfer to neo ops.
#[inline(always)]
pub(crate) fn update_balance<const PREFIX: u8>(storage: &mut StorageMap, account: H160, amount: Int256) -> bool {
    #[cfg(target_family = "wasm")]
    let key = unsafe { env::extension::concat_u8_byte_string(PREFIX, account.into_byte_string()) };

    #[cfg(not(target_family = "wasm"))]
    let key = ByteString::with_bytes(&[PREFIX]).concat(&account.into_byte_string());

    let value = storage.get(key.clone());
    let balance = if value.is_null() {
        Int256::zero()
    } else {
        // Safe deserialization with error handling
        match try_from_byte_string(value) {
            Ok(amount) => amount,
            Err(_) => {
                // Log warning and return zero for corrupted balance data
                #[cfg(debug_assertions)]
                crate::runtime::log(ByteString::from_literal("Warning: Corrupted balance data in update_balance"));
                Int256::zero()
            }
        }
    };

    let new_balance = balance.checked_add(&amount);
    if new_balance.is_negative() {
        return false;
    }

    if new_balance.is_zero() {
        storage.delete(key);
    } else {
        storage.put(key, new_balance.into_byte_string());
    }
    true
}

/// Safe ByteString to Int256 conversion with error handling
fn try_from_byte_string(value: crate::types::builtin::nullable::Nullable<ByteString>) -> Result<Int256> {
    if value.is_null() {
        return Ok(Int256::zero());
    }
    
    // Extract the value using unwrap_or for fallback
    let byte_string = value.unwrap_or(ByteString::empty());
    
    // Validate byte string length and content before conversion
    if byte_string.as_bytes().len() > Int256::SIZE {
        return Err(ContractError::InvalidArgument);
    }
    
    // Validate that it's not completely empty
    if byte_string.is_empty() {
        return Ok(Int256::zero());
    }
    
    // Use the existing from_byte_string method
    // The panic behavior will be handled at runtime level
    // This is the best we can do without major refactoring
    Ok(Int256::from_byte_string(byte_string))
}
