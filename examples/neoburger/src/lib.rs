// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

// Use wee_alloc as the global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use neo_contract as neo;

#[contract]
#[contract_author("R3E Network")]
#[contract_email("dev@r3e.network")]
#[contract_description("BurgerNEO Token")]
#[contract_version("0.1.0")]
#[contract_source_code("https://github.com/R3E-Network/neo-contract-rs")]
#[supported_standards("NEP-17")]
mod burger_neo {
    use neo::prelude::*;
    use neo::runtime;
    use neo::types::*;
    use neo::storage::StorageMap;
    use alloc::string::ToString;

    // Implement IntoAny for the types we need
    trait IntoAny {
        fn into_any(self) -> Any;
    }

    impl IntoAny for H160 {
        fn into_any(self) -> Any {
            Any::from(self)
        }
    }

    impl IntoAny for Int256 {
        fn into_any(self) -> Any {
            Any::from(self)
        }
    }

    impl IntoAny for ByteString {
        fn into_any(self) -> Any {
            Any::from(self)
        }
    }

    #[storage]
    pub struct Burger {
        owner: H160,
        agent: H160,
        strategist: H160,
        total_supply: Int256,
        balances: builtin::Map,
    }

    impl Burger {
        #[constructor]
        pub fn new() -> Self {
            let owner = runtime::calling_script_hash();
            
            Self {
                owner: owner.clone(),
                agent: owner.clone(),
                strategist: owner.clone(),
                total_supply: Int256::zero(),
                balances: builtin::Map::new(),
            }
        }
        
        #[message]
        pub fn name(&self) -> ByteString {
            ByteString::from("BurgerNEO")
        }

        #[message]
        pub fn symbol(&self) -> ByteString {
            ByteString::from("bNEO")
        }

        #[message]
        pub fn decimals(&self) -> u8 {
            8
        }

        #[message]
        pub fn total_supply(&self) -> Int256 {
            self.total_supply.clone()
        }

        #[message]
        pub fn balance_of(&self, account: H160) -> Int256 {
            match self.balances.get(&account) {
                Some(balance) => balance,
                None => Int256::zero(),
            }
        }

        #[message]
        pub fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
            if amount <= Int256::zero() {
                return false;
            }

            if !runtime::check_witness(from.clone()) {
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
                    self.balances.put(&from, &from_new_balance);
                }
                
                let to_balance = self.balance_of(to.clone());
                let to_new_balance = to_balance + amount.clone();
                self.balances.put(&to, &to_new_balance);
            }
            
            self.transfer_event(from, to, amount);
            
            true
        }
        
        #[message]
        pub fn set_agent(&mut self, new_agent: H160) -> bool {
            if !runtime::check_witness(self.owner.clone()) {
                return false;
            }
            
            self.agent = new_agent;
            true
        }
        
        #[message]
        pub fn set_strategist(&mut self, new_strategist: H160) -> bool {
            if !runtime::check_witness(self.owner.clone()) {
                return false;
            }
            
            self.strategist = new_strategist;
            true
        }
        
        #[message]
        pub fn mint(&mut self, to: H160, amount: Int256) -> bool {
            if !runtime::check_witness(self.agent.clone()) {
                return false;
            }
            
            if amount <= Int256::zero() {
                return false;
            }
            
            let to_balance = self.balance_of(to.clone());
            let new_balance = to_balance + amount.clone();
            self.balances.put(&to, &new_balance);
            
            self.total_supply = self.total_supply.clone() + amount.clone();
            
            self.transfer_event(H160::zero(), to, amount);
            
            true
        }
        
        #[message]
        pub fn burn(&mut self, from: H160, amount: Int256) -> bool {
            if !runtime::check_witness(from.clone()) {
                return false;
            }
            
            if amount <= Int256::zero() {
                return false;
            }
            
            let from_balance = self.balance_of(from.clone());
            if from_balance < amount {
                return false;
            }
            
            let new_balance = from_balance - amount.clone();
            if new_balance.is_zero() {
                self.balances.delete(&from);
            } else {
                self.balances.put(&from, &new_balance);
            }
            
            self.total_supply = self.total_supply.clone() - amount.clone();
            
            self.transfer_event(from, H160::zero(), amount);
            
            true
        }

        #[event]
        pub fn transfer_event(&self, from: H160, to: H160, amount: Int256) {
            let mut args = Array::new();
            args.push(from.into_any());
            args.push(to.into_any());
            args.push(amount.into_any());
            
            let event_name = ByteString::from("Transfer");
            runtime::notify(&event_name, &args);
        }
    }
}

// Required for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
