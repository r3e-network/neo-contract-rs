// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Type-safe event system for Neo smart contracts
//! This module provides a structured way to define and emit events
//! in Neo smart contracts, ensuring type safety.

use alloc::string::String;
use alloc::vec::Vec;
use core::marker::PhantomData;

use crate::builtin::{Any, Array, ByteString};
use crate::Runtime;

/// Trait for types that can be converted to an event parameter
pub trait EventParam {
    /// Convert the type to an `Any` value for event emission
    fn to_event_param(&self) -> Any;
}

// Implement EventParam for common types
impl EventParam for crate::builtin::H160 {
    fn to_event_param(&self) -> Any {
        Any::from(self.clone())
    }
}

impl EventParam for crate::builtin::Int256 {
    fn to_event_param(&self) -> Any {
        Any::from(self.clone())
    }
}

impl EventParam for ByteString {
    fn to_event_param(&self) -> Any {
        Any::from(self.clone())
    }
}

impl EventParam for bool {
    fn to_event_param(&self) -> Any {
        Any::from(*self)
    }
}

impl EventParam for u8 {
    fn to_event_param(&self) -> Any {
        Any::from(*self as i64)
    }
}

impl EventParam for i32 {
    fn to_event_param(&self) -> Any {
        Any::from(*self as i64)
    }
}

impl EventParam for i64 {
    fn to_event_param(&self) -> Any {
        Any::from(*self)
    }
}

impl<T: EventParam> EventParam for Option<T> {
    fn to_event_param(&self) -> Any {
        match self {
            Some(value) => value.to_event_param(),
            None => Any::new(),
        }
    }
}

/// Base trait for event definitions
pub trait Event {
    /// Get the event name
    fn name() -> ByteString;
    
    /// Convert event to parameters for emission
    fn to_params(&self) -> Array<Any>;
    
    /// Emit the event
    fn emit(&self) {
        let name = Self::name();
        let params = self.to_params();
        Runtime::notify(&name, &params);
    }
}

/// Helper struct for creating typed events
#[derive(Debug, Clone)]
pub struct EventBuilder<T> {
    name: ByteString,
    _phantom: PhantomData<T>,
}

impl<T> EventBuilder<T> {
    /// Create a new event builder
    pub fn new(name: &str) -> Self {
        Self {
            name: ByteString::from(name),
            _phantom: PhantomData,
        }
    }
    
    /// Get the event name
    pub fn name(&self) -> ByteString {
        self.name.clone()
    }
}

/// Macro for defining typed events
#[macro_export]
macro_rules! define_event {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident : $field_type:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $name {
            $(
                $(#[$field_meta])*
                $field_vis $field_name: $field_type,
            )*
            _builder: $crate::events::EventBuilder<Self>,
        }
        
        impl $name {
            /// Creates a new event instance
            $vis fn new(
                $($field_name: $field_type,)*
            ) -> Self {
                Self {
                    $($field_name,)*
                    _builder: $crate::events::EventBuilder::new(stringify!($name)),
                }
            }
            
            /// Emits this event
            $vis fn emit(&self) {
                $crate::events::Event::emit(self);
            }
        }
        
        impl $crate::events::Event for $name {
            fn name() -> $crate::builtin::ByteString {
                $crate::builtin::ByteString::from(stringify!($name))
            }
            
            fn to_params(&self) -> $crate::builtin::Array<$crate::builtin::Any> {
                let mut params = $crate::builtin::Array::new();
                $(
                    params.push($crate::events::EventParam::to_event_param(&self.$field_name));
                )*
                params
            }
        }
    };
}

// Standard events

/// Define the NEP-17 Transfer event
#[derive(Debug, Clone)]
pub struct Transfer {
    /// The address tokens are transferred from (None for minting)
    pub from: Option<crate::builtin::H160>,
    /// The address tokens are transferred to (None for burning)
    pub to: Option<crate::builtin::H160>,
    /// The amount of tokens transferred
    pub amount: crate::builtin::Int256,
    /// Event builder
    _builder: EventBuilder<Self>,
}

impl Transfer {
    /// Create a new Transfer event
    pub fn new(from: Option<crate::builtin::H160>, to: Option<crate::builtin::H160>, amount: crate::builtin::Int256) -> Self {
        Self {
            from,
            to,
            amount,
            _builder: EventBuilder::new("Transfer"),
        }
    }
    
    /// Emit this event
    pub fn emit(&self) {
        Event::emit(self);
    }
}

impl Event for Transfer {
    fn name() -> ByteString {
        ByteString::from("Transfer")
    }
    
    fn to_params(&self) -> Array<Any> {
        let mut params = Array::new();
        params.push(self.from.to_event_param());
        params.push(self.to.to_event_param());
        params.push(self.amount.to_event_param());
        params
    }
}
