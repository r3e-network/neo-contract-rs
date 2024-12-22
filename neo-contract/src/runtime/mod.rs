// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub mod assert;

pub use assert::*;

use crate::{env, types::*};

#[inline(always)]
pub fn get_trigger() -> TriggerType {
    unsafe { env::syscall::system_runtime_trigger() }
}

#[inline(always)]
pub fn get_platform() -> ByteString {
    unsafe { env::syscall::system_runtime_platform() }
}

#[inline(always)]
pub fn get_tx() -> Tx {
    unsafe { env::syscall::system_runtime_tx() }
}

#[inline(always)]
pub fn get_executing_script_hash() -> H160 {
    unsafe { env::syscall::system_runtime_executing_script_hash() }
}

#[inline(always)]
pub fn get_calling_script_hash() -> H160 {
    unsafe { env::syscall::system_runtime_calling_script_hash() }
}

#[inline(always)]
pub fn get_entry_script_hash() -> H160 {
    unsafe { env::syscall::system_runtime_entry_script_hash() }
}

#[inline(always)]
pub fn get_time() -> u64 {
    unsafe { env::syscall::system_runtime_time() }
}

#[inline(always)]
pub fn get_invocation_counter() -> u32 {
    unsafe { env::syscall::system_runtime_invocation_counter() }
}

#[inline(always)]
pub fn get_gas_left() -> Int256 {
    unsafe { env::syscall::system_runtime_gas_left() }
}

#[inline(always)]
pub fn get_address_version() -> u32 {
    unsafe { env::syscall::system_runtime_address_version() }
}

#[inline(always)]
pub fn get_notifications() -> Array<Notification> {
    unsafe { env::syscall::system_runtime_notifications() }
}

#[inline(always)]
pub fn check_witness_with_account(account: H160) -> bool {
    unsafe { env::syscall::system_runtime_check_witness_with_account(account) }
}

#[inline(always)]
pub fn check_witness_with_public_key(public_key: PublicKey) -> bool {
    unsafe { env::syscall::system_runtime_check_witness_with_public_key(public_key) }
}

#[inline(always)]
pub fn log(message: ByteString) {
    unsafe { env::syscall::system_runtime_log(message) }
}

#[inline(always)]
pub fn burn_gas(amount: Int256) {
    unsafe { env::syscall::system_runtime_burn_gas(amount) }
}

#[inline(always)]
pub fn get_random() -> Int256 {
    unsafe { env::syscall::system_runtime_get_random() }
}

#[inline(always)]
pub fn get_network() -> u32 {
    unsafe { env::syscall::system_runtime_get_network() }
}

#[inline(always)]
pub fn load_script(script_hash: H160, call_flags: CallFlags, args: Array<Any>) -> Any {
    unsafe { env::syscall::system_runtime_load_script(script_hash, call_flags, args) }
}

#[inline(always)]
pub fn current_signers() -> Array<Signer> {
    unsafe { env::syscall::system_runtime_current_signers() }
}
