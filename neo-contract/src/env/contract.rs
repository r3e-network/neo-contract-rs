// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use crate::call_flags::CallFlags;
use crate::types::builtin::array::Array;
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
pub fn calling_script_hash() -> H160 { unsafe { crate::env::syscall_non_wasm::system_runtime_calling_script_hash() } }

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_calling_script_hash() -> H160 {
    // In the wasm environment, this would call the native calling_script_hash function
    H160::zero()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn entry_script_hash() -> H160 { unsafe { crate::env::syscall_non_wasm::system_runtime_entry_script_hash() } }

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
pub fn check_witness(hash: H160) -> bool { unsafe { crate::env::syscall_non_wasm::system_runtime_check_witness(hash) } }

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_platform() -> ByteString {
    // In the wasm environment, this would call the native platform function
    ByteString::default()
}

// For non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn platform() -> ByteString { unsafe { crate::env::syscall_non_wasm::system_runtime_platform() } }

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_gas_left() -> i64 {
    // In the wasm environment, this would call the native gas_left function
    0
}

// For non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn gas_left() -> i64 { unsafe { crate::env::syscall_non_wasm::system_runtime_gas_left() } }

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_invocation_counter() -> i32 {
    // In the wasm environment, this would call the native invocation_counter function
    0
}

// For non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn invocation_counter() -> i32 { unsafe { crate::env::syscall_non_wasm::system_runtime_invocation_counter() } }

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_time() -> u64 {
    // In the wasm environment, this would call the native time function
    0
}

// For non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn time() -> u64 { unsafe { crate::env::syscall_non_wasm::system_runtime_time() } }

// For wasm32 target
#[cfg(target_arch = "wasm32")]
pub fn native_log(message: &str) {
    // In the wasm environment, this would call the native log function
}

// For non-wasm targets
#[cfg(not(target_arch = "wasm32"))]
pub fn log(message: &str) { unsafe { crate::env::syscall_non_wasm::system_runtime_log(ByteString::from(message)) } }

// Native contract hash functions for wasm32 target

// ContractManagement native contract
#[cfg(target_arch = "wasm32")]
pub fn native_contract_management_contract_hash() -> H160 {
    // In the wasm environment, this would call the native contract_management_contract_hash function
    H160::zero()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn contract_management_contract_hash() -> H160 {
    // This would be implemented to return the actual contract hash in non-wasm environments
    H160::from_hex("fffdc93764dbaddd97c48f252a53ea4643faa3fd").unwrap()
}

// CryptoLib native contract
#[cfg(target_arch = "wasm32")]
pub fn native_crypto_lib_contract_hash() -> H160 {
    // In the wasm environment, this would call the native crypto_lib_contract_hash function
    H160::zero()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn crypto_lib_contract_hash() -> H160 {
    // This would be implemented to return the actual contract hash in non-wasm environments
    H160::from_hex("726cb6e0cd8628a1350a611384688911ab75f51b").unwrap()
}

// GAS native contract
#[cfg(target_arch = "wasm32")]
pub fn native_gas_contract_hash() -> H160 {
    // In the wasm environment, this would call the native gas_contract_hash function
    H160::zero()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn gas_contract_hash() -> H160 {
    // This would be implemented to return the actual contract hash in non-wasm environments
    H160::from_hex("d2a4cff31913016155e38e474a2c06d08be276cf").unwrap()
}

// Ledger native contract
#[cfg(target_arch = "wasm32")]
pub fn native_ledger_contract_hash() -> H160 {
    // In the wasm environment, this would call the native ledger_contract_hash function
    H160::zero()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn ledger_contract_hash() -> H160 {
    // This would be implemented to return the actual contract hash in non-wasm environments
    H160::from_hex("da65b600f7124ce6c79950c1772a36403104f2be").unwrap()
}

// NEO native contract
#[cfg(target_arch = "wasm32")]
pub fn native_neo_contract_hash() -> H160 {
    // In the wasm environment, this would call the native neo_contract_hash function
    H160::zero()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn neo_contract_hash() -> H160 {
    // This would be implemented to return the actual contract hash in non-wasm environments
    H160::from_hex("ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5").unwrap()
}

// Oracle native contract
#[cfg(target_arch = "wasm32")]
pub fn native_oracle_contract_hash() -> H160 {
    // In the wasm environment, this would call the native oracle_contract_hash function
    H160::zero()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn oracle_contract_hash() -> H160 {
    // This would be implemented to return the actual contract hash in non-wasm environments
    H160::from_hex("fe924b7cfe89ddd271abaf7210a80a7e11178758").unwrap()
}

// Policy native contract
#[cfg(target_arch = "wasm32")]
pub fn native_policy_contract_hash() -> H160 {
    // In the wasm environment, this would call the native policy_contract_hash function
    H160::zero()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn policy_contract_hash() -> H160 {
    // This would be implemented to return the actual contract hash in non-wasm environments
    H160::from_hex("cc5e4edd9f5f8dba8bb65734541df7a1c081c67b").unwrap()
}

// RoleManagement native contract
#[cfg(target_arch = "wasm32")]
pub fn native_role_management_contract_hash() -> H160 {
    // In the wasm environment, this would call the native role_management_contract_hash function
    H160::zero()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn role_management_contract_hash() -> H160 {
    // This would be implemented to return the actual contract hash in non-wasm environments
    H160::from_hex("49cf4e5378ffcd4dec034fd98a174c5491e395e2").unwrap()
}

// StdLib native contract
#[cfg(target_arch = "wasm32")]
pub fn native_std_lib_contract_hash() -> H160 {
    // In the wasm environment, this would call the native std_lib_contract_hash function
    H160::zero()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn std_lib_contract_hash() -> H160 {
    // This would be implemented to return the actual contract hash in non-wasm environments
    H160::from_hex("acce6fd80d44e1796aa0c2c625e9e4e0ce39efc0").unwrap()
}
