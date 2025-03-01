// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![no_std]
#![no_main]

use neo_contract as neo;

#[neo::contract]
#[neo::contract_author("R3E Network")]
#[neo::contract_email("dev@r3e.network")]
#[neo::contract_description("An example token contract using ink!-style attributes")]
#[neo::contract_version("0.1.0")]
#[neo::contract_source_code("https://github.com/R3E-Network/neo-contract-rs")]
#[neo::contract_permission("*", "*")]
#[neo::contract_trust("*")]
#[neo::supported_standards("NEP-17")]
mod token {
    use neo::prelude::*;
    use neo::types::*;
    use neo::runtime;
    use neo::storage::StorageMap;

    #[neo(storage)]
    pub struct Token {
        total_supply: Int256,
        balances: builtin::Map,
        symbol: ByteString,
        decimals: Int256,
    }

    impl Token {
        #[neo(constructor)]
        pub fn new(initial_supply: Int256) -> Self {
            let mut balances = builtin::Map::new();
            let owner = runtime::calling_script_hash();
            
            balances.put(&owner, &initial_supply);
            
            Self {
                total_supply: initial_supply,
                balances,
                symbol: ByteString::new("TKN"),
                decimals: Int256::from(8),
            }
        }

        #[neo(message)]
        pub fn symbol(&self) -> ByteString {
            self.symbol.clone()
        }

        #[neo(message)]
        pub fn decimals(&self) -> Int256 {
            self.decimals
        }

        #[neo(message)]
        pub fn total_supply(&self) -> Int256 {
            self.total_supply
        }

        #[neo(message)]
        pub fn balance_of(&self, account: H160) -> Int256 {
            match self.balances.get(&account) {
                Some(balance) => balance,
                None => Int256::zero(),
            }
        }

        #[neo(message)]
        pub fn transfer(&mut self, from: H160, to: H160, amount: Int256, data: Option<Any>) -> bool {
            if !runtime::check_witness_with_account(from) {
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
                    self.balances.put(&from, &from_new_balance);
                }
                
                let to_balance = self.balance_of(to);
                let to_new_balance = to_balance + amount;
                self.balances.put(&to, &to_new_balance);
            }
            
            self.transfer_event(from, to, amount);
            
            true
        }

        #[neo(event)]
        pub fn transfer_event(&self, from: H160, to: H160, amount: Int256) {}
    }
}
