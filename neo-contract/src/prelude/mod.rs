// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

// Re-export the main types and modules
pub use crate::builtin::*;
pub use crate::runtime::*;
pub use crate::storage::*;
pub use crate::env::syscall;
pub use crate::env::syscall_non_wasm;

// Re-export neo namespace
pub mod neo;
pub use neo::*;

// Re-export native contracts for easy access
#[cfg(feature = "std")]
pub use crate::contracts::native::*;

// Add a dedicated ink! style module for a cleaner import experience
pub mod ink_style {
    // Re-export all ink! style attributes for easy imports
    pub use neo_contract_proc_macros::{
        // Core contract attributes
        contract, storage, constructor, message, event,
        
        // Metadata attributes
        contract_author, contract_email, contract_description, contract_version,
        contract_permission, contract_trust, supported_standards, manifest_extra,
        
        // Field initialization attributes
        byte_array, hash160, integer, public_key, string, contract_hash,
        
        // Security attributes
        safe, no_reentrant, no_reentrant_method,
        
        // Function modifier attributes
        modifier, calling_convention, op_code, syscall
    };
    
    // Re-export common types used in ink! style contracts
    pub use crate::builtin::{H160, H256, Int256, ByteString, Map, Array, Any};
}
