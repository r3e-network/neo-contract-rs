// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

extern crate alloc;

use neo_contract::{
    builtin::{H160, Int256, ByteString, Map, Any, Array},
    Runtime,
    contract, storage, constructor, message, event,
};
use alloc::vec::Vec;

#[contract]
mod token {
    use super::*;
    
    #[storage]
    pub struct Token {
        total_supply: Int256,
        balances: Map,
    }
    
    impl Token {
        #[constructor]
        pub fn new(initial_supply: Int256) -> Self {
            let mut balances = Map::new();
            let owner = Runtime::calling_script_hash();
            
            balances.put(owner.clone(), initial_supply.clone());
            
            Self {
                total_supply: initial_supply,
                balances,
            }
        }
        
        #[message]
        pub fn total_supply(&self) -> Int256 {
            self.total_supply.clone()
        }
        
        #[message]
        pub fn balance_of(&self, account: H160) -> Int256 {
            // Map doesn't have a get method in the current implementation
            // We need to iterate through the keys and values
            for i in 0..self.balances.len() {
                if let Some(key) = self.balances.keys.get(i) {
                    if *key == account {
                        if let Some(value) = self.balances.values.get(i) {
                            return value.clone();
                        }
                    }
                }
            }
            Int256::zero()
        }
        
        #[event]
        pub fn transfer(from: Option<H160>, to: Option<H160>, amount: Int256) {}
    }
}
