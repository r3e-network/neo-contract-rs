// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

use alloc::format;

// Global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Import all ink! style attributes and common types
use neo_contract::prelude::*;
use neo_contract::types::builtin::string::ByteString;
use neo_contract::types::builtin::int256::Int256;
use neo_contract::types::builtin::h160::H160;
use neo_contract::types::builtin::array::Array;
use neo_contract::types::builtin::map::Map;
use neo_contract::types::notification::Notification;
use neo_contract::runtime::Runtime;
use neo_contract::{
    contract, message, storage,
    contract_author, contract_email, contract_description, contract_version
};

// Define the contract with full metadata
#[contract]
#[contract_author("Neo N3 Developer")]
#[contract_email("dev@neo.org")]
#[contract_description("A complete example of ink! style Neo N3 contract")]
#[contract_version("1.0.0")]
#[supported_standards("NEP-17")]
mod token_contract {
    use super::*;
    
    // Define storage with #[storage] attribute
    #[storage]
    pub struct Token {
        total_supply: Int256,
        balances: Map<H160, Int256>,
        token_name: ByteString,
        token_symbol: ByteString,
        token_decimals: Int256,
        owner: H160,
    }
    
    impl Token {
        // Define constructor with #[constructor] attribute
        #[constructor]
        pub fn new(name: ByteString, symbol: ByteString, decimals: Int256, initial_supply: Int256) -> Self {
            let mut balances = Map::new();
            let owner = Runtime::calling_script_hash();
            
            balances.put(owner.clone(), initial_supply.clone());
            
            // Emit token creation event
            Self::emit_created_event(owner.clone(), initial_supply.clone());
            
            Self {
                total_supply: initial_supply,
                balances,
                token_name: name,
                token_symbol: symbol,
                token_decimals: decimals,
                owner,
            }
        }
        
        // Define public methods with #[method] attribute
        // Using #[safe] to indicate these are read-only methods
        #[safe]
        pub fn get_name(&self) -> ByteString {
            self.token_name.clone()
        }
        
        #[safe]
        pub fn get_symbol(&self) -> ByteString {
            self.token_symbol.clone()
        }
        
        #[safe]
        pub fn get_decimals(&self) -> Int256 {
            self.token_decimals.clone()
        }
        
        #[safe]
        pub fn get_total_supply(&self) -> Int256 {
            self.total_supply.clone()
        }
        
        #[safe]
        pub fn balance_of(&self, account: H160) -> Int256 {
            match self.balances.get(&account) {
                Some(balance) => balance.clone(),
                None => Int256::zero(),
            }
        }
        
        // State-modifying methods don't have the #[safe] attribute
        #[method]
        pub fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
            // Validate inputs
            if amount <= Int256::zero() {
                return false;
            }
            
            if !Runtime::check_witness(from.clone()) {
                return false;
            }
            
            // Check if the sender has enough balance
            let from_balance = match self.balances.get(&from) {
                Some(balance) => balance.clone(),
                None => Int256::zero(),
            };
            
            if from_balance < amount {
                return false;
            }
            
            // Update balances
            let new_from_balance = from_balance - amount.clone();
            if new_from_balance.is_zero() {
                self.balances.delete(&from);
            } else {
                self.balances.put(from.clone(), new_from_balance);
            }
            
            let to_balance = match self.balances.get(&to) {
                Some(balance) => balance.clone(),
                None => Int256::zero(),
            };
            
            self.balances.put(to.clone(), to_balance + amount.clone());
            
            // Emit transfer event
            Self::emit_transfer_event(from, to, amount);
            
            true
        }
        
        // State-modifying methods don't have the #[safe] attribute
        #[method]
        pub fn mint(&mut self, to: H160, amount: Int256) -> bool {
            // Only owner can mint
            if !Runtime::check_witness(self.owner.clone()) {
                return false;
            }
            
            // Validate amount
            if amount <= Int256::zero() {
                return false;
            }
            
            // Update total supply
            let new_supply = self.total_supply.clone() + amount.clone();
            self.total_supply = new_supply;
            
            // Update recipient balance
            let to_balance = match self.balances.get(&to) {
                Some(balance) => balance.clone(),
                None => Int256::zero(),
            };
            
            self.balances.put(to.clone(), to_balance + amount.clone());
            
            // Emit mint event
            Self::emit_mint_event(to, amount);
            
            true
        }
        
        // State-modifying methods don't have the #[safe] attribute
        #[method]
        pub fn burn(&mut self, from: H160, amount: Int256) -> bool {
            // Must be owner or the account itself
            if !Runtime::check_witness(from.clone()) && !Runtime::check_witness(self.owner.clone()) {
                return false;
            }
            
            // Validate amount
            if amount <= Int256::zero() {
                return false;
            }
            
            // Check if the account has enough balance
            let from_balance = match self.balances.get(&from) {
                Some(balance) => balance.clone(),
                None => Int256::zero(),
            };
            
            if from_balance < amount {
                return false;
            }
            
            // Update total supply
            let new_supply = self.total_supply.clone() - amount.clone();
            self.total_supply = new_supply;
            
            // Update account balance
            let new_from_balance = from_balance - amount.clone();
            if new_from_balance.is_zero() {
                self.balances.delete(&from);
            } else {
                self.balances.put(from.clone(), new_from_balance);
            }
            
            // Emit burn event
            Self::emit_burn_event(from, amount);
            
            true
        }
        
        // Emit events directly from methods
        fn emit_transfer_event(from: H160, to: H160, amount: Int256) {
            use alloc::format;
            use neo_contract::types::builtin::array::Array;
            use neo_contract::types::builtin::string::ByteString;
            use neo_contract::types::builtin::any::Any;
            use neo_contract::env::syscall;
            
            let mut args = Array::<Any>::new();
            args.push(Any::from(ByteString::from(format!("{:?}", from))));
            args.push(Any::from(ByteString::from(format!("{:?}", to))));
            args.push(Any::from(ByteString::from(format!("{:?}", amount))));
            
            let event_name = ByteString::from("transfer_event");
            unsafe {
                syscall::system_runtime_notify(event_name, args);
            }
        }
        
        fn emit_mint_event(to: H160, amount: Int256) {
            use alloc::format;
            use neo_contract::types::builtin::array::Array;
            use neo_contract::types::builtin::string::ByteString;
            use neo_contract::types::builtin::any::Any;
            use neo_contract::env::syscall;
            
            let mut args = Array::<Any>::new();
            args.push(Any::from(ByteString::from(format!("{:?}", to))));
            args.push(Any::from(ByteString::from(format!("{:?}", amount))));
            
            let event_name = ByteString::from("mint_event");
            unsafe {
                syscall::system_runtime_notify(event_name, args);
            }
        }
        
        fn emit_burn_event(from: H160, amount: Int256) {
            use alloc::format;
            use neo_contract::types::builtin::array::Array;
            use neo_contract::types::builtin::string::ByteString;
            use neo_contract::types::builtin::any::Any;
            use neo_contract::env::syscall;
            
            let mut args = Array::<Any>::new();
            args.push(Any::from(ByteString::from(format!("{:?}", from))));
            args.push(Any::from(ByteString::from(format!("{:?}", amount))));
            
            let event_name = ByteString::from("burn_event");
            unsafe {
                syscall::system_runtime_notify(event_name, args);
            }
        }
        
        fn emit_created_event(owner: H160, initial_supply: Int256) {
            use alloc::format;
            use neo_contract::types::builtin::array::Array;
            use neo_contract::types::builtin::string::ByteString;
            use neo_contract::types::builtin::any::Any;
            use neo_contract::env::syscall;
            
            let mut args = Array::<Any>::new();
            args.push(Any::from(ByteString::from(format!("{:?}", owner))));
            args.push(Any::from(ByteString::from(format!("{:?}", initial_supply))));
            
            let event_name = ByteString::from("created_event");
            unsafe {
                syscall::system_runtime_notify(event_name, args);
            }
        }
    }
}
