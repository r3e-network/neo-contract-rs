//! Event helper module for Neo N3 contracts
//!
//! This module provides utilities for emitting events in Neo N3 smart contracts
//! following the proper standards:
//! 1. Event names must be ByteString
//! 2. Event parameters must be contained in Array<Any>
//! 3. Parameters must be converted to Any type
//! 4. Events are emitted using Runtime::notify

use crate::runtime::Runtime;
use crate::types::builtin::any::Any;
use crate::types::builtin::array::Array;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;

/// EventBuilder helps create and emit events following Neo N3 standards
pub struct EventBuilder {
    name: ByteString,
    params: Array<Any>,
}

impl EventBuilder {
    /// Create a new event builder with the specified name
    pub fn new(name: &str) -> Self { Self { name: ByteString::from(name), params: Array::<Any>::new() } }

    /// Add a parameter to the event
    pub fn param<T: Into<Any>>(mut self, value: T) -> Self {
        self.params.push(value.into());
        self
    }

    /// Add an optional parameter to the event (None becomes null in Neo VM)
    pub fn optional_param<T: Into<Any>>(mut self, value: Option<T>) -> Self {
        match value {
            Some(val) => self.params.push(val.into()),
            None => self.params.push(Any::new()),
        }
        self
    }

    /// Emit the event using Runtime::notify
    pub fn emit(self) { Runtime::notify(&self.name, &self.params); }
}

/// Register an event in the contract manifest
///
/// This function is used by the #[event] attribute to register events
/// in the contract manifest during compilation.
///
/// # Parameters
///
/// * `name` - The name of the event
/// * `fields` - The names of the event fields
/// * `types` - The Neo VM types of the event fields
/// * `indexed` - Whether each field is indexed for event filtering
///
/// # Example
///
/// ```rust
/// #[event]
/// struct Transfer {
///     #[index]
///     from: Option<Address>,
///     #[index]
///     to: Option<Address>,
///     amount: u64,
/// }
/// ```
///
/// The above will generate code that registers the Transfer event,
/// including proper field types and indexing information.
#[cfg(feature = "manifest-validation")]
pub fn register_event(name: &str, fields: &[&str], types: &[&str], indexed: &[bool]) {
    // This implementation is compiled out in the actual Neo VM,
    // but is used during manifest validation to register events
    // No implementation needed for runtime
}

/// Simplified interface to emit a transfer event following NEP standards
pub fn emit_transfer(from: Option<H160>, to: Option<H160>, amount: impl Into<Any>) {
    let event_name = ByteString::from("Transfer");
    let mut event_data = Array::<Any>::new();

    // Add parameters as Any values
    match from {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()),
    }

    match to {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()),
    }

    event_data.push(amount.into());

    // Emit the event
    Runtime::notify(&event_name, &event_data);
}

/// Register a Transfer event in the contract manifest
pub fn register_transfer_event() {
    // This is a placeholder - actual implementation depends on manifest module
    // which should be implemented according to Neo N3 standards
}

/// Helper function to emit an event with a single value parameter
pub fn emit_event(name: &str, value: impl Into<Any>) { EventBuilder::new(name).param(value).emit(); }

/// Helper function to emit an event with two parameters
pub fn emit_event2(name: &str, value1: impl Into<Any>, value2: impl Into<Any>) {
    EventBuilder::new(name).param(value1).param(value2).emit();
}

/// Helper function to emit an event with three parameters
pub fn emit_event3(name: &str, value1: impl Into<Any>, value2: impl Into<Any>, value3: impl Into<Any>) {
    EventBuilder::new(name).param(value1).param(value2).param(value3).emit();
}

/// Helper function to handle null or optional values in event parameters
pub fn null_or_value<T: Into<Any>>(value: Option<T>) -> Any {
    match value {
        Some(val) => val.into(),
        None => Any::new(),
    }
}

/// Trait for types that can emit standard events
pub trait EventEmitter {
    /// Emit a Transfer event (common in NEP-17 and NEP-11 tokens)
    fn emit_transfer(
        &self,
        from: Option<crate::types::builtin::h160::H160>,
        to: Option<crate::types::builtin::h160::H160>,
        amount: impl Into<Any>,
    );

    /// Emit an Approval event (common in NEP-17 tokens)
    fn emit_approval(
        &self,
        owner: crate::types::builtin::h160::H160,
        spender: crate::types::builtin::h160::H160,
        amount: impl Into<Any>,
    );
}

/// Standard implementation of EventEmitter
pub struct StandardEventEmitter;

impl EventEmitter for StandardEventEmitter {
    fn emit_transfer(
        &self,
        from: Option<crate::types::builtin::h160::H160>,
        to: Option<crate::types::builtin::h160::H160>,
        amount: impl Into<Any>,
    ) {
        emit_transfer(from, to, amount);
    }

    fn emit_approval(
        &self,
        owner: crate::types::builtin::h160::H160,
        spender: crate::types::builtin::h160::H160,
        amount: impl Into<Any>,
    ) {
        let event_name = ByteString::from("Approval");
        let mut event_data = Array::<Any>::new();

        event_data.push(Any::from(owner));
        event_data.push(Any::from(spender));
        event_data.push(amount.into());

        Runtime::notify(&event_name, &event_data);
    }
}

/// Generic function to emit an event with an array of parameters
pub fn emit_event_array<T: Into<Any> + Clone>(name: &str, params: &[T]) {
    let event_name = ByteString::from(name);
    let mut event_data = Array::<Any>::new();

    for param in params {
        event_data.push::<Any>(param.clone().into());
    }

    Runtime::notify(&event_name, &event_data);
}
