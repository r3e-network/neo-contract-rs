// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, runtime, storage::StorageMap, types::*};

#[inline(always)]
pub(crate) fn prefixed_key<const PREFIX: u8>(key: ByteString) -> ByteString {
    ByteString::one_byte::<PREFIX>().concat(key)
}

pub(crate) fn total_supply<const KEY: u8>() -> Int256 {
    let key = ByteString::one_byte::<KEY>();
    let storage = StorageMap::new();
    let value = storage.get(key.clone());
    if value.is_null() {
        Int256::zero()
    } else {
        Int256::from_byte_string(unsafe { value.unwrap_unchecked() })
    }
}

// It must be inline becaue it has a reference argument.
// Otherwise, the compiled wasm ops cannot transfer to neo ops.
#[inline(always)]
pub(crate) fn update_total_supply<const KEY: u8>(storage: &mut StorageMap, amount: Int256) {
    let key = ByteString::one_byte::<KEY>();
    let value = storage.get(key.clone());
    let total_supply = if value.is_null() {
        Int256::zero()
    } else {
        Int256::from_byte_string(unsafe { value.unwrap_unchecked() })
    };

    let new_total_supply = total_supply.checked_add(&amount);
    if new_total_supply.is_negative() {
        runtime::abort(/* TODO: add message */);
        // return; // unreachable
    }

    storage.put(key, new_total_supply.into_byte_string());
}


pub(crate) fn balance_of<const PREFIX: u8>(account: H160) -> Int256 {
    let key = prefixed_key::<PREFIX>(account.into_byte_string());
    let storage = StorageMap::new();
    let value = storage.get(key.clone());
    if value.is_null() {
        Int256::zero()
    } else {
        Int256::from_byte_string(unsafe { value.unwrap_unchecked() })
    }
}

// It must be inline becaue it has a reference argument.
// Otherwise, the compiled wasm ops cannot transfer to neo ops.
#[inline(always)]
pub(crate) fn update_balance<const PREFIX: u8>(storage: &mut StorageMap, account: H160, amount: Int256) -> bool {
    let key = prefixed_key::<PREFIX>(account.into_byte_string());
    let value = storage.get(key.clone());
    let balance = if value.is_null() {
        Int256::zero()
    } else {
        Int256::from_byte_string(unsafe { value.unwrap_unchecked() })
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
