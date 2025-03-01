// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![no_std]
#![no_main]

use neo_contract as neo;

#[neo::contract]
#[neo::contract_author("R3E Network")]
#[neo::contract_email("dev@r3e.network")]
#[neo::contract_description("An example contract using C# features")]
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
    use neo::call_flags::CallFlags;

    // Static field initialization
    #[neo::byte_array("0123456789ABCDEF")]
    static BYTE_ARRAY: [u8; 8] = [0; 8];

    #[neo::hash160("0x0123456789abcdef0123456789abcdef01234567")]
    static HASH160: H160 = H160::zero();

    #[neo::integer("1000000")]
    static AMOUNT: Int256 = Int256::zero();

    #[neo::public_key("03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c")]
    static PUBLIC_KEY: [u8; 33] = [0; 33];

    #[neo::string("Hello, NEO!")]
    static GREETING: &str = "";

    #[neo::contract_hash("0x0123456789abcdef0123456789abcdef01234567")]
    static CONTRACT_HASH: H160 = H160::zero();

    #[neo(storage)]
    pub struct Token {
        total_supply: Int256,
        balances: builtin::Map,
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
            }
        }

        // Safe method that doesn't modify state
        #[neo::safe]
        #[neo(message)]
        pub fn total_supply(&self) -> Int256 {
            self.total_supply
        }

        // Method with reentrancy protection
        #[neo::no_reentrant]
        #[neo(message)]
        pub fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
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

        // Method with specific reentrancy protection
        #[neo::no_reentrant_method]
        #[neo(message)]
        pub fn withdraw(&mut self, account: H160, amount: Int256) -> bool {
            if !runtime::check_witness_with_account(account) {
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
                self.balances.put(&account, &new_balance);
            }
            
            true
        }

        // Method with call flags
        #[neo(message)]
        pub fn call_other_contract(&self, contract_hash: H160, method: &str, args: &[Any]) -> Any {
            // Use call flags
            let flags = CallFlags::ReadStates.add(CallFlags::AllowCall);
            runtime::call_contract(contract_hash, method, args, flags)
        }

        #[neo(message)]
        pub fn balance_of(&self, account: H160) -> Int256 {
            match self.balances.get(&account) {
                Some(balance) => balance,
                None => Int256::zero(),
            }
        }

        #[neo(event)]
        pub fn transfer_event(&self, from: H160, to: H160, amount: Int256) {}
    }
}
