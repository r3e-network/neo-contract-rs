// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! System calls for non-WASM environments.
//! This module provides access to the Neo N3 VM system calls from native code.
//! These functions are only available in non-WASM environments.

use crate::call_flags::CallFlags;
use crate::types::builtin::array::Array;
use crate::types::builtin::h160::H160;
use crate::types::builtin::h256::H256;

use crate::types::builtin::any::Any;
use crate::types::builtin::string::ByteString;

use crate::prelude::FindOptions;
use crate::types::context::StorageContext;

/// Get the executing script hash
pub unsafe fn system_runtime_executing_script_hash() -> H160 {
    // Stub implementation
    H160::zero()
}

/// Get the calling script hash
pub unsafe fn system_runtime_calling_script_hash() -> H160 {
    // Stub implementation
    H160::zero()
}

/// Get the entry script hash
pub unsafe fn system_runtime_entry_script_hash() -> H160 {
    // In a real implementation, this would call the runtime entry script hash syscall
    H160::zero()
}

/// Check if the witness is valid
pub unsafe fn system_runtime_check_witness(_hash: H160) -> bool {
    // Stub implementation
    false
}

/// Get the platform
pub unsafe fn system_runtime_platform() -> ByteString {
    // Stub implementation
    ByteString::from("NEO")
}

/// Get the gas left
pub unsafe fn system_runtime_gas_left() -> i64 {
    // Stub implementation
    0
}

/// Get the invocation counter
pub unsafe fn system_runtime_invocation_counter() -> i32 {
    // In a real implementation, this would call the runtime invocation counter syscall
    0
}

/// Get the time
pub unsafe fn system_runtime_time() -> u64 {
    // Stub implementation
    0
}

/// Notify an event
pub unsafe fn system_runtime_notify(_event_name: ByteString, _args: Array) {
    // Stub implementation
}

/// Log a message
pub unsafe fn system_runtime_log(_message: ByteString) {
    // Stub implementation
}

/// Call a contract
pub unsafe fn system_contract_call(_hash: H160, _method: ByteString, _flags: CallFlags, _args: Array) -> Any {
    // Stub implementation
    Any::default()
}

/// Create a contract
pub unsafe fn system_contract_create(_nef: ByteString, _manifest: ByteString, _call_flags: CallFlags) -> Any {
    // In a real implementation, this would call the contract create syscall
    Any::default()
}

/// Get the call flags
pub unsafe fn system_contract_get_call_flags() -> CallFlags { CallFlags::ALL }

/// Get candidates
pub unsafe fn system_neo_get_candidates() -> Array {
    // Stub implementation
    Array::new()
}

/// Get committee
pub unsafe fn system_neo_get_committee() -> Array {
    // Stub implementation
    Array::new()
}

/// Get next block validators
pub unsafe fn system_neo_get_next_block_validators() -> Array {
    // Stub implementation
    Array::new()
}

/// Get the storage context
pub unsafe fn system_storage_get_context() -> StorageContext {
    // Stub implementation
    StorageContext::default()
}

/// Get a read-only storage context
pub unsafe fn system_storage_get_read_only_context() -> StorageContext {
    // Stub implementation
    StorageContext::default()
}

/// Convert a storage context to a read-only storage context
pub unsafe fn system_storage_as_readonly(context: StorageContext) -> StorageContext {
    // In a real implementation, this would call the storage as readonly syscall
    context
}

/// Put a value in storage
pub unsafe fn system_storage_put(_context: StorageContext, _key: ByteString, _value: ByteString) {
    // Stub implementation
}

/// Get a value from storage
pub unsafe fn system_storage_get(_context: StorageContext, _key: ByteString) -> ByteString {
    // Stub implementation
    ByteString::default()
}

/// Delete a value from storage
pub unsafe fn system_storage_delete(_context: StorageContext, _key: ByteString) {
    // Stub implementation
}

/// Find values in storage
pub unsafe fn system_storage_find(_context: StorageContext, _prefix: ByteString) -> i32 {
    // Stub implementation
    0
}

/// Find values in storage with options
pub unsafe fn system_storage_find_with_options(
    _context: StorageContext,
    _prefix: ByteString,
    _options: FindOptions,
) -> i32 {
    // Stub implementation
    0
}

/// Check a signature
pub unsafe fn system_crypto_check_sign(_public_key: ByteString, _sign: ByteString) -> bool {
    // Stub implementation
    false
}

/// Check multiple signatures
pub unsafe fn system_crypto_check_multi_signs(_public_keys: Array, _signs: Array) -> bool {
    // Stub implementation
    false
}

/// Get the current trigger type
pub unsafe fn system_runtime_trigger() -> u32 {
    // Stub implementation
    0
}

/// Get the current network ID
pub unsafe fn system_runtime_get_network() -> i32 {
    // Stub implementation
    0
}

/// Get random number
pub unsafe fn system_runtime_get_random() -> u64 {
    // Stub implementation
    0
}

/// Get the notifications from a transaction
pub unsafe fn system_runtime_get_notifications(_hash: H160) -> Array {
    // Stub implementation
    Array::new()
}

/// Get the current block hash
pub unsafe fn system_runtime_get_current_block_hash() -> H256 {
    // Stub implementation
    H256::zero()
}

/// Enter the native contract context
pub unsafe fn system_runtime_enter_script(_script_hash: H160) {
    // Stub implementation
}

/// Get the invocation counter
pub unsafe fn system_runtime_get_invocation_counter() -> i32 {
    // Stub implementation
    0
}

/// Check if the hash is a contract
pub unsafe fn system_contract_is_contract(_hash: H160) -> bool {
    // Stub implementation
    false
}

/// Update the contract
pub unsafe fn system_contract_update(_script: ByteString, _manifest: ByteString, _data: Any) -> bool {
    // Stub implementation
    false
}

/// Destroy the contract
pub unsafe fn system_contract_destroy() {
    // Stub implementation
}

/// Verify signature with ECDSA
pub unsafe fn system_crypto_verify_with_ecdsa(
    _message: ByteString,
    _pubkey: ByteString,
    _signature: ByteString,
    _curve: u32,
) -> bool {
    // Stub implementation
    false
}

/// Calculate SHA256 hash
pub unsafe fn system_crypto_sha256(_data: ByteString) -> H256 {
    // Stub implementation
    H256::zero()
}

/// Calculate RIPEMD160 hash
pub unsafe fn system_crypto_ripemd160(_data: ByteString) -> H160 {
    // Stub implementation
    H160::zero()
}

/// Generic hash function
pub unsafe fn system_crypto_hash(_data: ByteString, _hash_type: u32) -> ByteString {
    // Stub implementation
    ByteString::default()
}

/// Check multisig
pub unsafe fn system_crypto_check_multisig(_message: ByteString, _signatures: Array, _public_keys: Array) -> bool {
    // Stub implementation
    false
}

/// Convert script hash to address
pub unsafe fn system_crypto_to_address(_script_hash: H160) -> ByteString {
    // Stub implementation
    ByteString::default()
}

/// Convert address to script hash
pub unsafe fn system_crypto_to_script_hash(_address: ByteString) -> H160 {
    // Stub implementation
    H160::zero()
}

/// Generate BLS signature
pub unsafe fn system_crypto_bls_generate(_message: ByteString, _private_key: ByteString) -> ByteString {
    // Stub implementation
    ByteString::default()
}

/// Verify BLS signature
pub unsafe fn system_crypto_bls_verify(_message: ByteString, _signature: ByteString, _public_key: ByteString) -> bool {
    // Stub implementation
    false
}
