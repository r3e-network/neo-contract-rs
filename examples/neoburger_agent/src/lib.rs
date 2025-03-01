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
use neo_contract::builtin::{H160, Int256, ByteString, Array, Any};

/// BurgerAgent is a contract that manages the BurgerNEO contract
pub struct BurgerAgent {
    /// Owner of the contract
    pub owner: H160,
    /// BurgerNEO contract hash
    pub burger_neo: H160,
    /// Fee percentage
    pub fee_percentage: u8,
    /// Fee collector
    pub fee_collector: H160,
}

impl BurgerAgent {
    /// Initialize the contract
    pub fn new() -> Self {
        let owner = Runtime::calling_script_hash();
        Self {
            owner: owner.clone(),
            burger_neo: H160::zero(),
            fee_percentage: 5, // 5% fee
            fee_collector: owner,
        }
    }

    /// Set the BurgerNEO contract hash
    pub fn set_burger_neo(&mut self, contract_hash: H160) -> bool {
        if !Runtime::check_witness(self.owner.clone()) {
            return false;
        }

        self.burger_neo = contract_hash;
        true
    }

    /// Set the fee percentage
    pub fn set_fee_percentage(&mut self, percentage: u8) -> bool {
        if !Runtime::check_witness(self.owner.clone()) {
            return false;
        }

        if percentage > 100 {
            return false;
        }

        self.fee_percentage = percentage;
        true
    }

    /// Set the fee collector
    pub fn set_fee_collector(&mut self, collector: H160) -> bool {
        if !Runtime::check_witness(self.owner.clone()) {
            return false;
        }

        self.fee_collector = collector;
        true
    }

    /// Collect fees
    pub fn collect_fees(&mut self) -> bool {
        if !Runtime::check_witness(self.owner.clone()) {
            return false;
        }

        if self.burger_neo == H160::zero() {
            return false;
        }

        // Call BurgerNEO to get rewards
        let mut args = Array::new();
        args.push(Any::from(Runtime::executing_script_hash()));
        
        let result = Runtime::call_contract(
            self.burger_neo.clone(),
            ByteString::from("getReward"),
            args
        );

        // Transfer fees to the fee collector
        if let Some(amount) = result.as_int256() {
            if amount > Int256::zero() {
                let fee_percentage = Int256::from(self.fee_percentage as i64);
                let fee_amount = amount.clone() * fee_percentage / Int256::from(100);
                let remaining = amount - fee_amount.clone();

                // Emit fee collected event
                let mut fee_args = Array::new();
                fee_args.push(Any::from(self.fee_collector.clone()));
                fee_args.push(Any::from(fee_amount));
                
                Runtime::notify(
                    &ByteString::from("FeeCollected"),
                    &fee_args
                );
                
                // Emit reward collected event
                let mut reward_args = Array::new();
                reward_args.push(Any::from(self.owner.clone()));
                reward_args.push(Any::from(remaining));
                
                Runtime::notify(
                    &ByteString::from("RewardCollected"),
                    &reward_args
                );
                
                return true;
            }
        }

        false
    }
}

// Required for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
