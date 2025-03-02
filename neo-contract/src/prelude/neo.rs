// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

/// Re-export proc macros with the neo namespace 
/// and add ink!-style attributes for consistency
pub use neo_contract_proc_macros::{
    // Core contract attributes
    contract, stored as storage, constructor, message, event,
    
    // Metadata attributes
    contract_permission, contract_trust, supported_standards, manifest_extra,
    
    // Field initialization attributes
    byte_array, hash160, integer, public_key, string, contract_hash,
    
    // Security attributes
    safe, no_reentrant, no_reentrant_method,
    
    // Function modifier attributes
    modifier, calling_convention, op_code, syscall
};
