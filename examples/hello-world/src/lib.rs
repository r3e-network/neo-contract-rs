// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![no_std]
#![no_main]

use neo_contract as neo;
use neo::{contract::*, types::*};

pub struct HelloWorld;

#[neo::contract]
impl HelloWorld {
    /// Returns a greeting message
    ///
    /// If no name is provided, returns a generic "Hello, World!" message.
    /// Otherwise, returns a personalized greeting with the provided name.
    ///
    /// # Arguments
    ///
    /// * `name` - Optional name to personalize the greeting
    ///
    /// # Returns
    ///
    /// A ByteString containing the greeting message
    pub fn hello(name: ByteString) -> ByteString {
        if name.is_empty() {
            return ByteString::from("Hello, World!");
        }
        
        // Create a personalized greeting
        let mut result = ByteString::from("Hello, ");
        result = result.concat(&name);
        result = result.concat(&ByteString::from("!"));
        
        result
    }
    
    /// Returns information about the contract
    ///
    /// # Returns
    ///
    /// A ByteString containing contract information
    pub fn contract_info() -> ByteString {
        ByteString::from("Hello World Contract - A simple Neo N3 smart contract example written in Rust")
    }
    
    /// Stores a greeting message in contract storage
    ///
    /// # Arguments
    ///
    /// * `name` - The name to associate with the greeting
    /// * `message` - The custom greeting message to store
    pub fn store_greeting(name: ByteString, message: ByteString) {
        let mut storage = StorageMap::new();
        storage.put(name, message);
    }
    
    /// Retrieves a stored greeting message
    ///
    /// # Arguments
    ///
    /// * `name` - The name associated with the greeting
    ///
    /// # Returns
    ///
    /// The stored greeting message, or an empty ByteString if not found
    pub fn get_greeting(name: ByteString) -> ByteString {
        let storage = StorageMap::new();
        let value = storage.get(name);
        
        if value.is_null() {
            return ByteString::empty();
        }
        
        value.unwrap()
    }
} 