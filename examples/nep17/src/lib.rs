// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

extern crate alloc;

use neo_contract::{
    builtin::{H160, Int256, ByteString, Map, Array, Any},
    Runtime,
    contract::nep17::NEP17,
};
use alloc::vec::Vec;

// Storage for the token contract
struct TokenStorage {
    total_supply: Int256,
    balances: Map,
}

static mut TOKEN_STORAGE: Option<TokenStorage> = None;

// Initialize the token contract
#[no_mangle]
pub fn _deploy(initial_supply: Int256) {
    let mut balances = Map::new();
    let owner = Runtime::calling_script_hash();
    
    balances.put(owner.clone(), initial_supply.clone());
    
    unsafe {
        TOKEN_STORAGE = Some(TokenStorage {
            total_supply: initial_supply,
            balances,
        });
    }
}

// Get the token symbol
#[no_mangle]
pub fn symbol() -> ByteString {
    ByteString::from("NEP17")
}

// Get the token decimals
#[no_mangle]
pub fn decimals() -> u8 {
    8
}

// Get the total supply of tokens
#[no_mangle]
pub fn total_supply() -> Int256 {
    unsafe {
        match &TOKEN_STORAGE {
            Some(storage) => storage.total_supply.clone(),
            None => Int256::zero(),
        }
    }
}

// Get the balance of an account
#[no_mangle]
pub fn balance_of(account: H160) -> Int256 {
    unsafe {
        match &TOKEN_STORAGE {
            Some(storage) => {
                // Map doesn't have a get method in the current implementation
                // We need to iterate through the keys and values
                for i in 0..storage.balances.len() {
                    if let Some(key) = storage.balances.keys.get(i) {
                        if *key == account {
                            if let Some(value) = storage.balances.values.get(i) {
                                return value.clone();
                            }
                        }
                    }
                }
                Int256::zero()
            },
            None => Int256::zero(),
        }
    }
}

// Transfer tokens from one account to another
#[no_mangle]
pub fn transfer(from: H160, to: H160, amount: Int256, data: Option<Any>) -> bool {
    if !Runtime::check_witness(from.clone()) {
        return false;
    }
    
    if amount <= Int256::zero() {
        return false;
    }
    
    let from_balance = balance_of(from.clone());
    if from_balance < amount {
        return false;
    }
    
    unsafe {
        if let Some(storage) = &mut TOKEN_STORAGE {
            if from != to {
                let from_new_balance = from_balance - amount.clone();
                if from_new_balance.is_zero() {
                    storage.balances.delete(&from);
                } else {
                    storage.balances.put(from.clone(), from_new_balance);
                }
                
                let to_balance = balance_of(to.clone());
                let to_new_balance = to_balance + amount.clone();
                storage.balances.put(to.clone(), to_new_balance);
            }
        } else {
            return false;
        }
    }
    
    // Emit transfer event
    let mut args = Array::new();
    args.push(Any::from(from));
    args.push(Any::from(to));
    args.push(Any::from(amount));
    
    Runtime::notify(
        &ByteString::from("Transfer"),
        &args
    );
    
    true
}
