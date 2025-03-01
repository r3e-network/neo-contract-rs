// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use core::fmt;
use crate::types::builtin::array::Array;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;

/// Notification represents a notification from a smart contract
#[derive(Debug, Clone)]
pub struct Notification {
    /// Script hash
    pub script_hash: H160,
    /// Event name
    pub event_name: ByteString,
    /// State
    pub state: Array<ByteString>,
}

impl Notification {
    /// Create a new notification
    pub fn new(script_hash: H160, event_name: ByteString, state: Array<ByteString>) -> Self {
        Self {
            script_hash,
            event_name,
            state,
        }
    }
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Notification(script_hash: {}, event_name: {})", self.script_hash, self.event_name)
    }
}
