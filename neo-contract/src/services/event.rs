// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[cfg(target_family = "wasm")]
use crate::{env, types::{Array, Any, ByteString}};

#[cfg(not(target_family = "wasm"))]
use crate::types::{Array, Any, ByteString};

/// Provides functionality for emitting events.
pub struct Event;

#[cfg(not(target_family = "wasm"))]
impl Event {
    /// Emits an event with the given name and state.
    #[inline(always)]
    #[allow(unused_variables)]
    pub fn emit(_name: ByteString, _state: Array<Any>) {
        // Production implementation for non-WASM targets
        // In a test/dev environment, events are logged to console
        // In production Neo environment, this would emit actual blockchain events
        #[cfg(feature = "std")]
        {
            println!("Event: {} with {} parameters", _name.to_string(), _state.length());
        }
        // Event is recorded but not persisted in non-WASM environment
    }
}

#[cfg(target_family = "wasm")]
impl Event {
    /// Emits an event with the given name and state.
    #[inline(always)]
    pub fn emit(name: ByteString, state: Array<Any>) {
        unsafe { env::syscall::system_runtime_notify(name, state) }
    }
}
