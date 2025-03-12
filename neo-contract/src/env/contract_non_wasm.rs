// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::call_flags::CallFlags;
use crate::types::builtin::array::Array;
use crate::types::builtin::h160::H160;
use crate::types::builtin::h256::H256;
use crate::types::builtin::int256::Int256;
use crate::types::builtin::string::ByteString;
use crate::types::bytes::Bytes;
use crate::types::contract::NeoCandidate;
use crate::types::notification::Notification;
use crate::types::placeholder::Placeholder;
use crate::types::context::StorageContext;

/// Get the executing script hash
pub unsafe fn system_runtime_executing() -> H160 {
    H160::zero()
}

/// Get the calling script hash
pub unsafe fn system_runtime_calling() -> H160 {
    H160::zero()
}

/// Get the entry script hash
pub unsafe fn system_runtime_entry_script_hash() -> H160 {
    H160::zero()
}

/// Check if the witness is valid
pub unsafe fn system_runtime_check_witness(_hash: H160) -> bool {
    false
}

/// Get the platform
pub unsafe fn system_runtime_platform() -> ByteString {
    ByteString::from("NEO")
}

/// Get the gas left
pub unsafe fn system_runtime_gas_left() -> i64 {
    0
}

/// Get the invocation counter
pub unsafe fn system_runtime_invocation_counter() -> i32 {
    0
}

/// Get the time
pub unsafe fn system_runtime_time() -> u64 {
    0
}

/// Log a message
pub unsafe fn system_runtime_log(_message: ByteString) {
    // In a real implementation, this would log the message
}

/// Get the current transaction
pub unsafe fn system_runtime_tx() -> Placeholder {
    Placeholder::new()
}

/// Call a contract
pub unsafe fn system_contract_call(
    _hash: H160,
    _method: ByteString,
    _call_flags: CallFlags,
    _args: Array<ByteString>,
) -> Placeholder {
    Placeholder::new()
}

/// Create a contract
pub unsafe fn system_contract_create(
    _nef: ByteString,
    _manifest: ByteString,
    _call_flags: CallFlags,
) -> Placeholder {
    Placeholder::new()
}

/// Get the call flags
pub unsafe fn system_contract_get_call_flags() -> CallFlags {
    CallFlags::ALL
}

/// Get the Neo candidates
pub unsafe fn system_neo_get_candidates() -> Array<NeoCandidate> {
    Array::new()
}

/// Get the Neo committee
pub unsafe fn system_neo_get_committee() -> Array<H160> {
    Array::new()
}

/// Get the Neo next block validators
pub unsafe fn system_neo_get_next_block_validators() -> Array<H160> {
    Array::new()
}

/// Get the storage context
pub unsafe fn system_storage_get_context() -> StorageContext {
    StorageContext::new()
}

/// Get a read-only storage context
pub unsafe fn system_storage_get_read_only_context() -> StorageContext {
    StorageContext::new()
}

/// Convert a storage context to a read-only storage context
pub unsafe fn system_storage_as_readonly(context: StorageContext) -> StorageContext {
    context
}

/// Put a value in storage
pub unsafe fn system_storage_put(context: StorageContext, key: ByteString, value: ByteString) {
    // In a real implementation, this would call the storage put syscall
}

/// Get a value from storage
pub unsafe fn system_storage_get(context: StorageContext, key: ByteString) -> ByteString {
    // In a real implementation, this would call the storage get syscall
    ByteString::empty()
}

/// Delete a value from storage
pub unsafe fn system_storage_delete(context: StorageContext, key: ByteString) {
    // In a real implementation, this would call the storage delete syscall
}

/// Find values in storage
pub unsafe fn system_storage_find(context: StorageContext, prefix: ByteString) -> i32 {
    // In a real implementation, this would call the storage find syscall
    0
}

/// Check a signature
pub unsafe fn system_crypto_check_sign(public_key: ByteString, sign: ByteString) -> bool {
    // In a real implementation, this would call the crypto check sign syscall
    false
}

/// Check multiple signatures
pub unsafe fn system_crypto_check_multi_signs(public_keys: Array<ByteString>, signs: Array<ByteString>) -> bool {
    // In a real implementation, this would call the crypto check multi signs syscall
    false
}

/// Get the notifications
pub unsafe fn system_runtime_notifications() -> Array<Notification> {
    // In a real implementation, this would call the runtime notifications syscall
    Array::new()
}

/// Get the network
pub unsafe fn system_runtime_get_network() -> u32 {
    // In a real implementation, this would call the runtime get network syscall
    0
}

/// Get a random number
pub unsafe fn system_runtime_get_random() -> Int256 {
    // In a real implementation, this would call the runtime get random syscall
    Int256::zero()
}

/// Burn gas
pub unsafe fn system_runtime_burn_gas(_amount: Int256) {
    // In a real implementation, this would call the runtime burn gas syscall
}

/// Get the address version
pub unsafe fn system_runtime_address_version() -> u32 {
    // In a real implementation, this would call the runtime address version syscall
    0
}
