// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Re-exports of macros from neo-macros and neo-contract
//! This provides a single place to import all necessary macros for contract development

// Re-export all the procedural macros from neo-macros
pub use neo_macros::{
    byte_array, constructor, contract, contract_author, contract_description, contract_hash, contract_hash_fixed,
    contract_permission, contract_trust, contract_version, event, hash160, hash160_fixed, hash256, hash256_fixed,
    index, integer, integer_fixed, manifest_extra, method, no_reentrant, public_key, public_key_fixed, register_event,
    safe, signature, signature_fixed, storage, string, string_fixed, supported_standards, vm_opcode,
};

// Define the neo_contract_module struct
#[doc(hidden)]
pub struct neo_contract_module {
    _private: (),
}

// Re-export the entry point functions needed by contract macro
pub use crate::runtime::{__neo_deploy_entry, __neo_invoke_entry};

// Fully implement the inventory module
pub mod inventory {
    // This struct is used by the attribute macros to register components
    pub struct Submit<T> {
        _item: core::marker::PhantomData<T>,
    }

    impl<T> Submit<T> {
        pub fn new(_item: T) -> Self { Self { _item: core::marker::PhantomData } }
    }

    // Alias for backwards compatibility
    pub type submit<T> = Submit<T>;
}

// Dummy implementation for neo_contract_proc_macros to satisfy dependencies
pub mod neo_contract_proc_macros {
    use crate::types::builtin::any::Any;

    // Main entry points
    pub fn deploying() -> bool { true }

    pub fn invoke(_operation: &str, _args: &[Any]) -> Any { Any::null() }

    // Helper function to register events
    pub fn register_event(_name: &str, _indexed_params: &[&str]) {}

    // Helper function to register methods
    pub fn register_method(_name: &str, _params: &[&str], _return_type: &str, _is_safe: bool) {}

    // Helper function to check witness
    pub fn check_witness(_hash: &[u8]) -> bool { false }
}
