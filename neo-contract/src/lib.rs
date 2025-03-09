//! Neo Contract Rust Framework
//!
//! This crate provides a framework for developing Neo N3 smart contracts in Rust.
//! It is inspired by the ink! smart contract framework for Substrate.

#![no_std]
#![cfg_attr(feature = "std", feature(alloc_error_handler))]
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
pub mod events;
pub mod find_options;
pub mod role;
pub mod static_values;
pub mod transaction_attribute_type;

// Core functionality
pub mod types;
pub mod storage;
pub mod env;
pub mod num256;
pub mod policy;
pub mod utils;
pub mod macros;
pub mod runtime;

// Token standards and implementations
pub mod token;

// Type re-exports for convenience
pub use types::builtin::h160::H160;
pub use types::builtin::h256::H256;
pub use types::builtin::string::ByteString;
pub use types::builtin::int256::Int256;
pub use types::builtin::array::Array;
pub use types::builtin::any::Any;
pub use types::builtin::map::Map;

// Module re-exports for convenience - env module already has these
// pub use env::{storage, runtime as env_runtime, blockchain, contract};
// Re-export Runtime struct for backward compatibility
pub use self::runtime::Runtime;

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
    pub use crate::env::{storage, runtime, blockchain, contract};
    // Import crypto and system directly from crate
    pub use crate::crypto;
    
    pub use crate::types::builtin::h160::H160;
    pub use crate::types::builtin::h256::H256;
    pub use crate::types::builtin::string::ByteString;
    pub use crate::types::builtin::int256::Int256;
    pub use crate::types::builtin::array::Array;
    pub use crate::types::builtin::any::Any;
    pub use crate::types::builtin::map::Map;
    
    pub use crate::call_flags::CallFlags;
    pub use crate::error::{Error, ErrorCode, Result};
    pub use crate::events;
    pub use crate::find_options::FindOptions;
    pub use crate::role::Role;
    
    // Storage modules are now properly implemented
    pub use crate::storage::{
        Context as StorageContext,  // Import Context directly from storage module
        item::Item,
        map::Map as StorageMap,
        versioned::VersionedItem,
        iter::{StorageIter, StorageIterator},
        pagination::{Page, Paginator},
    };
    
    pub use crate::macros::*;
    
    // Attribute macros
    pub use neo_contract_proc_macros::{
        contract,
        contract_permission,
        contract_trust,
        manifest_extra,
        supported_standards,
        no_reentrant,
        safe,
    };
    
    // Commonly used alloc types
    pub use alloc::{string::String, vec::Vec, boxed::Box};
}

// This section is needed for the proc macros
pub use neo_contract_proc_macros::{
    contract,
    contract_permission,
    contract_trust,
    manifest_extra,
    supported_standards,
    no_reentrant,
    safe,
};

// Add a re-export for neo_method to make it available for the safe attribute
#[doc(hidden)]
pub use neo_contract_proc_macros::method as neo_method;

// Re-export for serialization
pub mod serialize;

// Export test utilities (only in test builds)
#[cfg(test)]
pub mod test_utils;
