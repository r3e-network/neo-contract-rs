//! Events for Neo Contract RS
//!
//! This module provides functionality for emitting events from smart contracts.

use alloc::string::String;
use alloc::vec::Vec;
use crate::types::builtin::any::Any;

/// Represents an event that can be emitted from a smart contract
#[derive(Debug, Clone)]
pub struct Event {
    /// The name of the event
    pub name: String,
    
    /// The event parameters (name-value pairs)
    pub params: Vec<(String, Any)>,
}

impl Event {
    /// Creates a new event with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Event {
            name: name.into(),
            params: Vec::new(),
        }
    }
    
    /// Adds a parameter to the event
    pub fn add_param(&mut self, name: impl Into<String>, value: impl Into<Any>) {
        self.params.push((name.into(), value.into()));
    }
    
    /// Creates a new event with the given name and adds a parameter
    pub fn with_param(mut self, name: impl Into<String>, value: impl Into<Any>) -> Self {
        self.add_param(name, value);
        self
    }
    
    /// Returns the parameter with the given name
    pub fn get_param(&self, name: &str) -> Option<&Any> {
        self.params.iter()
            .find(|(param_name, _)| param_name == name)
            .map(|(_, value)| value)
    }
    
    /// Emits the event
    pub fn emit(&self) {
        // In a real implementation, this would call into the Neo VM
        // to emit the event. For now, we'll just provide the function
        // signature, and the actual implementation will be provided
        // by the WASM to Neo converter.
        
        // The Neo VM needs to know the event name and parameters
        // The event name is self.name
        // The parameters are in self.params
        
        // This is a placeholder and will be replaced with actual code
        // that interfaces with the Neo VM
    }
}

/// Helper function to create and emit an event
pub fn emit(name: impl Into<String>, params: Vec<(String, Any)>) {
    let mut event = Event::new(name);
    for (param_name, param_value) in params {
        event.add_param(param_name, param_value);
    }
    event.emit();
}

/// NEP-17 transfer event
pub fn nep17_transfer(from: Option<&[u8]>, to: Option<&[u8]>, amount: u64) {
    let mut event = Event::new("Transfer");
    
    // NEP-17 specifications require these parameters
    event.add_param("from", match from {
        Some(addr) => Any::byte_string(addr),
        None => Any::null(),
    });
    
    event.add_param("to", match to {
        Some(addr) => Any::byte_string(addr),
        None => Any::null(),
    });
    
    event.add_param("amount", Any::integer(amount));
    
    event.emit();
}

/// NEP-11 transfer event
pub fn nep11_transfer(from: Option<&[u8]>, to: Option<&[u8]>, amount: u64, token_id: &[u8]) {
    let mut event = Event::new("Transfer");
    
    // NEP-11 specifications require these parameters
    event.add_param("from", match from {
        Some(addr) => Any::byte_string(addr),
        None => Any::null(),
    });
    
    event.add_param("to", match to {
        Some(addr) => Any::byte_string(addr),
        None => Any::null(),
    });
    
    event.add_param("amount", Any::integer(amount));
    event.add_param("tokenId", Any::byte_string(token_id));
    
    event.emit();
}
