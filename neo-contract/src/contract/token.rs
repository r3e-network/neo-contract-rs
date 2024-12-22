// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::contract::{SmartContract, BALANCE_KEY, TOTAL_SUPPLY_KEY};

#[allow(unused_imports)]
use crate::{env, storage::StorageMap, types::*};

pub trait TokenContract: SmartContract {
    fn symbol() -> ByteString;

    fn decimals() -> u32;

    fn total_supply() -> Int256 {
        #[cfg(target_family = "wasm")]
        let key = unsafe { env::extension::concat_u8_byte_string(TOTAL_SUPPLY_KEY, ByteString::empty()) };

        #[cfg(not(target_family = "wasm"))]
        let key = ByteString::with_bytes(&[TOTAL_SUPPLY_KEY]);

        let storage = StorageMap::new();
        let value = storage.get(key.clone());
        if value.is_null() {
            Int256::zero()
        } else {
            Int256::from_byte_string(value.unwrap())
        }
    }

    fn balance_of(account: H160) -> Int256 {
        #[cfg(target_family = "wasm")]
        let key = unsafe { env::extension::concat_u8_byte_string(BALANCE_KEY, account.into_byte_string()) };

        #[cfg(not(target_family = "wasm"))]
        let key = ByteString::with_bytes(&[BALANCE_KEY]).concat(&account.into_byte_string());

        let storage = StorageMap::new();
        let value = storage.get(key.clone());
        if value.is_null() {
            Int256::zero()
        } else {
            Int256::from_byte_string(value.unwrap())
        }
    }
}
