#![no_std]

//! # Event Demo Contract for Neo N3
//!
//! This example demonstrates proper Neo N3 event handling:
//! 1. Events are defined with proper structures using the #[event] attribute
//! 2. Events are emitted using the standardized emit() method
//! 3. The #[event] attribute automatically ensures proper Neo N3 format:
//!    - Event names as ByteString
//!    - Parameters in Array<Any>
//!    - Proper handling of Option types
//! 4. Methods are marked appropriately with #[safe] for read-only operations

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::ToString;

use neo_contract::prelude::*;

/// Transfer event following NEP-17 standard
#[event]
struct Transfer {
    #[index]
    from: Option<H160>,
    #[index]
    to: Option<H160>,
    amount: u64,
}

/// Approval event for allowance feature
#[event]
struct Approval {
    #[index]
    owner: H160,
    #[index]
    spender: H160,
    amount: u64,
}

/// Custom event for logging user actions
#[event]
struct CustomEvent {
    #[index]
    user: H160,
    action: String,
    value: u64,
}

/// Event for batch operations with multiple addresses and values
#[event]
struct BatchOperation {
    addresses: Vec<H160>,
    values: Vec<u64>,
    timestamp: u64,
}

// Main contract using Neo Contract annotation syntax
#[contract]
#[contract_author("R3E Network")]
#[contract_description("Event Demo Contract for Neo N3")]
#[contract_version("1.0.0")]
#[supported_standards("NEP-17")]
mod event_demo_contract {
    use super::*;
    
    #[storage]
    struct EventDemoContract {
        balances: StorageMap<H160, u64>,
        allowances: StorageMap<(H160, H160), u64>,
        total_supply: StorageItem<u64>,
    }
    
    impl EventDemoContract {
        #[constructor]
        pub fn new() -> Self {
            Self {
                balances: StorageMap::new(),
                allowances: StorageMap::new(),
                total_supply: StorageItem::new(0),
            }
        }
        
        /// Mint tokens to an address
        #[method]
        pub fn mint(&mut self, to: H160, amount: u64) -> bool {
            // Only contract owner can mint (simplified example)
            let caller = Runtime::calling_script_hash();
            if !Runtime::check_witness(&caller) {
                return false;
            }
            
            // Update balance
            let current_balance = self.balances.get(&to).unwrap_or(0);
            self.balances.set(&to, &(current_balance + amount)).unwrap_or(());
            
            // Update total supply
            let current_supply = self.total_supply.get().unwrap_or(0);
            self.total_supply.set(&(current_supply + amount)).unwrap_or(());
            
            // Emit Transfer event using standardized event pattern
            // From is None for minting operations
            Transfer {
                from: None,
                to: Some(to),
                amount,
            }.emit();
            
            true
        }
        
        /// Transfer tokens from sender to recipient
        #[method]
        pub fn transfer(&mut self, to: H160, amount: u64) -> bool {
            let from = Runtime::calling_script_hash();
            if !Runtime::check_witness(&from) {
                return false;
            }
            
            let from_balance = self.balances.get(&from).unwrap_or(0);
            if from_balance < amount {
                return false;
            }
            
            // Update balances
            self.balances.set(&from, &(from_balance - amount)).unwrap_or(());
            let to_balance = self.balances.get(&to).unwrap_or(0);
            self.balances.set(&to, &(to_balance + amount)).unwrap_or(());
            
            // Emit Transfer event using standardized event pattern
            Transfer {
                from: Some(from),
                to: Some(to),
                amount,
            }.emit();
            
            true
        }
        
        /// Approve spender to use tokens on behalf of owner
        #[method]
        pub fn approve(&mut self, spender: H160, amount: u64) -> bool {
            let owner = Runtime::calling_script_hash();
            if !Runtime::check_witness(&owner) {
                return false;
            }
            
            // Set allowance
            self.allowances.set(&(owner, spender), &amount).unwrap_or(());
            
            // Emit Approval event using standardized event pattern
            Approval {
                owner,
                spender,
                amount,
            }.emit();
            
            true
        }
        
        /// Get balance of an address (read-only)
        #[safe]
        pub fn balance_of(&self, address: H160) -> u64 {
            self.balances.get(&address).unwrap_or(0)
        }
        
        /// Get total supply (read-only)
        #[safe]
        pub fn total_supply(&self) -> u64 {
            self.total_supply.get().unwrap_or(0)
        }
        
        /// Get allowance for owner-spender pair (read-only)
        #[safe]
        pub fn allowance(&self, owner: H160, spender: H160) -> u64 {
            self.allowances.get(&(owner, spender)).unwrap_or(0)
        }
        
        /// Example of emitting a custom event
        #[method]
        pub fn log_custom_event(&mut self, user: H160, action: String, value: u64) -> bool {
            if !Runtime::check_witness(&user) {
                return false;
            }
            
            // Perform some action (simplified example)
            
            // Emit CustomEvent using standardized event pattern
            CustomEvent {
                user,
                action,
                value,
            }.emit();
            
            true
        }
        
        /// Example of emitting events with arrays
        #[method]
        pub fn batch_operation(&mut self, addresses: Vec<H160>, values: Vec<u64>) -> bool {
            let caller = Runtime::calling_script_hash();
            if !Runtime::check_witness(&caller) {
                return false;
            }
            
            // Validate inputs
            if addresses.len() != values.len() || addresses.is_empty() {
                return false;
            }
            
            // Perform batch operation (simplified example)
            for i in 0..addresses.len() {
                let address = addresses[i];
                let value = values[i];
                
                let current_balance = self.balances.get(&address).unwrap_or(0);
                self.balances.set(&address, &(current_balance + value)).unwrap_or(());
            }
            
            // Get current timestamp
            let current_time = Runtime::time();
            
            // Emit BatchOperation event using standardized event pattern
            BatchOperation {
                addresses,
                values,
                timestamp: current_time,
            }.emit();
            
            true
        }
    }
}
