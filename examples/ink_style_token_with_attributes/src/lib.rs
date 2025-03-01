// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

extern crate alloc;

use neo_contract::{
    builtin::{H160, Int256, ByteString, Map, Any},
    Runtime,
    contract, contract_author, contract_email, contract_description,
    contract_version, contract_source_code, contract_permission,
    contract_trust, supported_standards,
    storage, constructor, message, event,
};
use alloc::vec::Vec;

#[contract]
#[contract_author("R3E Network")]
#[contract_email("dev@r3e.network")]
#[contract_description("An example token contract using ink!-style attributes")]
#[contract_version("0.1.0")]
#[contract_source_code("https://github.com/R3E-Network/neo-contract-rs")]
#[contract_permission("*", "*")]
#[contract_trust("*")]
#[supported_standards("NEP-17")]
mod token {
    use super::*;

    #[storage]
    pub struct Token {
        total_supply: Int256,
        balances: Map,
        symbol: ByteString,
        decimals: Int256,
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
                symbol: ByteString::from("TKN"),
                decimals: Int256::from(8),
            }
        }

        #[message]
        pub fn symbol(&self) -> ByteString {
            self.symbol.clone()
        }

        #[message]
        pub fn decimals(&self) -> Int256 {
            self.decimals.clone()
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

        #[message]
        pub fn transfer(&mut self, from: H160, to: H160, amount: Int256, data: Option<Any>) -> bool {
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
            
            self.transfer_event(from, to, amount);
            
            true
        }

        #[event]
        pub fn transfer_event(&self, from: H160, to: H160, amount: Int256) {}
    }
}
