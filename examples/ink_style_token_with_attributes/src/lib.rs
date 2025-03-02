// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

use neo_contract::{
    builtin::{H160, Int256, ByteString, Map, Array, Any},
    Runtime,
    contract, contract_author, contract_email, contract_description,
    contract_version, contract_permission,
    contract_trust, supported_standards,
    storage, constructor, message, safe,
};
use alloc::vec::Vec;
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
#[contract_description("An example token contract using ink!-style attributes")]
#[contract_version("0.1.0")]
// Removed contract_source_code attribute
#[contract_permission("*", "*")]
#[contract_trust("*")]
#[supported_standards("NEP-17")]
mod token {
    use super::*;

    // Helper function to emit a Transfer event
    pub fn emit_transfer(from: H160, to: H160, amount: Int256) {
        // Create event name as ByteString
        let event_name = ByteString::from("Transfer");
        
        // Create an Array to hold our parameters
        let mut event_data = Array::<Any>::new();
        
        // Add parameters as Any values
        event_data.push(Any::from(from));
        event_data.push(Any::from(to));
        event_data.push(Any::from(amount));
        
        // Emit the event
        Runtime::notify(&event_name, &event_data);
    }

    #[storage]
    pub struct Token {
        total: Int256, 
        balances: Map<H160, Int256>,
        token_symbol: ByteString, 
        token_decimals: Int256, 
    }

    impl Token {
        #[constructor]
        pub fn new(initial_supply: Int256) -> Self {
            let mut balances = Map::new();
            let owner = Runtime::calling_script_hash();
            
            balances.put(owner.clone(), initial_supply.clone());
            
            Self {
                total: initial_supply,
                balances,
                token_symbol: ByteString::from("TKN"),
                token_decimals: Int256::from(8),
            }
        }

        #[message]
        #[safe]
        pub fn symbol(&self) -> ByteString {
            self.token_symbol.clone()
        }

        #[message]
        #[safe]
        pub fn decimals(&self) -> Int256 {
            self.token_decimals.clone()
        }

        #[message]
        #[safe]
        pub fn total_supply(&self) -> Int256 {
            self.total.clone()
        }

        #[message]
        #[safe]
        pub fn balance_of(&self, account: H160) -> Int256 {
            match self.balances.get(&account) {
                Some(balance) => balance.clone(),
                None => Int256::zero(),
            }
        }

        #[message]
        pub fn transfer(&mut self, from: H160, to: H160, amount: Int256, _data: Option<Any>) -> bool {
            if !Runtime::check_witness(from.clone()) {
                return false;
            }
            
            if amount <= Int256::zero() {
                return false;
            }
            
            let from_balance = self.balance_of(from.clone());
            if from_balance < amount {
                return false;
            }
            
            if from != to {
                let from_new_balance = from_balance - amount.clone();
                if from_new_balance.is_zero() {
                    self.balances.delete(&from);
                } else {
                    self.balances.put(from.clone(), from_new_balance);
                }
                
                let to_balance = self.balance_of(to.clone());
                let to_new_balance = to_balance + amount.clone();
                self.balances.put(to.clone(), to_new_balance);
            }
            
            // Call emit_transfer instead of using the event attribute
            emit_transfer(from, to, amount);
            
            true
        }
    }
}
