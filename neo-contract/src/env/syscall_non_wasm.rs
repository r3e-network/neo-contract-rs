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
    // In a real implementation, this would call the runtime executing syscall
    H160::zero()
}

/// Get the calling script hash
pub unsafe fn system_runtime_calling() -> H160 {
    // In a real implementation, this would call the runtime calling syscall
    H160::zero()
}

/// Get the entry script hash
pub unsafe fn system_runtime_entry_script_hash() -> H160 {
    // In a real implementation, this would call the runtime entry script hash syscall
    H160::zero()
}

/// Check if the witness is valid
pub unsafe fn system_runtime_check_witness(_hash: H160) -> bool {
    // In a real implementation, this would call the runtime check witness syscall
    false
}

/// Get the platform
pub unsafe fn system_runtime_platform() -> ByteString {
    // In a real implementation, this would call the runtime platform syscall
    ByteString::from("NEO")
}

/// Get the gas left
pub unsafe fn system_runtime_gas_left() -> i64 {
    // In a real implementation, this would call the runtime gas left syscall
    0
}

/// Get the invocation counter
pub unsafe fn system_runtime_invocation_counter() -> i32 {
    // In a real implementation, this would call the runtime invocation counter syscall
    0
}

/// Get the time
pub unsafe fn system_runtime_time() -> u64 {
    // In a real implementation, this would call the runtime time syscall
    0
}

/// Call a contract
pub unsafe fn system_contract_call(
    _hash: H160,
    _method: ByteString,
    _call_flags: CallFlags,
    _args: Array<ByteString>,
) -> Placeholder {
    // In a real implementation, this would call the contract call syscall
    Placeholder::new()
}

/// Create a contract
pub unsafe fn system_contract_create(
    _nef: ByteString,
    _manifest: ByteString,
    _call_flags: CallFlags,
) -> Placeholder {
    // In a real implementation, this would call the contract create syscall
    Placeholder::new()
}

/// Get the call flags
pub unsafe fn system_contract_get_call_flags() -> CallFlags {
    CallFlags::All
}

/// Get the Neo candidates
pub unsafe fn system_neo_get_candidates() -> Array<NeoCandidate> {
    // In a real implementation, this would call the neo get candidates syscall
    Array::new()
}

/// Get the Neo committee
pub unsafe fn system_neo_get_committee() -> Array<H160> {
    // In a real implementation, this would call the neo get committee syscall
    Array::new()
}

/// Get the Neo next block validators
pub unsafe fn system_neo_get_next_block_validators() -> Array<H160> {
    // In a real implementation, this would call the neo get next block validators syscall
    Array::new()
}

/// Get the storage context
pub unsafe fn system_storage_get_context() -> StorageContext {
    // In a real implementation, this would call the storage get context syscall
    StorageContext::new()
}

/// Get a read-only storage context
pub unsafe fn system_storage_get_read_only_context() -> StorageContext {
    // In a real implementation, this would call the storage get read only context syscall
    StorageContext::new()
}

/// Convert a storage context to a read-only storage context
pub unsafe fn system_storage_as_readonly(context: StorageContext) -> StorageContext {
    // In a real implementation, this would call the storage as readonly syscall
    context
}

/// Put a value in storage
pub unsafe fn system_storage_put(context: StorageContext, key: ByteString, value: ByteString) {
    // In a real implementation, this would call the storage put syscall
}

/// Get a value from storage
pub unsafe fn system_storage_get(context: StorageContext, key: ByteString) -> ByteString {
    // In a real implementation, this would call the storage get syscall
    ByteString::new()
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
