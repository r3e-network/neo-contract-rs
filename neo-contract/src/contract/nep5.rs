// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::vec::Vec;
use crate::runtime;
use crate::storage::map::StorageMap;
use crate::types::builtin::h160::H160;
use crate::types::builtin::int256::Int256;
use crate::types::builtin::string::ByteString;
use crate::types::consts::*;

/// NEP5 represents a NEP5 token (legacy)
pub struct NEP5;

impl NEP5 {
    /// Get the balance of the specified address
    pub fn balance_of(account: H160) -> Int256 {
        let key = account.to_hex_string();
        let context = crate::types::context::StorageContext::new();
        StorageMap::new(context, Vec::from([DEFAULT_BALANCE_KEY])).get(&key)
            .unwrap_or_else(Int256::zero)
    }

    /// Transfer tokens from one address to another
    pub fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        if amount <= Int256::zero() {
            return false;
        }

        if !runtime::check_witness(&from) {
            return false;
        }

        let from_balance = Self::balance_of(from);
        if from_balance < amount {
            return false;
        }

        let from_key = from.to_hex_string();
        let to_key = to.to_hex_string();
        let context = crate::types::context::StorageContext::new();
        let balance_map = StorageMap::new(context, Vec::from([DEFAULT_BALANCE_KEY]));

        if from_balance == amount {
            balance_map.delete(&from_key);
        } else {
            balance_map.put(&from_key, from_balance - amount);
        }

        let to_balance = Self::balance_of(to);
        balance_map.put(&to_key, to_balance + amount);

        // Emit transfer event
        runtime::notify("Transfer", &[]);

        true
    }

    /// Get the total supply of the token
    pub fn total_supply() -> Int256 {
        let context = crate::types::context::StorageContext::new();
        StorageMap::new(context, Vec::from([DEFAULT_TOTAL_SUPPLY_KEY])).get("")
            .unwrap_or_else(Int256::zero)
    }

    /// Get the decimals of the token
    pub fn decimals() -> u8 {
        let context = crate::types::context::StorageContext::new();
        StorageMap::new(context, Vec::from([DEFAULT_DECIMALS_KEY])).get("")
            .map(|i: Int256| i.to_u8())
            .unwrap_or(DEFAULT_DECIMALS)
    }

    /// Get the symbol of the token
    pub fn symbol() -> ByteString {
        let context = crate::types::context::StorageContext::new();
        StorageMap::new(context, Vec::from([DEFAULT_SYMBOL_KEY])).get("")
            .unwrap_or_else(|| ByteString::from(DEFAULT_SYMBOL))
    }
}
