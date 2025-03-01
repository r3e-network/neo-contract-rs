// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::vec::Vec;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use crate::types::context::StorageContext;

pub mod context;
pub mod iter;
pub mod map;

/// Get the storage context
pub fn get_context() -> StorageContext {
    unsafe { crate::env::syscall_non_wasm::system_storage_get_context() }
}

/// Get a read-only storage context
pub fn get_readonly_context() -> StorageContext {
    unsafe { crate::env::syscall_non_wasm::system_storage_get_read_only_context() }
}

/// Convert a storage context to a read-only storage context
pub fn as_readonly(context: StorageContext) -> StorageContext {
    unsafe { crate::env::syscall_non_wasm::system_storage_as_readonly(context) }
}

/// Put a value in storage
pub fn put(context: StorageContext, key: ByteString, value: ByteString) {
    unsafe { crate::env::syscall_non_wasm::system_storage_put(context, key, value) }
}

/// Get a value from storage
pub fn get(context: StorageContext, key: ByteString) -> ByteString {
    unsafe { crate::env::syscall_non_wasm::system_storage_get(context, key) }
}

/// Delete a value from storage
pub fn delete(context: StorageContext, key: ByteString) {
    unsafe { crate::env::syscall_non_wasm::system_storage_delete(context, key) }
}

/// Find values in storage
pub fn find(context: StorageContext, prefix: ByteString) -> i32 {
    unsafe { crate::env::syscall_non_wasm::system_storage_find(context, prefix) }
}
