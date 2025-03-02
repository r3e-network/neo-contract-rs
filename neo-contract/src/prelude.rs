// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Prelude module that re-exports commonly used items
//! Import this module with `use neo_contract::prelude::*;` to get access to
//! most commonly used components without having to import them individually.

// Core types
pub use crate::builtin::{H160, H256, Int256, ByteString, Array, Map, Any};

// Runtime
pub use crate::Runtime;

// Contract attributes
pub use crate::{
    contract, storage, constructor, message, event,
    contract_author, contract_description, contract_version, 
    supported_standards, safe, no_reentrant, no_reentrant_method
};

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
pub use crate::storage::{StorageMap, StorageMapEntry};
