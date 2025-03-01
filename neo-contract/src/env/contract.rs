// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::vec::Vec;
use crate::call_flags::CallFlags;
use crate::types::builtin::array::Array;
use crate::types::builtin::h160::H160;
use crate::types::builtin::h256::H256;
use crate::types::builtin::int256::Int256;
use crate::types::builtin::string::ByteString;
use crate::types::bytes::Bytes;
use crate::types::context::StorageContext;
use crate::types::notification::Notification;
use crate::types::placeholder::Placeholder;

/// Get the executing script hash
pub fn executing_script_hash() -> H160 {
    unsafe { crate::env::syscall_non_wasm::system_runtime_executing_script_hash() }
}

/// Get the calling script hash
pub fn calling_script_hash() -> H160 {
    unsafe { crate::env::syscall_non_wasm::system_runtime_calling_script_hash() }
}

/// Get the entry script hash
pub fn entry_script_hash() -> H160 {
    unsafe { crate::env::syscall_non_wasm::system_runtime_entry_script_hash() }
}

/// Check if the witness is valid
pub fn check_witness(hash: H160) -> bool {
    unsafe { crate::env::syscall_non_wasm::system_runtime_check_witness(hash) }
}

/// Get the platform
pub fn platform() -> ByteString {
    unsafe { crate::env::syscall_non_wasm::system_runtime_platform() }
}

/// Get the gas left
pub fn gas_left() -> i64 {
    unsafe { crate::env::syscall_non_wasm::system_runtime_gas_left() }
}

/// Get the invocation counter
pub fn invocation_counter() -> i32 {
    unsafe { crate::env::syscall_non_wasm::system_runtime_invocation_counter() }
}

/// Get the time
pub fn time() -> u64 {
    unsafe { crate::env::syscall_non_wasm::system_runtime_time() }
}

/// Log a message
pub fn log(message: &str) {
    unsafe { crate::env::syscall_non_wasm::system_runtime_log(ByteString::from(message)) }
}
