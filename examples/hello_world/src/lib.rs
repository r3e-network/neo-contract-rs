#![no_std]

extern crate alloc;

use alloc::string::String;

//! # Hello World Contract for Neo N3
//!
//! A simple "Hello World" smart contract for the Neo N3 blockchain using the neo-contract-rs framework.
//! This contract demonstrates:
//! - Basic contract structure with Neo N3 annotations
//! - Storage usage with proper Neo N3 pattern
//! - Event handling with Neo N3 indexing
//! - Method visibility and security controls

#[neo_contract::contract]
mod hello_world {
    use neo_contract::prelude::*;
    use alloc::string::String;
    
    /// Event emitted when a message is updated
    #[event]
    struct MessageUpdated {
        #[index]
        from: Address,
        #[index]
        old_message: String,
        #[index]
        new_message: String,
    }
    
    /// Implementation for properly emitting the MessageUpdated event using Neo N3 standards
    impl MessageUpdated {
        /// Static method to emit the MessageUpdated event in Neo N3 format
        pub fn emit(from: Address, old_message: String, new_message: String) {
            // Create event name as ByteString (required for Neo N3)
            let event_name = ByteString::from("MessageUpdated");
            
            // Create Array to hold event parameters (required for Neo N3)
            let mut event_data = Array::<Any>::new();
            
            // Add parameters with proper Neo N3 format
            event_data.push(Any::from(from));
            event_data.push(Any::from(old_message));
            event_data.push(Any::from(new_message));
            
            // Emit the event using Runtime::notify (required for Neo N3)
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    /// Event emitted when ownership is transferred
    #[event]
    struct OwnershipTransferred {
        #[index]
        previous_owner: Address,
        #[index]
        new_owner: Address,
    }
    
    /// Implementation for properly emitting the OwnershipTransferred event using Neo N3 standards
    impl OwnershipTransferred {
        /// Static method to emit the OwnershipTransferred event in Neo N3 format
        pub fn emit(previous_owner: Address, new_owner: Address) {
            // Create event name as ByteString (required for Neo N3)
            let event_name = ByteString::from("OwnershipTransferred");
            
            // Create Array to hold event parameters (required for Neo N3)
            let mut event_data = Array::<Any>::new();
            
            // Add parameters with proper Neo N3 format
            event_data.push(Any::from(previous_owner));
            event_data.push(Any::from(new_owner));
            
            // Emit the event using Runtime::notify (required for Neo N3)
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    /// Contract storage structure
    #[storage]
    struct HelloWorld {
        /// The stored message
        message: Item<String>,
        
        /// Counter for number of updates
        update_counter: Item<u32>,
        
        /// Contract owner address
        owner: Item<Address>,
    }
    
    impl HelloWorld {
        /// Initialize a new Hello World contract
        #[constructor]
        fn new(owner: Address) -> Self {
            let mut instance = Self {
                message: Item::new("message"),
                update_counter: Item::new("update_counter"),
                owner: Item::new("owner"),
            };
            
            // Set initial values
            instance.message.set("Hello, Neo Smart Contract World!".to_string());
            instance.update_counter.set(0);
            instance.owner.set(owner);
            
            instance
        }
        
        /// Set a new message (only owner can call this)
        #[method]
        #[no_reentry]
        fn set_message(&mut self, new_message: String) -> bool {
            // Get the contract owner
            let owner = self.owner.get().unwrap_or_default();
            
            // Make sure only the owner can update the message
            assert!(Runtime::check_witness(&owner), "Only the owner can update the message");
            
            // Get the current message
            let old_message = self.message.get().unwrap_or_default();
            
            // Update the message
            self.message.set(new_message.clone());
            
            // Increment the update counter
            let counter = self.update_counter.get().unwrap_or_default();
            self.update_counter.set(counter + 1);
            
            // Emit event with proper Neo N3 format
            MessageUpdated::emit(owner, old_message, new_message);
            
            true
        }
        
        /// Get the current message (read-only)
        #[method]
        #[safe]
        fn get_message(&self) -> String {
            self.message.get().unwrap_or_default()
        }
        
        /// Get the number of times the message has been updated
        #[method]
        #[safe]
        fn get_update_count(&self) -> u32 {
            self.update_counter.get().unwrap_or_default()
        }
        
        /// Get information about the contract and its state
        #[method]
        #[safe]
        fn get_info(&self) -> String {
            let message = self.message.get().unwrap_or_default();
            let count = self.update_counter.get().unwrap_or_default();
            
            alloc::format!("Message: {}, Updated {} times", message, count)
        }
        
        /// Transfer ownership of the contract
        #[method]
        #[no_reentry]
        fn transfer_ownership(&mut self, new_owner: Address) -> bool {
            // Get current owner
            let current_owner = self.owner.get().unwrap_or_default();
            
            // Verify ownership
            assert!(Runtime::check_witness(&current_owner), "Only the owner can transfer ownership");
            
            // Ensure new owner is not zero address
            assert!(new_owner != Address::zero(), "Cannot transfer to zero address");
            
            // Update owner
            self.owner.set(new_owner);
            
            // Emit ownership transfer event
            OwnershipTransferred::emit(current_owner, new_owner);
            
            true
        }
        
        /// Get the current owner of the contract
        #[method]
        #[safe]
        fn get_owner(&self) -> Address {
            self.owner.get().unwrap_or_default()
        }
    }
}
