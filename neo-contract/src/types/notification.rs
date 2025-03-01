// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;

/// Notification represents a Neo notification
#[derive(Debug, Clone)]
pub struct Notification {
    sender: H160,
    script_hash: H160,
    event_name: ByteString,
    state: Vec<u8>,
}

impl Notification {
    /// Create a new notification
    pub fn new(event_name: ByteString, state: Vec<u8>) -> Self {
        Self {
            sender: H160::zero(),
            script_hash: H160::zero(),
            event_name,
            state,
        }
    }

    /// Get the sender
    pub fn sender(&self) -> H160 {
        self.sender.clone()
    }

    /// Get the script hash
    pub fn script_hash(&self) -> H160 {
        self.script_hash.clone()
    }

    /// Get the event name
    pub fn event_name(&self) -> ByteString {
        self.event_name.clone()
    }

    /// Get the state
    pub fn state(&self) -> Vec<u8> {
        self.state.clone()
    }
}
