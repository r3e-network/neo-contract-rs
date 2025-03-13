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
#[contract_description("BurgerNEO Agent Contract")]
#[contract_version("0.1.0")]
#[contract_source_code("https://github.com/R3E-Network/neo-contract-rs")]
mod burger_agent {
    use neo::prelude::*;
    use neo::runtime;
    use neo::types::*;

    /// BurgerAgent is a contract that manages the BurgerNEO contract
    #[storage]
    pub struct Agent {
        /// Owner of the contract
        owner: H160,
        /// BurgerNEO contract hash
        burger_neo: H160,
        /// Fee percentage
        fee_percentage: u8,
        /// Fee collector
        fee_collector: H160,
    }

    impl Agent {
        /// Initialize the contract
        #[constructor]
        pub fn new() -> Self {
            let owner = runtime::calling_script_hash();
            Self {
                owner: owner.clone(),
                burger_neo: H160::zero(),
                fee_percentage: 5, // 5% fee
                fee_collector: owner,
            }
        }

        /// Set the BurgerNEO contract hash
        #[method]
        pub fn set_burger_neo(&mut self, contract_hash: H160) -> bool {
            if !runtime::check_witness(self.owner.clone()) {
                return false;
            }

            self.burger_neo = contract_hash;
            true
        }

        /// Set the fee percentage
        #[method]
        pub fn set_fee_percentage(&mut self, percentage: u8) -> bool {
            if !runtime::check_witness(self.owner.clone()) {
                return false;
            }

            if percentage > 100 {
                return false;
            }

            self.fee_percentage = percentage;
            true
        }

        /// Set the fee collector
        #[method]
        pub fn set_fee_collector(&mut self, collector: H160) -> bool {
            if !runtime::check_witness(self.owner.clone()) {
                return false;
            }

            self.fee_collector = collector;
            true
        }

        /// Collect fees
        #[method]
        pub fn collect_fees(&mut self) -> bool {
            if !runtime::check_witness(self.owner.clone()) {
                return false;
            }

            if self.burger_neo == H160::zero() {
                return false;
            }

            // Call BurgerNEO to get rewards
            let mut args = Array::new();
            args.push(Any::from(runtime::executing_script_hash()));
            
            let result = runtime::call_contract(
                self.burger_neo.clone(),
                ByteString::from("getReward"),
                &args
            );

            // Transfer fees to the fee collector
            if let Some(amount) = result.as_int256() {
                if amount > Int256::zero() {
                    let fee_percentage = Int256::from(self.fee_percentage as i64);
                    let fee_amount = amount.clone() * fee_percentage / Int256::from(100);
                    let remaining = amount - fee_amount.clone();

                    self.fee_collected_event(self.fee_collector.clone(), fee_amount.clone());
                    self.reward_collected_event(self.owner.clone(), remaining.clone());
                    
                    return true;
                }
            }

            false
        }

        #[event]
        pub fn fee_collected_event(&self, collector: H160, amount: Int256) {}

        #[event]
        pub fn reward_collected_event(&self, owner: H160, amount: Int256) {}
    }
}

// Required for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
