#![no_std]

extern crate alloc;

use alloc::string::String;

//! # Contract Call Example for Neo N3
//!
//! A demonstration of cross-contract calls on the Neo N3 blockchain using neo-contract-rs.
//! This contract demonstrates:
//! - Calling other smart contracts on Neo N3
//! - Handling return values from contract calls
//! - Event logging with Neo N3 indexing
//! - Method visibility controls with Neo N3 annotations

#[neo_contract::contract]
mod contract_caller {
    use neo_contract::prelude::*;
    use alloc::string::String;
    
    /// Event emitted when a contract is called
    #[event]
    struct ContractCalled {
        #[index]
        caller: Address,
        #[index]
        target_contract: Hash160,
        method: String,
        success: bool
    }
    
    /// Implementation for properly emitting the ContractCalled event using Neo N3 standards
    impl ContractCalled {
        /// Static method to emit the ContractCalled event in Neo N3 format
        pub fn emit(caller: Address, target_contract: Hash160, method: String, success: bool) {
            // Create event name as ByteString (required for Neo N3)
            let event_name = ByteString::from("ContractCalled");
            
            // Create Array to hold event parameters (required for Neo N3)
            let mut event_data = Array::<Any>::new();
            
            // Add parameters with proper Neo N3 format
            event_data.push(Any::from(caller));
            event_data.push(Any::from(target_contract));
            event_data.push(Any::from(method));
            event_data.push(Any::from(success));
            
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
    
    /// Storage for the contract call example
    #[storage]
    struct ContractCaller {
        /// Owner of the contract
        owner: Item<Address>,
        
        /// Last called contract hash
        last_called: Item<Hash160>,
        
        /// Number of successful calls
        successful_calls: Item<u32>,
        
        /// Number of failed calls
        failed_calls: Item<u32>
    }
    
    impl ContractCaller {
        /// Initialize the contract with an owner
        #[constructor]
        fn new(owner: Address) -> Self {
            let mut instance = Self {
                owner: Item::new("owner"),
                last_called: Item::new("last_called"),
                successful_calls: Item::new("successful_calls"),
                failed_calls: Item::new("failed_calls"),
            };
            
            // Initialize storage values
            instance.owner.set(owner);
            instance.successful_calls.set(0);
            instance.failed_calls.set(0);
            
            instance
        }
        
        /// Call another contract with parameters
        #[method]
        #[no_reentry]
        fn call_contract(
            &mut self,
            script_hash: Hash160,
            method: String,
            args: Vec<ByteArray>
        ) -> bool {
            // Get the caller of this transaction
            let caller = Runtime::calling_script_hash();
            
            // Verify caller signature
            assert!(Runtime::check_witness(&caller), "Invalid signature");
            
            // Track success status
            let mut success = false;
            
            // Convert args from Vec<ByteArray> to an array of Any values
            let mut call_args = Array::<Any>::new();
            for arg in args {
                call_args.push(Any::from(arg));
            }
            
            // Call the target contract
            let result = Runtime::call_contract(
                &script_hash,
                &method,
                &call_args
            );
            
            // Update state based on result
            match result {
                Ok(_) => {
                    // Increment success counter
                    let current = self.successful_calls.get().unwrap_or_default();
                    self.successful_calls.set(current + 1);
                    success = true;
                },
                Err(_) => {
                    // Increment failure counter
                    let current = self.failed_calls.get().unwrap_or_default();
                    self.failed_calls.set(current + 1);
                }
            }
            
            // Update last called contract
            self.last_called.set(script_hash);
            
            // Emit event with proper Neo N3 format
            ContractCalled::emit(caller, script_hash, method, success);
            
            success
        }
        
        /// Get an integer value from another contract
        #[method]
        #[safe]
        fn get_integer(&self, script_hash: Hash160, method: String) -> u32 {
            // Call the target contract
            let result: Result<u32, Error> = Runtime::call_contract(
                &script_hash,
                &method,
                &[]
            );
            
            // Return the result or 0 if failed
            result.unwrap_or_default()
        }
        
        /// Get a string value from another contract
        #[method]
        #[safe]
        fn get_string(&self, script_hash: Hash160, method: String) -> String {
            // Call the target contract
            let result: Result<String, Error> = Runtime::call_contract(
                &script_hash,
                &method,
                &[]
            );
            
            // Return the result or empty string if failed
            result.unwrap_or_default()
        }
        
        /// Get the number of successful calls
        #[method]
        #[safe]
        fn get_successful_calls(&self) -> u32 {
            self.successful_calls.get().unwrap_or_default()
        }
        
        /// Get the number of failed calls
        #[method]
        #[safe]
        fn get_failed_calls(&self) -> u32 {
            self.failed_calls.get().unwrap_or_default()
        }
        
        /// Get the last called contract
        #[method]
        #[safe]
        fn get_last_called(&self) -> Hash160 {
            self.last_called.get().unwrap_or_default()
        }
        
        /// Get the contract owner
        #[method]
        #[safe]
        fn get_owner(&self) -> Address {
            self.owner.get().unwrap_or_default()
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
    }
}
