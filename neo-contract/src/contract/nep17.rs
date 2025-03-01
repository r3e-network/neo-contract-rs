// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::{contract::*, runtime, storage::StorageMap, types::*};

pub const DEFAULT_TOTAL_SUPPLY_KEY: u8 = 0x00;
pub const DEFAULT_BALANCE_KEY: u8 = 0x01;

pub trait Nep17Token: TokenContract {
    fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        if amount.is_negative() {
            runtime::abort();
            return false; // unreachable
        }

        if !runtime::check_witness_with_account(from) {
            return false;
        }

        if amount.is_positive() {
            let from_balance = Self::balance_of(from);
            if from_balance < amount {
                return false;
            }
            
            // Use copied bytes instead of chaining iterators
            let mut from_key = Vec::with_capacity(21);
            from_key.push(DEFAULT_BALANCE_KEY);
            from_key.extend_from_slice(from.as_bytes());
            
            let mut to_key = Vec::with_capacity(21);
            to_key.push(DEFAULT_BALANCE_KEY);
            to_key.extend_from_slice(to.as_bytes());
            
            let storage = StorageMap::new();
            
            // Update from balance
            let from_balance = storage.get::<Int256>(&from_key).unwrap_or_default();
            let new_from_balance = from_balance - amount;
            if new_from_balance.is_zero() {
                storage.delete(&from_key);
            } else {
                storage.put(&from_key, &new_from_balance);
            }
            
            // Update to balance
            let to_balance = storage.get::<Int256>(&to_key).unwrap_or_default();
            let new_to_balance = to_balance + amount;
            storage.put(&to_key, &new_to_balance);
        }

        return true;
    }

    // fn transfer_with_data(from: H160, to: H160, amount: Int256, data: Any) -> bool;

    fn mint(account: H160, amount: Int256) {
        if amount.is_negative() {
            runtime::abort();
            return;
        }
        
        // Update total supply
        let storage = StorageMap::new();
        let total_supply_key = [DEFAULT_TOTAL_SUPPLY_KEY].to_vec();
        let current_supply = storage.get::<Int256>(&total_supply_key).unwrap_or_default();
        let new_supply = current_supply + amount;
        storage.put(&total_supply_key, &new_supply);
        
        // Update account balance
        update_nep17_balance::<DEFAULT_BALANCE_KEY>(account, amount);
    }

    fn burn(account: H160, amount: Int256) {
        if amount.is_negative() {
            runtime::abort();
            return;
        }
        
        // Check balance
        let balance = Self::balance_of(account);
        if balance < amount {
            runtime::abort();
            return;
        }
        
        // Update total supply
        let storage = StorageMap::new();
        let total_supply_key = [DEFAULT_TOTAL_SUPPLY_KEY].to_vec();
        let current_supply = storage.get::<Int256>(&total_supply_key).unwrap_or_default();
        let new_supply = current_supply - amount;
        storage.put(&total_supply_key, &new_supply);
        
        // Update account balance
        update_nep17_balance::<DEFAULT_BALANCE_KEY>(account, -amount);
    }
}

pub fn update_nep17_balance<const BALANCE_PREFIX: u8>(account: H160, amount: Int256) {
    let storage = StorageMap::new();
    
    // Create key directly instead of using chain
    let mut key = Vec::with_capacity(21);
    key.push(BALANCE_PREFIX);
    key.extend_from_slice(account.as_bytes());
    
    let current_balance = storage.get::<Int256>(&key).unwrap_or_default();
    let new_balance = current_balance + amount;
    
    if new_balance.is_zero() {
        storage.delete(&key);
    } else {
        storage.put(&key, &new_balance);
    }
}
