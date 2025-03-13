//! Neo Contract Rust Framework
//!
//! This crate provides a framework for developing Neo N3 smart contracts in Rust.
//! It is inspired by the ink! smart contract framework for Substrate.

#![no_std]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![recursion_limit = "1024"]

extern crate alloc;

// Re-export core crates
pub use alloc::{boxed, collections, fmt, string, vec};

// Modules
pub mod attributes;
pub mod call_flags;
pub mod codec;
pub mod error;
pub mod event; // New Neo N3 standard event utilities
pub mod events; // Old events module - kept for backward compatibility
pub mod find_options;
pub mod manifest;
pub mod role;
pub mod static_values;
pub mod transaction_attribute_type;

// Core functionality
pub mod env;
pub mod macros;
pub mod num256;
pub mod policy;
pub mod runtime;
pub mod storage;
pub mod types;
pub mod utils;

// Token standards and implementations
pub mod token;

// Type re-exports for convenience
pub use types::builtin::any::Any;
pub use types::builtin::array::Array;
pub use types::builtin::h160::H160;
pub use types::builtin::h256::H256;
pub use types::builtin::int256::Int256;
pub use types::builtin::map::Map;
pub use types::builtin::string::ByteString;

// Re-export all Neo N3 attribute macros
pub use neo_macros::{
    constructor,
    // Contract macros
    contract,
    contract_permission,
    contract_trust,
    // Basic contract structure macros
    event,
    index,
    // Manifest related macros
    manifest_extra,
    method,
    no_reentrant,
    safe,
    storage,
    supported_standards,
};

// Module re-exports for convenience - env module already has these
// pub use env::{storage, runtime as env_runtime, blockchain, contract};
// Re-export Runtime struct for backward compatibility
pub use self::runtime::Runtime;

// Re-export manifest functions for proc-macros
pub use self::manifest::{register_contract, register_event, register_method, register_supported_standard};

// Re-export event helpers for easier Neo N3 standard event emission
pub use self::event::{
    emit_event2, emit_event3, emit_event_array, emit_transfer, null_or_value, register_transfer_event, EventBuilder,
    EventEmitter, StandardEventEmitter,
};

// Contract module
pub mod contract;

// Security module
pub mod security;

// Crypto module
pub mod crypto;

// Profiling module
pub mod profiling;

// Prelude module
pub mod prelude {
    //! The prelude module exports all the most commonly used types and functions.
    //! This allows users to import everything they need with a single import.

    // Import env modules
    pub use crate::env::{blockchain, contract, runtime, storage};
    // Import crypto and system directly from crate
    pub use crate::crypto;

    pub use crate::types::builtin::any::Any;
    pub use crate::types::builtin::array::Array;
    pub use crate::types::builtin::h160::H160;
    pub use crate::types::builtin::h256::H256;
    pub use crate::types::builtin::int256::Int256;
    pub use crate::types::builtin::map::Map;
    pub use crate::types::builtin::string::ByteString;

    // Manifest module for Neo N3 registration
    pub use crate::manifest;

    pub use crate::call_flags::CallFlags;
    pub use crate::error::{Error, ErrorCode, Result};
    pub use crate::events;
    pub use crate::find_options::FindOptions;
    pub use crate::role::Role;

    // Storage modules are now properly implemented
    pub use crate::storage::{
        item::Item,
        iter::{StorageIter, StorageIterator},
        map::Map as StorageMap,
        pagination::{Page, Paginator},
        versioned::VersionedItem,
        Context as StorageContext, // Import Context directly from storage module
    };

    // macros are imported directly where needed

    // Attribute macros
    pub use neo_macros::{
        constructor, contract, contract_permission, contract_trust, manifest_extra, method, no_reentrant, safe,
        storage, supported_standards,
    };

    // Commonly used alloc types
    pub use alloc::{boxed::Box, string::String, vec::Vec};
}

// Note: macros are already imported above, so we don't need to import them again

// Add a re-export for neo_method to make it available for the safe attribute
#[doc(hidden)]
pub use neo_macros::method as neo_method;

// Re-export for serialization
pub mod serialize;

// Export test utilities (only in test builds)
#[cfg(test)]
pub mod test_utils;
