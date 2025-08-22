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
    pub fn emit(name: ByteString, state: Array<Any>) {
        #[cfg(target_family = "wasm")]
        {
            // Production WASM implementation using Neo blockchain event system
            use crate::runtime;
            runtime::Runtime::notify(name, &[state.into()]);
        }
        
        #[cfg(not(target_family = "wasm"))]
        {
            // Native implementation for testing - structured event logging
            #[cfg(feature = "std")]
            {
                println!("Neo Event [{}]: {} parameters emitted", name.to_string(), state.length());
            }
            // In testing environment, events are logged but not persisted to blockchain
        }
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
