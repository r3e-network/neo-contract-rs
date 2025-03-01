// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::types::builtin::array::Array;
use crate::types::builtin::h160::H160;
use crate::types::builtin::h256::H256;
use crate::types::builtin::string::ByteString;
use crate::types::notification::Notification;
use crate::types::tx::TriggerType;

/// Get the executing script hash
pub fn get_executing_script_hash() -> H160 {
    H160::zero()
}

/// Get the calling script hash
pub fn get_calling_script_hash() -> H160 {
    H160::zero()
}

/// Get the entry script hash
pub fn get_entry_script_hash() -> H160 {
    H160::zero()
}

/// Check if the witness is valid
pub fn check_witness(_hash: &H160) -> bool {
    false
}

/// Get the platform
pub fn platform() -> ByteString {
    ByteString::from("NEO")
}

/// Get the trigger
pub fn trigger() -> TriggerType {
    TriggerType::Application
}

/// Get the gas left
pub fn gas_left() -> i64 {
    0
}

/// Get the invocation counter
pub fn invocation_counter() -> i32 {
    0
}

/// Get the time
pub fn time() -> u64 {
    0
}

/// Notify an event
pub fn notify(event_name: &str, arg: &[u8]) -> Notification {
    Notification::new(ByteString::from(event_name), Vec::from(arg))
}

/// Log a message
pub fn log(message: &str) {
    // In a real implementation, this would log the message
}

/// Get the notifications
pub fn get_notifications(_hash: H160) -> Array<Notification> {
    Array::new()
}

/// Get the random
pub fn get_random() -> u64 {
    0
}

/// Get the network
pub fn network() -> u8 {
    0
}
