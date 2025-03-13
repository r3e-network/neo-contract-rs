// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

use neo_contract::{
    builtin::{H160, Int256, ByteString, Map, Array, Any},
    Runtime,
    contract, contract_author, contract_description,
    contract_version, contract_email,
    storage, constructor, message,
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

#[contract]
#[contract_author("R3E Network")]
#[contract_email("dev@r3e.network")]
#[contract_description("An example contract using C# features")]
#[contract_version("0.1.0")]
mod token_contract {
    use super::*;

    pub fn emit_transfer(from: H160, to: H160, amount: Int256) {
        // Create event name as ByteString
        let event_name = ByteString::from("transfer");
        
        // Create an Array to hold our parameters
        let mut event_data = Array::<Any>::new();
        
        // Add the parameters as Any values
        event_data.push(Any::from(from));
        event_data.push(Any::from(to));
        event_data.push(Any::from(amount));
        
        // Emit the event
        Runtime::notify(&event_name, &event_data);
    }
    
    #[storage]
    pub struct Token {
        // Total supply of tokens
        token_supply: Int256,
        // Map of account balances
        balances: Map<H160, Int256>,
    }

    impl Token {
        // Constructor
        #[constructor]
        pub fn new(initial_supply: Int256) -> Self {
            let owner = Runtime::executing_script_hash();
            
            let mut balances = Map::new();
            balances.put(owner, initial_supply);
            
            Self {
                token_supply: initial_supply,
                balances,
            }
        }
        
        // Safe method that doesn't modify state
        #[safe]
        pub fn total_supply(&self) -> Int256 {
            self.token_supply
        }
        
        // Method with transfer functionality
        #[message]
        pub fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
            if !Runtime::check_witness(from) {
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
                    self.balances.delete(&from);
                } else {
                    self.balances.put(from, from_new_balance);
                }
                
                let to_balance = self.balance_of(to);
                let to_new_balance = to_balance + amount;
                self.balances.put(to, to_new_balance);
            }
            
            // Emit the transfer event
            emit_transfer(from, to, amount);
            
            true
        }
        
        // Method with withdraw functionality
        #[message]
        pub fn withdraw(&mut self, account: H160, amount: Int256) -> bool {
            if !Runtime::check_witness(account) {
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
                self.balances.delete(&account);
            } else {
                self.balances.put(account, new_balance);
            }
            
            // Emit withdraw event (same as transfer from account to 0)
            emit_transfer(account, H160::zero(), amount);
            
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
            match self.balances.get(&account) {
                Some(balance) => *balance,
                None => Int256::zero(),
            }
        }
    }
}
