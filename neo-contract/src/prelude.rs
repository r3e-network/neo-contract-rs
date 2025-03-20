// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Prelude module that re-exports commonly used items
//! Import this module with `use neo_contract::prelude::*;` to get access to
//! most commonly used components without having to import them individually.

// Core types
pub use crate::types::builtin::{h160::H160, h256::H256, int256::Int256, string::ByteString, array::Array, map::Map, any::Any};

// Runtime
pub use crate::runtime::Runtime;

// Event system
pub use crate::event::{register_event, EventBuilder, emit_transfer, emit_event, emit_event2, emit_event3};

// Re-export all macros from exports module
pub use crate::exports::*;

// Contract attributes
pub use crate::{
    contract, storage, constructor, method, event,
    contract_author, contract_description, contract_version, 
    supported_standards, safe, no_reentrant, manifest_extra,
    contract_permission, contract_trust, index
};

// Internal macro components used by other macros
#[doc(hidden)]
pub use crate::neo_method as manifest_method;

// Neo contract module marker
#[doc(hidden)]
pub struct neo_contract_module {
    _private: ()
}

// Create a global static instance for macros to find
#[doc(hidden)]
#[allow(non_upper_case_globals)]
pub static neo_contract_module: neo_contract_module = neo_contract_module { _private: () };

// Re-export runtime entry point functions needed by contract macro
#[doc(hidden)]
pub use crate::runtime::{__neo_deploy_entry, __neo_invoke_entry};

// Native contracts
pub use crate::contract::native::{gas, legder, neo, oracle, policy};

// Storage
pub use crate::storage::{get_context, get_readonly_context, as_readonly};

// NEP standards
pub use crate::contract::{nep11, nep17};

// Crypto
pub use crate::crypto::{check_sign, check_multi_signs, NamedCurveHash};

// CallFlags for contract calls
pub use crate::CallFlags;

// Common system crates
pub use alloc::{vec, string::ToString, format};
pub use alloc::vec::Vec;
pub use alloc::string::String;
pub use core::panic;

// Re-export commonly used storage helpers
pub use crate::storage::{StorageContext, StorageMap, StorageKey};

// Neo N3 Event utilities
pub use crate::event::{EventEmitter, StandardEventEmitter, null_or_value};

// Neo N3 specific utilities
pub use crate::manifest::{ContractManifest, ContractABI, ContractMethod, ContractEvent};
pub use crate::storage::{StorageMap, StorageMapEntry};
