//! Events for Neo Contract RS
//!
//! This module provides functionality for emitting events from Neo N3 smart contracts.
//!
//! Neo N3 events should be emitted using the Runtime::notify method as specified in the
//! Neo N3 documentation. This module follows the Neo N3 standards for event emission.

use crate::runtime::Runtime;
use crate::types::builtin::any::Any;
use crate::types::builtin::array::Array;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use alloc::vec::Vec;

/// Represents an event that can be emitted from a Neo N3 smart contract
///
/// This implementation follows the Neo N3 standards for event emission,
/// using Runtime::notify with properly structured parameters.
#[derive(Debug, Clone)]
pub struct Event {
    /// The name of the event as a ByteString
    pub name: ByteString,

    /// The event parameters as Array<Any>
    pub params: Array<Any>,
}

impl Event {
    /// Creates a new event with the given name
    pub fn new(name: impl Into<ByteString>) -> Self { Event { name: name.into(), params: Array::new() } }

    /// Adds a parameter to the event
    pub fn add_param(&mut self, value: impl Into<Any>) { self.params.push(value.into()); }

    /// Creates a new event with the given name and adds a parameter
    pub fn with_param(mut self, value: impl Into<Any>) -> Self {
        self.add_param(value);
        self
    }

    /// Emits the event using Runtime::notify as per Neo N3 standards
    pub fn emit(&self) {
        // Use Runtime::notify to emit the event in Neo N3 blockchain
        Runtime::notify(&self.name, &self.params);
    }
}

/// Helper function to create and emit an event following Neo N3 standards
pub fn emit(name: impl Into<ByteString>, params: Vec<Any>) {
    let mut event = Event::new(name);
    for param_value in params {
        event.add_param(param_value);
    }
    event.emit();
}

/// NEP-17 Transfer event implementation following Neo N3 standards
///
/// This implements the Transfer event as defined in the NEP-17 token standard for Neo N3.
/// The event has the following parameters:
/// - from: The address tokens are transferred from (Option<H160>)
/// - to: The address tokens are transferred to (Option<H160>)
/// - amount: The amount of tokens transferred (u64)
pub fn nep17_transfer(from: Option<H160>, to: Option<H160>, amount: u64) {
    // Create event name as ByteString as required by Neo N3
    let event_name = ByteString::from("Transfer");

    // Create Array<Any> for parameters
    let mut event_data = Array::<Any>::new();

    // Add the from parameter (null if None)
    match from {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()), // Use empty Any for null
    }

    // Add the to parameter (null if None)
    match to {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()), // Use empty Any for null
    }

    // Add the amount parameter
    event_data.push(Any::from(amount));

    // Emit using Runtime::notify as required for Neo N3
    Runtime::notify(&event_name, &event_data);
}

/// NEP-11 Transfer event implementation following Neo N3 standards
///
/// This implements the Transfer event as defined in the NEP-11 token standard for Neo N3.
/// The event has the following parameters:
/// - from: The address tokens are transferred from (Option<H160>)
/// - to: The address tokens are transferred to (Option<H160>)
/// - amount: The amount of tokens transferred (usually 1 for NFTs) (u64)
/// - tokenId: The unique identifier of the token being transferred (ByteString)
pub fn nep11_transfer(from: Option<H160>, to: Option<H160>, amount: u64, token_id: ByteString) {
    // Create event name as ByteString as required by Neo N3
    let event_name = ByteString::from("Transfer");

    // Create Array<Any> for parameters
    let mut event_data = Array::<Any>::new();

    // Add the from parameter (null if None)
    match from {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()), // Use empty Any for null
    }

    // Add the to parameter (null if None)
    match to {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()), // Use empty Any for null
    }

    // Add the amount parameter
    event_data.push(Any::from(amount));

    // Add the tokenId parameter
    event_data.push(Any::from(token_id));

    // Emit using Runtime::notify as required for Neo N3
    Runtime::notify(&event_name, &event_data);
}
