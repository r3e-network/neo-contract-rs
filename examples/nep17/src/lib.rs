// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![no_std]
#![no_main]

use neo_contract as neo;

use neo::contract::nep17::NEP17;
use neo::contract::SmartContract;
use neo::runtime;
use neo::types::*;

// Storage for the token contract
struct TokenStorage {
    total_supply: Int256,
    balances: builtin::Map,
}

static mut TOKEN_STORAGE: Option<TokenStorage> = None;

// Initialize the token contract
#[no_mangle]
pub fn _deploy(initial_supply: Int256) {
    let mut balances = builtin::Map::new();
    let owner = runtime::calling_script_hash();
    
    balances.put(&owner, &initial_supply);
    
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
    ByteString::new("NEP17".to_string())
}

// Get the token decimals
#[no_mangle]
pub fn decimals() -> Int256 {
    Int256::from(8)
}

// Get the total supply of tokens
#[no_mangle]
pub fn total_supply() -> Int256 {
    unsafe {
        match &TOKEN_STORAGE {
            Some(storage) => storage.total_supply,
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
                match storage.balances.get(&account) {
                    Some(balance) => balance,
                    None => Int256::zero(),
                }
            },
            None => Int256::zero(),
        }
    }
}

// Transfer tokens from one account to another
#[no_mangle]
pub fn transfer(from: H160, to: H160, amount: Int256, data: Option<Any>) -> bool {
    if !runtime::check_witness_with_account(from) {
        return false;
    }
    
    if amount <= Int256::zero() {
        return false;
    }
    
    let from_balance = balance_of(from);
    if from_balance < amount {
        return false;
    }
    
    unsafe {
        if let Some(storage) = &mut TOKEN_STORAGE {
            if from != to {
                let from_new_balance = from_balance - amount;
                if from_new_balance.is_zero() {
                    storage.balances.delete(&from);
                } else {
                    storage.balances.put(&from, &from_new_balance);
                }
                
                let to_balance = balance_of(to);
                let to_new_balance = to_balance + amount;
                storage.balances.put(&to, &to_new_balance);
            }
        } else {
            return false;
        }
    }
    
    // Emit transfer event
    let args = builtin::Array::new();
    args.push(from.into());
    args.push(to.into());
    args.push(amount.into());
    
    unsafe {
        neo::env::syscall::system_runtime_notify(
            ByteString::new("Transfer".to_string()),
            args
        );
    }
    
    true
}
