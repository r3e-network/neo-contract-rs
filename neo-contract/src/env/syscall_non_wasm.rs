// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![allow(unused)]

use crate::{storage::*, types::*};

// Non-WASM implementations for testing
pub unsafe fn system_runtime_trigger() -> TriggerType {
    TriggerType::Application
}

pub unsafe fn system_runtime_platform() -> ByteString {
    "NEO".into()
}

pub unsafe fn system_runtime_tx() -> Tx {
    unimplemented!()
}

pub unsafe fn system_runtime_executing_script_hash() -> H160 {
    H160::default()
}

pub unsafe fn system_runtime_calling_script_hash() -> H160 {
    H160::default()
}

pub unsafe fn system_runtime_entry_script_hash() -> H160 {
    H160::default()
}

pub unsafe fn system_runtime_time() -> u64 {
    0
}

pub unsafe fn system_runtime_invocation_counter() -> u32 {
    0
}

pub unsafe fn system_runtime_gas_left() -> Int256 {
    Int256::zero()
}

pub unsafe fn system_runtime_address_version() -> u32 {
    0
}

pub unsafe fn system_runtime_notifications() -> Array<Notification> {
    Array::new()
}

pub unsafe fn system_runtime_check_witness_with_account(_account: H160) -> bool {
    true
}

pub unsafe fn system_runtime_check_witness_with_public_key(_public_key: PublicKey) -> bool {
    true
}

pub unsafe fn system_runtime_log(_message: ByteString) {
    // No-op for non-wasm
}

pub unsafe fn system_runtime_burn_gas(_amount: Int256) {
    // No-op for non-wasm
}

pub unsafe fn system_runtime_get_random() -> Int256 {
    Int256::zero()
}

pub unsafe fn system_runtime_get_network() -> u32 {
    0
}

pub unsafe fn system_runtime_load_script(
    _script_hash: H160,
    _call_flags: CallFlags,
    _args: Array<Any>,
) -> Any {
    Any::default()
}

pub unsafe fn system_runtime_current_signers() -> Array<Signer> {
    Array::new()
}

pub unsafe fn system_contract_call(
    _contract: H160,
    _method: ByteString,
    _call_flags: CallFlags,
    _args: Array<Any>,
) -> Any {
    Any::default()
}

pub unsafe fn system_contract_get_call_flags() -> CallFlags {
    CallFlags::All
}

pub unsafe fn system_contract_create_standard_account(_public_key: PublicKey) -> H160 {
    H160::default()
}

pub unsafe fn system_contract_create_multi_signs_account(
    _m: u32,
    _public_keys: Array<PublicKey>,
) -> H160 {
    H160::default()
}

pub unsafe fn system_crypto_check_sign(_public_key: PublicKey, _sign: ByteString) -> bool {
    true
}

pub unsafe fn system_crypto_check_multi_signs(
    _public_keys: Array<PublicKey>,
    _signs: Array<ByteString>,
) -> bool {
    true
}

pub unsafe fn system_iterator_next(_iterator: Placeholder) -> bool {
    false
}

pub unsafe fn system_iterator_value(_iterator: Placeholder) -> Placeholder {
    Placeholder::new(0)
}

pub unsafe fn system_storage_get_context() -> StorageContext {
    StorageContext::new()
}

pub unsafe fn system_storage_get_readonly_context() -> ReadOnlyStorageContext {
    ReadOnlyStorageContext::new()
}

pub unsafe fn system_storage_as_readonly(_cx: StorageContext) -> ReadOnlyStorageContext {
    ReadOnlyStorageContext::new()
}

pub unsafe fn system_storage_string_key_get(
    _context: StorageContext,
    _key: ByteString,
) -> Placeholder {
    Placeholder::new(0)
}

pub unsafe fn system_storage_bytes_key_get(_context: StorageContext, _key: Bytes) -> Placeholder {
    Placeholder::new(0)
}

pub unsafe fn system_storage_string_key_put(
    _context: StorageContext,
    _key: ByteString,
    _value: Placeholder,
) {
    // No-op for non-wasm
}

pub unsafe fn system_storage_bytes_key_put(
    _context: StorageContext,
    _key: Bytes,
    _value: Placeholder,
) {
    // No-op for non-wasm
}

pub unsafe fn system_storage_string_key_delete(_context: StorageContext, _key: ByteString) {
    // No-op for non-wasm
}

pub unsafe fn system_storage_bytes_key_delete(_context: StorageContext, _key: Bytes) {
    // No-op for non-wasm
}

pub unsafe fn system_storage_string_key_find(
    _context: StorageContext,
    _prefix: ByteString,
    _options: FindOptions,
) -> Placeholder {
    Placeholder::new(0)
}

pub unsafe fn system_storage_bytes_key_find(
    _context: StorageContext,
    _prefix: Bytes,
    _options: FindOptions,
) -> Placeholder {
    Placeholder::new(0)
}
