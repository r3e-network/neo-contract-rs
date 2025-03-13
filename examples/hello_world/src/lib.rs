#![no_std]

extern crate alloc;

use alloc::string::String;
use neo_contract::prelude::*;

/// # Hello World Contract for Neo N3
///
/// A simple "Hello World" smart contract for the Neo N3 blockchain using the neo-contract-rs framework.
/// This contract demonstrates:
/// - Basic contract structure with Neo N3 annotations
/// - Storage usage with proper Neo N3 pattern
/// - Event handling with Neo N3 indexing
/// - Method visibility and security controls

#[contract]
#[contract_author("R3E Network")]
#[contract_description("Hello World Example for Neo N3")]
#[contract_version("0.1.0")]
mod hello_world {
    use super::*;

    /// Event emitted when a message is updated
    #[event]
    struct MessageUpdated {
        #[index]
        from: Address,
        old_message: String,
        new_message: String,
    }

    /// Event emitted when ownership is transferred
    #[event]
    struct OwnershipTransferred {
        #[index]
        previous_owner: Address,
        #[index]
        new_owner: Address,
    }

    /// Contract storage structure
    #[storage]
    pub struct HelloWorld {
        /// The stored message
        message: StorageItem<ByteString>,

        /// Counter for message updates
        update_counter: StorageItem<u32>,

        /// The contract owner
        owner: StorageItem<Address>,
    }

    impl HelloWorld {
        /// Initialize a new Hello World contract
        #[constructor]
        pub fn new(owner: Address) -> Self {
            let mut this = Self {
                // Initialize storage items
                message: StorageItem::new(),
                update_counter: StorageItem::new(),
                owner: StorageItem::new(),
            };

            // Set default values
            this.message.set(ByteString::from("Hello, Neo N3!"));
            this.update_counter.set(0u32);
            this.owner.set(owner);

            this
        }

        /// Set a new message (only owner can call this)
        #[method]
        #[no_reentry]
        pub fn set_message(&mut self, new_message: String) -> bool {
            // Get current owner
            let owner = self.owner.get().unwrap_or_default();

            // Verify ownership
            assert!(Runtime::check_witness(&owner), "Only the owner can set the message");

            // Get old message
            let old_message = self.message.get().unwrap_or_default();
            let old_message_string = String::from(&old_message);

            // Don't update if message is the same
            if old_message_string == new_message {
                return false;
            }

            // Convert String to ByteString for storage
            let new_message_bytes = ByteString::from(new_message.clone());

            // Update message
            self.message.set(new_message_bytes);

            // Update counter
            let current_count = self.update_counter.get().unwrap_or_default();
            self.update_counter.set(current_count + 1);

            // Emit event with the standardized event pattern
            MessageUpdated { from: owner, old_message: old_message_string, new_message }.emit();

            true
        }

        /// Get the current message (read-only)
        #[safe]
        pub fn get_message(&self) -> String {
            let message = self.message.get().unwrap_or_default();
            String::from(&message)
        }

        /// Get the number of times the message has been updated
        #[safe]
        pub fn get_update_count(&self) -> u32 { self.update_counter.get().unwrap_or_default() }

        /// Get information about the contract and its state
        #[safe]
        pub fn get_info(&self) -> String {
            let message = self.message.get().unwrap_or_default();
            let count = self.update_counter.get().unwrap_or_default();

            alloc::format!("Message: {}, Updated {} times", String::from(&message), count)
        }

        /// Transfer ownership of the contract
        #[method]
        #[no_reentry]
        pub fn transfer_ownership(&mut self, new_owner: Address) -> bool {
            // Get current owner
            let current_owner = self.owner.get().unwrap_or_default();

            // Verify ownership
            assert!(Runtime::check_witness(&current_owner), "Only the owner can transfer ownership");

            // Ensure new owner is not zero address
            assert!(new_owner != Address::zero(), "Cannot transfer to zero address");

            // Update owner
            self.owner.set(new_owner);

            // Emit ownership transfer event with the standardized event pattern
            OwnershipTransferred { previous_owner: current_owner, new_owner }.emit();

            true
        }

        /// Get the current owner of the contract
        #[safe]
        pub fn get_owner(&self) -> Address { self.owner.get().unwrap_or_default() }
    }
}
