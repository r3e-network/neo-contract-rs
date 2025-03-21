// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![no_std]
#![no_main]

use neo_contract as neo;
use neo::{contract::*, types::*};
use neo_contract::storage::StorageMap;

/// HelloWorld is a simple example contract demonstrating annotations
/// for NEO N3 smart contract development with Rust.
/// 
/// @contract_author("Neo Contract Rust Team")
/// @contract_permission("*:*")
/// @contract_meta("Version", "1.0.0")
/// @contract_meta("Website", "https://example.com/hello-world")
pub struct HelloWorld;

#[neo::contract]
impl Nep17Token for HelloWorld {
    /// @method
    /// @safe
    fn symbol() -> ByteString {
        ByteString::from_literal("HELLO")
    }

    /// @method
    /// @safe
    fn decimals() -> u32 {
        8
    }
}

// Additional methods for the HelloWorld contract
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
    /// 
    /// @method
    /// @safe
    /// @wasm_export(name = "add")
    pub fn hello(name: &ByteString) -> ByteString {
        if name.is_empty() {
            return ByteString::from_literal("Hello, World!");
        }
        
        // Create a personalized greeting
        let mut result = ByteString::from_literal("Hello, ");
        result = result.concat(name);
        result = result.concat(&ByteString::from_literal("!"));
        
        result
    }
    
    /// Returns information about the contract
    ///
    /// # Returns
    ///
    /// A ByteString containing contract information
    /// 
    /// @method
    /// @safe
    /// @wasm_export(name = "flip")
    pub fn contract_info() -> ByteString {
        ByteString::from_literal("Hello World Contract - A simple Neo N3 smart contract example written in Rust")
    }
    
    /// Stores a greeting message in contract storage
    ///
    /// # Arguments
    ///
    /// * `name` - The name to associate with the greeting
    /// * `message` - The custom greeting message to store
    /// 
    /// @method
    /// @wasm_export(name = "option")
    pub fn store_greeting(name: &ByteString, message: &ByteString) {
        let mut storage = StorageMap::new();
        storage.put(name.clone(), message.clone());
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
    /// 
    /// @method
    /// @safe
    /// @wasm_export(name = "main")
    pub fn get_greeting(name: &ByteString) -> ByteString {
        let storage = StorageMap::new();
        let value = storage.get(name.clone());
        
        if value.is_null() {
            return ByteString::empty();
        }
        
        value.unwrap()
    }
} 