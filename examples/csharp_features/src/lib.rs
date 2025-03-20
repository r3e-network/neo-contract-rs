// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

use neo_contract::{
    builtin::{H160, Int256, ByteString, Map, Array, Any},
    Runtime,
    prelude::*,
};
use core::panic::PanicInfo;

// Use wee_alloc as the global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Define a panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Define the Transfer event
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: H160,
    #[index]
    pub to: H160,
    pub amount: Int256,
}

#[neo_contract::contract]
#[contract_author("R3E Network")]
#[contract_email("dev@r3e.network")]
#[contract_description("An example contract using C# features")]
#[contract_version("0.1.0")]
pub struct Token {
    // Total supply of tokens
    #[storage]
    token_supply: StorageItem<Int256>,
    // Map of account balances
    #[storage]
    balances: StorageMap<H160, Int256>,
}

impl Token {
    // Constructor
    #[constructor]
    pub fn new(initial_supply: Int256) -> Self {
        let owner = Runtime::executing_script_hash();
        
        let mut instance = Self {
            token_supply: StorageItem::new(b"token_supply"),
            balances: StorageMap::new(b"balances"),
        };
        
        instance.balances.insert(owner, initial_supply);
        instance.token_supply.set(&initial_supply);
        
        // Emit transfer event for minting
        Transfer {
            from: H160::zero(),
            to: owner,
            amount: initial_supply
        }.notify();
        
        instance
    }
    
    // Safe method that doesn't modify state
    #[safe]
    pub fn total_supply(&self) -> Int256 {
        self.token_supply.get().unwrap_or_default()
    }
    
    // Method with transfer functionality
    #[method]
    #[no_reentry]
    pub fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
        if !Runtime::check_witness(&from) {
            return false;
        }
        
        if amount <= Int256::zero() {
            return false;
        }
        
        let from_balance = self.balance_of(from);
        if from_balance < amount {
            return false;
        }
        
        if from != to {
            let from_new_balance = from_balance - amount;
            if from_new_balance.is_zero() {
                self.balances.remove(&from);
            } else {
                self.balances.insert(from, from_new_balance);
            }
            
            let to_balance = self.balance_of(to);
            let to_new_balance = to_balance + amount;
            self.balances.insert(to, to_new_balance);
        }
        
        // Emit the transfer event
        Transfer {
            from: from,
            to: to,
            amount: amount
        }.notify();
        
        true
    }
    
    // Method with withdraw functionality
    #[method]
    #[no_reentry]
    pub fn withdraw(&mut self, account: H160, amount: Int256) -> bool {
        if !Runtime::check_witness(&account) {
            return false;
        }
        
        if amount <= Int256::zero() {
            return false;
        }
        
        let balance = self.balance_of(account);
        if balance < amount {
            return false;
        }
        
        let new_balance = balance - amount;
        if new_balance.is_zero() {
            self.balances.remove(&account);
        } else {
            self.balances.insert(account, new_balance);
        }
        
        // Emit withdraw event (same as transfer from account to 0)
        Transfer {
            from: account,
            to: H160::zero(),
            amount: amount
        }.notify();
        
        true
    }
    
    // Call other contract
    #[safe]
    pub fn call_other_contract(&self, contract_hash: H160, method: ByteString, args: Array<Any>) -> Any {
        // Call contract without flags
        Runtime::call_contract(contract_hash, method, args)
    }
    
    // Safe method to check balance
    #[safe]
    pub fn balance_of(&self, account: H160) -> Int256 {
        self.balances.get(&account).unwrap_or_default()
    }
}
