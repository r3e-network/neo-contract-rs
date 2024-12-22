// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::{contract::*, runtime, storage::StorageMap, types::*};

/// Default total supply key.
/// Do not change the default TOTAL_SUPPLY_KEY value if really necessary.
pub const TOTAL_SUPPLY_KEY: u8 = 0x00;

/// Default balance key prefix.
/// Do not change the default BALANCE_KEY value if really necessary.
pub const BALANCE_KEY: u8 = 0x01;

pub trait Nep17Token: TokenContract {
    fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        if amount.is_negative() {
            runtime::abort();
            return false;
        }

        if runtime::check_witness_with_account(from) {
            return false;
        }

        if amount.is_positive() {
            if !update_nep17_balance::<BALANCE_KEY>(from, amount.checked_neg()) {
                return false;
            }
            let _ = update_nep17_balance::<BALANCE_KEY>(to, amount);
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

        let _ = update_nep17_balance::<BALANCE_KEY>(account, amount);
        update_nep17_total_supply::<TOTAL_SUPPLY_KEY>(amount);
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
        let _ = update_nep17_balance::<BALANCE_KEY>(account, burned);
        update_nep17_total_supply::<TOTAL_SUPPLY_KEY>(burned);
    }
}

pub fn update_nep17_balance<const PREFIX: u8>(account: H160, amount: Int256) -> bool {
    #[cfg(target_family = "wasm")]
    let key = unsafe { env::extension::concat_u8_byte_string(PREFIX, account.into_byte_string()) };

    #[cfg(not(target_family = "wasm"))]
    let key = ByteString::with_bytes(&[PREFIX]).concat(&account.into_byte_string());

    let mut storage = StorageMap::new();
    let value = storage.get(key.clone());
    let balance = if value.is_null() {
        Int256::zero()
    } else {
        Int256::from_byte_string(value.unwrap())
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

pub fn update_nep17_total_supply<const KEY: u8>(amount: Int256) {
    #[cfg(target_family = "wasm")]
    let key = unsafe { env::extension::concat_u8_byte_string(KEY, ByteString::empty()) };

    #[cfg(not(target_family = "wasm"))]
    let key = ByteString::with_bytes(&[KEY]);

    let mut storage = StorageMap::new();
    let value = storage.get(key.clone());
    let total_supply = if value.is_null() {
        Int256::zero()
    } else {
        Int256::from_byte_string(value.unwrap())
    };

    let new_total_supply = total_supply.checked_add(&amount);
    if new_total_supply.is_negative() {
        runtime::abort();
        return;
    }

    storage.put(key, new_total_supply.into_byte_string());
}
