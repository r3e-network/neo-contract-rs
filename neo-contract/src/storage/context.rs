// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

/// Storage context for contract storage operations
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StorageContext {
    pub(crate) contract_hash: H160,
    pub(crate) read_only: bool,
}

impl StorageContext {
    /// Creates a new storage context
    pub fn new(contract_hash: H160, read_only: bool) -> Self {
        Self {
            contract_hash,
            read_only,
        }
    }

    /// Creates a read-only storage context
    pub fn as_read_only(&self) -> Self {
        Self {
            contract_hash: self.contract_hash,
            read_only: true,
        }
    }

    /// Checks if the storage context is read-only
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// Gets the contract hash of the storage context
    pub fn get_contract_hash(&self) -> H160 {
        self.contract_hash
    }
}

/// Trait for storage context operations
pub trait StorageContextOps {
    /// Creates a storage context for the current contract
    fn current_context() -> StorageContext;
    
    /// Creates a storage context for a specific contract
    fn for_contract(contract_hash: H160) -> StorageContext;
}

#[cfg(target_family = "wasm")]
impl StorageContextOps for StorageContext {
    fn current_context() -> StorageContext {
        unsafe {
            let contract_hash = crate::env::contract::get_current_contract_hash();
            StorageContext::new(contract_hash, false)
        }
    }
    
    fn for_contract(contract_hash: H160) -> StorageContext {
        StorageContext::new(contract_hash, false)
    }
}

#[cfg(not(target_family = "wasm"))]
impl StorageContextOps for StorageContext {
    fn current_context() -> StorageContext {
        unsafe {
            let contract_hash = crate::env::contract_non_wasm::get_current_contract_hash();
            StorageContext::new(contract_hash, false)
        }
    }
    
    fn for_contract(contract_hash: H160) -> StorageContext {
        StorageContext::new(contract_hash, false)
    }
}
