// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

use neo_contract::{
    builtin::{H160, Int256, ByteString, Map, Array, Any},
    Runtime,
    contract, contract_author, contract_description,
    contract_version, supported_standards,
    storage, constructor, message, event,
};
use alloc::vec::Vec;

// Add global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Add panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[contract]
#[contract_author("R3E Network")]
#[contract_description("NEP-17 Standard Implementation")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
mod nep17_contract {
    use super::*;

    #[storage]
    pub struct Nep17Token {
        total_supply: Int256,
        balances: Map<H160, Int256>,
    }

    impl Nep17Token {
        #[constructor]
        pub fn new(initial_supply: Int256) -> Self {
            let mut balances = Map::new();
            let owner = Runtime::calling_script_hash();
            
            balances.put(owner.clone(), initial_supply.clone());
            
            // Emit transfer event for minting
            Self::transfer_event(None, Some(owner), initial_supply.clone());
            
            Self {
                total_supply: initial_supply,
                balances,
            }
        }
        
        #[message]
        pub fn symbol(&self) -> ByteString {
            ByteString::from("NEP17")
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
                Some(balance) => balance.clone(),
                None => Int256::zero(),
            }
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
            
            // Emit transfer event
            Self::transfer_event(Some(from), Some(to), amount);
            
            true
        }
        
        #[event]
        pub fn transfer_event(from: Option<H160>, to: Option<H160>, amount: Int256) {}
    }
}

// NEP-17 Trait Implementation
impl neo_contract::nep17::NEP17 for nep17_contract::Nep17Token {
    fn symbol(&self) -> ByteString {
        self.symbol()
    }

    fn decimals(&self) -> u8 {
        self.decimals()
    }

    fn total_supply(&self) -> Int256 {
        self.total_supply()
    }

    fn balance_of(&self, account: H160) -> Int256 {
        self.balance_of(account)
    }

    fn transfer(&mut self, from: H160, to: H160, amount: Int256, data: ByteString) -> bool {
        self.transfer(from, to, amount, Some(Any::from(data)))
    }
}
