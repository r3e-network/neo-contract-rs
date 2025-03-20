#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;
use neo_contract::prelude::*;
use neo_contract::types::builtin::h160::H160;
use neo_contract::types::builtin::string::ByteString;
use neo_contract::types::builtin::any::Any;
use neo_contract::types::builtin::array::Array;
use neo_contract::Runtime;

/// # Hello World Contract for Neo N3
///
/// A simple "Hello World" smart contract for the Neo N3 blockchain using the neo-contract-rs framework.
/// This contract demonstrates:
/// - Basic contract structure with Neo N3 annotations
/// - Storage usage with proper Neo N3 pattern
/// - Event handling with Neo N3 indexing
/// - Method visibility and security controls

// NOTE: These macro imports cause errors because the macros aren't properly exposed
// in the current version. We'll use manual implementation instead.
// use neo_macros::{contract, contract_author, contract_description, contract_version};
// use neo_macros::{event, index, storage, constructor, method, safe};

// Instead of using macros, we'll define our contract manually
mod hello_world {
    use super::*;

    /// Event emitted when a message is updated
    pub struct MessageUpdated {
        pub from: H160,
        pub old_message: String,
        pub new_message: String,
    }

    /// Event emitted when ownership is transferred
    pub struct OwnershipTransferred {
        pub previous_owner: H160,
        pub new_owner: H160,
    }

    // Helper functions for conversions
    fn byte_string_to_vec(bs: &ByteString) -> Vec<u8> {
        bs.as_bytes().to_vec()
    }
    
    fn vec_to_byte_string(v: &[u8]) -> ByteString {
        ByteString::from(v)
    }
    
    fn byte_string_to_string(bs: &ByteString) -> String {
        String::from_utf8_lossy(bs.as_bytes()).into_owned()
    }
    
    pub fn from_byte_string(bs: &ByteString) -> String {
        String::from_utf8_lossy(bs.as_bytes()).into_owned()
    }

    /// Manual implementation of storage
    pub struct HelloWorldStorage {
        message: Vec<u8>,
        update_counter: u32,
        owner: H160,
    }

    impl HelloWorldStorage {
        /// Initialize a new Hello World contract
        pub fn new(owner: H160) -> Self {
            let hello_message = "Hello, Neo N3!";
            Self {
                message: hello_message.as_bytes().to_vec(),
                update_counter: 0,
                owner,
            }
        }

        /// Set a new message (only owner can call this)
        pub fn set_message(&mut self, new_message: String) -> bool {
            // Verify ownership
            assert!(Runtime::check_witness(&self.owner), "Only the owner can set the message");
            
            let old_message = vec_to_byte_string(&self.message);
            let old_message_string = byte_string_to_string(&old_message);
            
            // Don't update if message is the same
            if old_message_string == new_message {
                return false;
            }
            
            // Update message
            self.message = new_message.as_bytes().to_vec();
            
            // Update counter
            self.update_counter += 1;
            
            // Emit event
            let mut event_args = Array::new();
            event_args.push(Any::from(self.owner.clone()));
            event_args.push(Any::from(old_message_string));
            event_args.push(Any::from(new_message));
            
            Runtime::notify(&ByteString::from("MessageUpdated"), &event_args);
            
            true
        }

        /// Get the current message
        pub fn get_message(&self) -> String {
            let message = vec_to_byte_string(&self.message);
            byte_string_to_string(&message)
        }

        /// Get the number of times the message has been updated
        pub fn get_update_count(&self) -> u32 {
            self.update_counter
        }

        /// Get information about the contract
        pub fn get_info(&self) -> String {
            let message = vec_to_byte_string(&self.message);
            format!(
                "HelloWorld Contract - Message: {}, Updates: {}, Owner: {:?}",
                byte_string_to_string(&message),
                self.update_counter,
                self.owner
            )
        }

        /// Transfer ownership of the contract
        pub fn transfer_ownership(&mut self, new_owner: H160) -> bool {
            // Only the current owner can transfer ownership
            assert!(Runtime::check_witness(&self.owner), "Only the owner can transfer ownership");
            
            // Can't transfer to zero address
            assert!(new_owner != H160::zero(), "Cannot transfer to zero address");
            
            // Can't transfer to self
            if self.owner == new_owner {
                return false;
            }
            
            let current_owner = self.owner.clone();
            
            // Update owner
            self.owner = new_owner.clone();
            
            // Emit event
            let mut event_args = Array::new();
            event_args.push(Any::from(current_owner));
            event_args.push(Any::from(new_owner));
            
            Runtime::notify(&ByteString::from("OwnershipTransferred"), &event_args);
            
            true
        }

        /// Get the current contract owner
        pub fn get_owner(&self) -> H160 {
            self.owner.clone()
        }
    }
}

// Manual implementation of Neo entry points
#[no_mangle]
pub fn deploying() -> bool {
    // Deploy contract
    true
}

#[no_mangle]
pub fn invoke(action: String, args: Vec<Any>) -> Any {
    // Initialize storage
    let mut storage = hello_world::HelloWorldStorage::new(H160::zero());
    
    match action.as_str() {
        "setMessage" => {
            if args.len() != 1 {
                return Any::boolean(false);
            }
            
            let new_message = if let Some(bs) = args[0].as_byte_string() {
                hello_world::from_byte_string(bs)
            } else {
                return Any::boolean(false);
            };
            
            Any::boolean(storage.set_message(new_message))
        },
        "getMessage" => {
            Any::byte_string(storage.get_message())
        },
        "getUpdateCount" => {
            Any::integer(storage.get_update_count())
        },
        "getInfo" => {
            Any::byte_string(storage.get_info())
        },
        "transferOwnership" => {
            if args.len() != 1 {
                return Any::boolean(false);
            }
            
            let new_owner = if let Some(h160) = args[0].as_h160() {
                h160
            } else {
                return Any::boolean(false);
            };
            
            Any::boolean(storage.transfer_ownership(new_owner))
        },
        "getOwner" => {
            Any::from(storage.get_owner())
        },
        _ => {
            Any::null()
        }
    }
}
