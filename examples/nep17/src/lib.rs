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
    contract_version,
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
#[contract_description("NEP-17 token example")]
#[contract_version("0.1.0")]
mod token {
    use super::*;

    // Helper function to emit a Transfer event
    pub fn emit_transfer(from: Option<H160>, to: Option<H160>, amount: Int256) {
        // Create event name as ByteString
        let event_name = ByteString::from("Transfer");
        
        // Create an Array to hold parameters
        let mut event_data = Array::<Any>::new();
        
        // Add parameters as Any values
        match from {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()),
        }
        
        match to {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()),
        }
        
        event_data.push(Any::from(amount));
        
        // Emit the event
        Runtime::notify(&event_name, &event_data);
    }

    #[storage]
    pub struct Token {
        total: Int256,  
        balances: Map<H160, Int256>,
        token_name: ByteString,      
        token_symbol: ByteString,    
        token_decimals: Int256,      
    }

    impl Token {
        #[constructor]
        pub fn new(owner: H160, total_supply: Int256) -> Self {
            let mut balances = Map::new();
            
            // Store the total_supply to owner's balance
            balances.put(owner.clone(), total_supply.clone());
            
            // Emit transfer event from null address to owner
            emit_transfer(None, Some(owner), total_supply.clone());
            
            Self {
                total: total_supply,  
                balances,
                token_name: ByteString::from("NEP17 Token"),
                token_symbol: ByteString::from("NEP"),
                token_decimals: Int256::from(8),
            }
        }

        // NEP-17 methods
        #[message]
        #[safe]
        pub fn name(&self) -> ByteString {
            self.token_name.clone()
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
        pub fn transfer(&mut self, from: H160, to: H160, amount: Int256, _data: ByteString) -> bool {
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
            
            // Emit transfer event
            emit_transfer(Some(from), Some(to), amount);
            
            true
        }
    }
}
