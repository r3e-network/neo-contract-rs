#![no_std]
#![allow(unused_imports)]

extern crate alloc;

// Import necessary types
use alloc::string::String;
use alloc::vec::Vec;

// Import everything from prelude
use neo_contract::prelude::*;

// Import core types directly
use neo_contract::types::builtin::h160::H160;
use neo_contract::types::builtin::string::ByteString;
use neo_contract::types::builtin::array::Array;
use neo_contract::types::builtin::any::Any;
use neo_contract::Runtime;
use neo_contract::storage::item::Item;

/// A simple Hello World contract for Neo N3
/// 
/// This demonstrates how to structure a Neo contract with proper annotations
/// while working around current limitations in the framework.

// Contract module - in a full implementation this would use #[contract] etc.
// For now, we'll use comments to show the annotations for documentation purposes
// #[contract]
// #[contract_author("NEO Rust Team")]
// #[contract_description("A hello world contract with attribute macros")]
// #[contract_version("0.1.0")]
mod hello_contract {
    use super::*;
    
    // Define the Transfer event
    // In a full implementation this would use #[event] and #[index]
    // #[event]
    pub struct MessageUpdated {
        // #[index]
        pub from: H160,
        pub old_message: String,
        pub new_message: String,
    }
    
    // Manual implementation for event emission
    impl MessageUpdated {
        pub fn emit(from: H160, old_message: String, new_message: String) {
            let mut event_args = Array::new();
            event_args.push(Any::from(from));
            event_args.push(Any::from(ByteString::from(old_message)));
            event_args.push(Any::from(ByteString::from(new_message)));
            
            Runtime::notify(&ByteString::from("MessageUpdated"), &event_args);
        }
    }
    
    // Define contract storage
    // In a full implementation this would use #[storage]
    // #[storage]
    pub struct HelloContract {
        pub message: Item<ByteString>,
        pub update_counter: Item<u32>,
        pub owner: Item<H160>,
    }
    
    impl HelloContract {
        // Constructor for initializing the contract
        // In a full implementation this would use #[constructor]
        // #[constructor]
        pub fn new() -> Self {
            // Set default message
            let message = ByteString::from("Hello, Neo N3!");
            
            // Get the deployer's address
            let owner = Runtime::calling_script_hash();
            
            // Create instance
            let mut contract = Self {
                message: Item::new(b"message"),
                update_counter: Item::new(b"update_counter"),
                owner: Item::new(b"owner"),
            };
            
            // Initialize storage
            contract.message.set(&message).unwrap_or(());
            contract.update_counter.set(&0).unwrap_or(());
            contract.owner.set(&owner).unwrap_or(());
            
            contract
        }
        
        // Method to update the message
        // In a full implementation this would use #[method]
        // #[method]
        pub fn update_message(&mut self, new_message: ByteString) -> bool {
            // Check that the caller is the owner
            let caller = Runtime::calling_script_hash();
            let owner = self.owner.get().unwrap_or(None).unwrap_or_default();
            
            if caller != owner {
                return false;
            }
            
            // Get the old message
            let old_message = self.message.get().unwrap_or(None).unwrap_or_default();
            
            // Update the message
            self.message.set(&new_message).unwrap_or(());
            
            // Increment the counter
            let counter = self.update_counter.get().unwrap_or(None).unwrap_or(0);
            self.update_counter.set(&(counter + 1)).unwrap_or(());
            
            // Emit event
            MessageUpdated::emit(
                caller, 
                String::from_utf8_lossy(&old_message).into_owned(), 
                String::from_utf8_lossy(&new_message).into_owned()
            );
            
            true
        }
        
        // Read-only method to get the message
        // In a full implementation this would use #[safe]
        // #[safe]
        pub fn get_message(&self) -> ByteString {
            self.message.get().unwrap_or(None).unwrap_or_default()
        }
        
        // Read-only method to get the update count
        // In a full implementation this would use #[safe]
        // #[safe]
        pub fn get_update_count(&self) -> u32 {
            self.update_counter.get().unwrap_or(None).unwrap_or(0)
        }
        
        // Read-only method to get the owner
        // In a full implementation this would use #[safe]
        // #[safe]
        pub fn get_owner(&self) -> H160 {
            self.owner.get().unwrap_or(None).unwrap_or_default()
        }
    }
}

// Entry points for Neo VM
#[no_mangle]
pub fn deploying() -> bool {
    true
}

#[no_mangle]
pub fn invoke(operation: String, args: Vec<Any>) -> Any {
    // Create an instance of the contract
    let mut contract = hello_contract::HelloContract::new();
    
    // Handle operations
    match operation.as_str() {
        "updateMessage" => {
            if args.len() != 1 {
                return Any::boolean(false);
            }
            
            let new_message = if let Some(bs) = args[0].as_byte_string() {
                bs.clone()
            } else {
                return Any::boolean(false);
            };
            
            Any::boolean(contract.update_message(new_message))
        },
        "getMessage" => {
            Any::byte_string(contract.get_message())
        },
        "getUpdateCount" => {
            Any::integer(contract.get_update_count() as i64)
        },
        "getOwner" => {
            Any::h160(contract.get_owner())
        },
        _ => {
            Any::null()
        }
    }
} 