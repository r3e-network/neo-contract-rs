#![no_std]

extern crate alloc;

use alloc::string::String;
use neo_contract::prelude::*;

//! # Contract Call Example for Neo N3
//!
//! A demonstration of cross-contract calls on the Neo N3 blockchain using neo-contract-rs.
//! This contract demonstrates:
//! - Calling other smart contracts on Neo N3
//! - Handling return values from contract calls
//! - Event logging with Neo N3 indexing
//! - Method visibility controls with Neo N3 annotations

/// Event emitted when a contract is called
#[neo_contract::event]
pub struct ContractCalled {
    #[index]
    pub caller: Address,
    #[index]
    pub target_contract: Hash160,
    pub method: String,
    pub success: bool
}

/// Event emitted when ownership is transferred
#[neo_contract::event]
pub struct OwnershipTransferred {
    #[index]
    pub previous_owner: Address,
    #[index]
    pub new_owner: Address,
}

/// Contract Call Example for Neo N3
#[neo_contract::contract]
#[contract_author("R3E Network")]
#[contract_description("Contract Call Example for Neo N3")]
#[contract_version("0.1.0")]
pub struct ContractCaller {
    /// Owner of the contract
    #[storage]
    owner: StorageItem<Address>,
    
    /// Last called contract hash
    #[storage]
    last_called: StorageItem<Hash160>,
    
    /// Number of successful calls
    #[storage]
    successful_calls: StorageItem<u32>,
    
    /// Number of failed calls
    #[storage]
    failed_calls: StorageItem<u32>
}

impl ContractCaller {
    /// Creates a new instance of the contract
    #[constructor]
    pub fn new(owner: Address) -> Self {
        // Initialize the contract
        let mut instance = Self {
            owner: StorageItem::new(b"owner"),
            last_called: StorageItem::new(b"last_called"),
            successful_calls: StorageItem::new(b"successful_calls"),
            failed_calls: StorageItem::new(b"failed_calls"),
        };
        
        // Set the owner
        instance.owner.set(&owner);
        
        // Initialize counters
        instance.successful_calls.set(&0);
        instance.failed_calls.set(&0);
        
        // Return the initialized contract
        instance
    }
    
    /// Call another contract with specified parameters
    #[method]
    #[no_reentry]
    pub fn call_contract(
        &mut self,
        script_hash: Hash160,
        method: String,
        args: Vec<ByteArray>
    ) -> bool {
        // Get the caller
        let caller = Runtime::calling_script_hash();
        
        // Check authorization
        assert!(Runtime::check_witness(&caller), "No authorization");
        
        // Set the last called contract
        self.last_called.set(&script_hash);
        
        // Convert arguments to Any for Neo VM
        let mut call_args = Array::<Any>::new();
        for arg in args {
            call_args.push(Any::from(arg));
        }
        
        // Make the contract call
        let result = Runtime::call_contract(&script_hash, &method, &call_args);
        
        let success = match result {
            Ok(_) => {
                // Increment successful calls counter
                let count = self.successful_calls.get().unwrap_or(0);
                self.successful_calls.set(&(count + 1));
                true
            },
            Err(_) => {
                // Increment failed calls counter
                let count = self.failed_calls.get().unwrap_or(0);
                self.failed_calls.set(&(count + 1));
                false
            }
        };
        
        // Emit event
        ContractCalled {
            caller,
            target_contract: script_hash,
            method,
            success
        }.notify();
        
        success
    }
    
    /// Get an integer value from another contract
    #[safe]
    pub fn get_integer(&self, script_hash: Hash160, method: String) -> u32 {
        // Call the contract
        let mut args = Array::<Any>::new();
        let result: Result<Any, Error> = Runtime::call_contract(&script_hash, &method, &args);
        
        // Return the result
        match result {
            Ok(value) => value.as_integer().unwrap_or(0) as u32,
            Err(_) => 0,
        }
    }
    
    /// Get a string value from another contract
    #[safe]
    pub fn get_string(&self, script_hash: Hash160, method: String) -> String {
        // Call the contract
        let mut args = Array::<Any>::new();
        let result: Result<Any, Error> = Runtime::call_contract(&script_hash, &method, &args);
        
        // Return the result
        match result {
            Ok(value) => value.as_string().unwrap_or_default().to_string(),
            Err(_) => "".to_string(),
        }
    }
    
    /// Get the number of successful contract calls
    #[safe]
    pub fn get_successful_calls(&self) -> u32 {
        self.successful_calls.get().unwrap_or(0)
    }
    
    /// Get the number of failed contract calls
    #[safe]
    pub fn get_failed_calls(&self) -> u32 {
        self.failed_calls.get().unwrap_or(0)
    }
    
    /// Get the last called contract hash
    #[safe]
    pub fn get_last_called(&self) -> Hash160 {
        self.last_called.get().unwrap_or_default()
    }
    
    /// Get the owner of the contract
    #[safe]
    pub fn get_owner(&self) -> Address {
        self.owner.get().unwrap_or_default()
    }
    
    /// Transfer ownership of the contract to a new owner
    #[method]
    #[no_reentry]
    pub fn transfer_ownership(&mut self, new_owner: Address) -> bool {
        // Get current owner
        let current_owner = self.get_owner();
        
        // Check if caller is owner
        assert!(Runtime::check_witness(&current_owner), "Only owner can transfer ownership");
        
        // Cannot transfer to zero address
        assert!(new_owner != Address::zero(), "New owner cannot be zero address");
        
        // Set the new owner
        self.owner.set(&new_owner);
        
        // Emit ownership transferred event
        OwnershipTransferred {
            previous_owner: current_owner,
            new_owner
        }.notify();
        
        true
    }
}
