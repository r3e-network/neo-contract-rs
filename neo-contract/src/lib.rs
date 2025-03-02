// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![allow(unused_variables)]

extern crate alloc;

pub mod attributes;
pub mod call_flags;
pub mod contract;
pub mod crypto;
pub mod env;
pub mod events;
pub mod error;
pub mod find_options;
pub mod macros;
pub mod runtime;
pub mod security;
pub mod policy;
pub mod profiling;
pub mod serialize;
pub mod static_values;
pub mod storage;
pub mod token;
pub mod types;
pub mod utils;

// Re-exports
pub use call_flags::CallFlags;
pub use contract::{nep11, nep17, native};
pub use crypto::*;
pub use env::{contract as env_contract, contract_non_wasm, syscall, syscall_non_wasm};
pub use find_options::FindOptions;
// Re-export macros
pub use macros::*;
pub use runtime::Runtime;
pub use static_values::*;
pub use storage::*;
pub use types::{block, builtin as types_builtin, bytes, context, key, notification, signer, storage as types_storage, tx};
pub use utils::*;
pub use events::*;
pub use error::*;
pub use security::*;

// Re-export the reentrancy guard macro
pub use reentrancy_guard;
pub use no_reentrant_method;
pub use ensure;
pub use require_witness;
pub use define_event;

// Re-export proc macros
pub use neo_contract_proc_macros::{
    export_trait, smart_contract, 
    // Core ink!-style attributes
    contract, storage, constructor, message, event,
    // Metadata attributes
    contract_author, contract_email, contract_description, contract_version,
    manifest_extra, contract_permission, contract_trust, supported_standards,
    // Static field attributes
    byte_array, hash160, integer, public_key, string, contract_hash,
    // Security attributes
    safe, no_reentrant, no_reentrant_method, stored,
    // Function attributes
    modifier, calling_convention, op_code, syscall
};

// Re-export builtin types for easier access
pub mod builtin {
    pub use crate::types::builtin::h160::H160;
    pub use crate::types::builtin::h256::H256;
    pub use crate::types::builtin::int256::Int256;
    pub use crate::types::builtin::string::ByteString;
    pub use crate::types::builtin::array::Array;
    pub use crate::types::builtin::any::Any;
    pub use crate::types::builtin::map::Map;
}

// Add prelude module
pub mod prelude;
