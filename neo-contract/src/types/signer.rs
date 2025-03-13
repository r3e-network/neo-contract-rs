// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::types::builtin::h160::H160;
use alloc::vec::Vec;

/// Signer represents a signer of a transaction
#[derive(Debug, Clone)]
pub struct Signer {
    account: H160,
    scopes: u8,
    allowed_contracts: Vec<H160>,
    allowed_groups: Vec<H160>,
}

/// Witness scope
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WitnessScope {
    /// No scope
    None = 0,
    /// Called by entry
    CalledByEntry = 1,
    /// Custom contracts
    CustomContracts = 16,
    /// Custom groups
    CustomGroups = 32,
    /// Global
    Global = 128,
}

impl Signer {
    /// Create a new signer
    pub fn new(account: H160, scopes: u8) -> Self {
        Self {
            account,
            scopes,
            allowed_contracts: Vec::new(),
            allowed_groups: Vec::new(),
        }
    }

    /// Create a new signer with allowed contracts
    pub fn new_with_contracts(account: H160, scopes: u8, allowed_contracts: Vec<H160>) -> Self {
        Self {
            account,
            scopes,
            allowed_contracts,
            allowed_groups: Vec::new(),
        }
    }

    /// Create a new signer with allowed groups
    pub fn new_with_groups(account: H160, scopes: u8, allowed_groups: Vec<H160>) -> Self {
        Self {
            account,
            scopes,
            allowed_contracts: Vec::new(),
            allowed_groups,
        }
    }

    /// Get the account of the signer
    pub fn account(&self) -> H160 { self.account.clone() }

    /// Get the scopes of the signer
    pub fn scopes(&self) -> u8 { self.scopes }

    /// Get the allowed contracts of the signer
    pub fn allowed_contracts(&self) -> Vec<H160> { self.allowed_contracts.clone() }

    /// Get the allowed groups of the signer
    pub fn allowed_groups(&self) -> Vec<H160> { self.allowed_groups.clone() }
}
