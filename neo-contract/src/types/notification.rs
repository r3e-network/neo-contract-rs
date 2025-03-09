// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use core::fmt;
use crate::types::builtin::array::Array;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;

/// Notification represents a notification from a contract.
/// It includes the script hash of the contract, the name of the event, and the state (arguments) of the event.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Notification {
    /// The script hash of the contract that emitted the notification.
    pub script_hash: H160,
    /// The name of the event.
    pub event_name: ByteString,
    /// The arguments of the event.
    pub state: Array,
}

impl Notification {
    /// Create a new notification.
    pub fn new(script_hash: H160, event_name: ByteString, state: Array) -> Self {
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
