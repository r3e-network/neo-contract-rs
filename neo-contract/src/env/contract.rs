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
use crate::types::builtin::any::Any;

// For non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn executing_script_hash() -> H160 {
    unsafe { crate::env::syscall_non_wasm::system_runtime_executing_script_hash() }
}

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_executing_script_hash() -> H160 {
    // In the wasm environment, this would call the native executing_script_hash function
    H160::zero()
}

// For non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn calling_script_hash() -> H160 {
    unsafe { crate::env::syscall_non_wasm::system_runtime_calling_script_hash() }
}

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_calling_script_hash() -> H160 {
    // In the wasm environment, this would call the native calling_script_hash function
    H160::zero()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn entry_script_hash() -> H160 {
    unsafe { crate::env::syscall_non_wasm::system_runtime_entry_script_hash() }
}

#[cfg(target_arch = "wasm32")]
pub fn native_entry_script_hash() -> H160 {
    // In the wasm environment, this would call the native entry_script_hash function
    H160::zero()
}

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_check_witness(hash: H160) -> bool {
    // In the wasm environment, this would call the native check_witness function
    true
}

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_contract_call(hash: H160, method: ByteString, flags: CallFlags, args: Array<Any>) -> Any {
    // In the wasm environment, this would call the native contract_call function
    Any::default()
}

// For non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn check_witness(hash: H160) -> bool {
    unsafe { crate::env::syscall_non_wasm::system_runtime_check_witness(hash) }
}

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_platform() -> ByteString {
    // In the wasm environment, this would call the native platform function
    ByteString::default()
}

// For non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn platform() -> ByteString {
    unsafe { crate::env::syscall_non_wasm::system_runtime_platform() }
}

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_gas_left() -> i64 {
    // In the wasm environment, this would call the native gas_left function
    0
}

// For non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn gas_left() -> i64 {
    unsafe { crate::env::syscall_non_wasm::system_runtime_gas_left() }
}

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_invocation_counter() -> i32 {
    // In the wasm environment, this would call the native invocation_counter function
    0
}

// For non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn invocation_counter() -> i32 {
    unsafe { crate::env::syscall_non_wasm::system_runtime_invocation_counter() }
}

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_time() -> u64 {
    // In the wasm environment, this would call the native time function
    0
}

// For non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn time() -> u64 {
    unsafe { crate::env::syscall_non_wasm::system_runtime_time() }
}

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_log(message: &str) {
    // In the wasm environment, this would call the native log function
}

// For non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn log(message: &str) {
    unsafe { crate::env::syscall_non_wasm::system_runtime_log(ByteString::from(message)) }
}

// Native contract hash functions for wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_oracle_contract_hash() -> H160 {
    // In the wasm environment, this would call the native oracle_contract_hash function
    H160::zero()
}

#[cfg(target_arch = "wasm32")]
pub fn native_policy_contract_hash() -> H160 {
    // In the wasm environment, this would call the native policy_contract_hash function
    H160::zero()
}

#[cfg(target_arch = "wasm32")]
pub fn native_role_management_contract_hash() -> H160 {
    // In the wasm environment, this would call the native role_management_contract_hash function
    H160::zero()
}
