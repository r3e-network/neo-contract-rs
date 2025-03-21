// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![no_std]
#![no_main]

use neo_contract as neo;
use neo::{contract::*, types::*};

pub struct SimpleStorage;

#[neo::contract]
impl SimpleStorage {
    /// Stores a value in the contract storage.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to associate with the value
    /// * `value` - The value to store
    pub fn set(key: ByteString, value: ByteString) {
        // Create storage context
        let mut storage = StorageMap::new();
        // Store the value
        storage.put(key, value);
    }
    
    /// Retrieves a stored value from contract storage.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the value to retrieve
    ///
    /// # Returns
    ///
    /// The stored value, or an empty ByteString if not found
    pub fn get(key: ByteString) -> ByteString {
        // Create storage context
        let storage = StorageMap::new();
        // Get the value
        let value = storage.get(key);
        
        // Check if value exists
        if value.is_null() {
            return ByteString::empty();
        }
        
        // Return the value
        value.unwrap()
    }
    
    /// Deletes a key-value pair from contract storage.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to delete
    pub fn delete(key: ByteString) {
        let mut storage = StorageMap::new();
        storage.delete(key);
    }
    
    /// Checks if a key exists in the contract storage.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check
    ///
    /// # Returns
    ///
    /// True if the key exists, false otherwise
    pub fn has_key(key: ByteString) -> bool {
        let storage = StorageMap::new();
        storage.contains_key(key)
    }
    
    /// Increments an integer value stored at a given key.
    /// If the key doesn't exist, it initializes it with 1.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the integer value
    ///
    /// # Returns
    ///
    /// The updated value after incrementing
    pub fn increment(key: ByteString) -> Int256 {
        let mut storage = StorageMap::new();
        
        // Get current value or default to zero
        let value = if storage.contains_key(key.clone()) {
            let stored_value = storage.get(key.clone()).unwrap();
            Int256::from_byte_string(stored_value)
        } else {
            Int256::zero()
        };
        
        // Increment the value
        let new_value = value.checked_add(&Int256::from_i32(1));
        
        // Store the updated value
        storage.put(key, new_value.into_byte_string());
        
        // Return the new value
        new_value
    }
    
    /// Stores multiple key-value pairs in a single operation.
    ///
    /// # Arguments
    ///
    /// * `keys` - Array of keys
    /// * `values` - Array of values (must match keys length)
    ///
    /// # Returns
    ///
    /// True if successful, false if arrays have different lengths
    pub fn set_batch(keys: Array<ByteString>, values: Array<ByteString>) -> bool {
        // Check if arrays have the same length
        if keys.len() != values.len() {
            return false;
        }
        
        let mut storage = StorageMap::new();
        
        // Store each key-value pair
        for i in 0..keys.len() {
            let key = keys.get(i);
            let value = values.get(i);
            storage.put(key, value);
        }
        
        true
    }
    
    /// Returns the contract's name.
    ///
    /// # Returns
    ///
    /// The name of the contract
    pub fn name() -> ByteString {
        ByteString::from("SimpleStorage")
    }
    
    /// Returns information about the contract.
    ///
    /// # Returns
    ///
    /// A description of the contract
    pub fn contract_info() -> ByteString {
        ByteString::from("A simple storage contract example for Neo N3 written in Rust")
    }
} 