// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::types::builtin::h160::H160;

/// Signer represents a transaction signer
#[derive(Debug, Clone)]
pub struct Signer {
    account: H160,
    scopes: WitnessScope,
    allowed_contracts: Vec<H160>,
    allowed_groups: Vec<H160>,
}

/// WitnessScope represents the scope of a witness
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitnessScope {
    /// None
    None = 0,
    /// CalledByEntry
    CalledByEntry = 1,
    /// CustomContracts
    CustomContracts = 16,
    /// CustomGroups
    CustomGroups = 32,
    /// Global
    Global = 128,
}

/// WitnessRule represents a witness rule
#[derive(Debug, Clone)]
pub struct WitnessRule {
    action: WitnessRuleAction,
    condition: WitnessCondition,
}

/// WitnessRuleAction represents a witness rule action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitnessRuleAction {
    /// Deny
    Deny = 0,
    /// Allow
    Allow = 1,
}

/// WitnessCondition represents a witness condition
#[derive(Debug, Clone)]
pub struct WitnessCondition {
    condition_type: WitnessConditionType,
    expression: Vec<u8>,
}

/// WitnessConditionType represents a witness condition type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitnessConditionType {
    /// Boolean
    Boolean = 0,
    /// Not
    Not = 1,
    /// And
    And = 2,
    /// Or
    Or = 3,
    /// ScriptHash
    ScriptHash = 4,
    /// Group
    Group = 5,
    /// CalledByEntry
    CalledByEntry = 6,
    /// CalledByContract
    CalledByContract = 7,
    /// CalledByGroup
    CalledByGroup = 8,
}

impl Signer {
    /// Create a new signer
    pub fn new() -> Self {
        Self {
            account: H160::zero(),
            scopes: WitnessScope::None,
            allowed_contracts: Vec::new(),
            allowed_groups: Vec::new(),
        }
    }

    /// Get the account
    pub fn account(&self) -> H160 {
        self.account.clone()
    }

    /// Get the scopes
    pub fn scopes(&self) -> WitnessScope {
        self.scopes.clone()
    }

    /// Get the allowed contracts
    pub fn allowed_contracts(&self) -> Vec<H160> {
        self.allowed_contracts.clone()
    }

    /// Get the allowed groups
    pub fn allowed_groups(&self) -> Vec<H160> {
        self.allowed_groups.clone()
    }
}
