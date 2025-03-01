// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

// Use wee_alloc as the global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use neo_contract::Runtime;
use neo_contract::builtin;
use neo_contract::builtin::{H160, Int256, ByteString, Array, Map, Any};

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

mod burger_neo {
    use super::*;
    use alloc::string::ToString;
    use super::IntoAny;

    pub struct BurgerStorage {
        pub owner: H160,
        pub agent: H160,
        pub strategist: H160,
        pub total_supply: Int256,
        pub balances: Map<H160, Int256>,
    }

    impl BurgerStorage {
        pub fn new() -> Self {
            let owner = Runtime::calling_script_hash();
            
            Self {
                owner: owner.clone(),
                agent: owner.clone(),
                strategist: owner.clone(),
                total_supply: Int256::zero(),
                balances: Map::new(),
            }
        }
        
        pub fn name(&self) -> ByteString {
            ByteString::from("BurgerNEO")
        }

        pub fn symbol(&self) -> ByteString {
            ByteString::from("bNEO")
        }

        pub fn decimals(&self) -> u8 {
            8
        }

        pub fn total_supply(&self) -> Int256 {
            self.total_supply.clone()
        }

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

        pub fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
            if amount <= Int256::zero() {
                return false;
            }

            if !Runtime::check_witness(from.clone()) {
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
            let mut args = Array::new();
            args.push(from.into_any());
            args.push(to.into_any());
            args.push(amount.into_any());
            
            let event_name = ByteString::from("Transfer");
            Runtime::notify(&event_name, &args);
            
            true
        }
        
        pub fn set_agent(&mut self, new_agent: H160) -> bool {
            if !Runtime::check_witness(self.owner.clone()) {
                return false;
            }
            
            self.agent = new_agent;
            true
        }
        
        pub fn set_strategist(&mut self, new_strategist: H160) -> bool {
            if !Runtime::check_witness(self.owner.clone()) {
                return false;
            }
            
            self.strategist = new_strategist;
            true
        }
        
        pub fn mint(&mut self, to: H160, amount: Int256) -> bool {
            if !Runtime::check_witness(self.agent.clone()) {
                return false;
            }
            
            if amount <= Int256::zero() {
                return false;
            }
            
            let to_balance = self.balance_of(to.clone());
            let new_balance = to_balance + amount.clone();
            self.balances.put(to.clone(), new_balance);
            
            self.total_supply = self.total_supply.clone() + amount.clone();
            
            // Emit transfer event (mint from null address)
            let mut args = Array::new();
            args.push(H160::zero().into_any());
            args.push(to.into_any());
            args.push(amount.into_any());
            
            let event_name = ByteString::from("Transfer");
            Runtime::notify(&event_name, &args);
            
            true
        }
        
        pub fn burn(&mut self, from: H160, amount: Int256) -> bool {
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
            
            let new_balance = from_balance - amount.clone();
            if new_balance.is_zero() {
                self.balances.delete(&from);
            } else {
                self.balances.put(from.clone(), new_balance);
            }
            
            self.total_supply = self.total_supply.clone() - amount.clone();
            
            // Emit transfer event (burn to null address)
            let mut args = Array::new();
            args.push(from.into_any());
            args.push(H160::zero().into_any());
            args.push(amount.into_any());
            
            let event_name = ByteString::from("Transfer");
            Runtime::notify(&event_name, &args);
            
            true
        }
    }
}

// Required for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
