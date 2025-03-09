//! Roles for Neo Contract RS
//!
//! This module defines the roles used in the Neo governance system.

use core::fmt;
use alloc::string::String;

/// Roles in the Neo governance system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Role {
    /// State validator - validates the state of the blockchain
    StateValidator = 0x04,
    
    /// Oracle - provides external data to smart contracts
    Oracle = 0x08,
    
    /// NeoFS Alphabet node - NeoFS storage nodes with special permissions
    NeoFSAlphabetNode = 0x10,
    
    /// Policy committee - committee that decides on policy changes
    PolicyCommittee = 0x20,
    
    /// Contract deployer - allowed to deploy contracts
    ContractDeployer = 0x40,
}

impl Role {
    /// Returns the byte value of the role
    pub fn value(&self) -> u8 {
        *self as u8
    }
    
    /// Tries to convert a byte value to a Role
    pub fn from_value(value: u8) -> Option<Self> {
        match value {
            0x04 => Some(Role::StateValidator),
            0x08 => Some(Role::Oracle),
            0x10 => Some(Role::NeoFSAlphabetNode),
            0x20 => Some(Role::PolicyCommittee),
            0x40 => Some(Role::ContractDeployer),
            _ => None,
        }
    }
    
    /// Returns a human-readable name for the role
    pub fn name(&self) -> &'static str {
        match self {
            Role::StateValidator => "State Validator",
            Role::Oracle => "Oracle",
            Role::NeoFSAlphabetNode => "NeoFS Alphabet Node",
            Role::PolicyCommittee => "Policy Committee",
            Role::ContractDeployer => "Contract Deployer",
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Converts a Role to a u8
impl From<Role> for u8 {
    fn from(role: Role) -> Self {
        role.value()
    }
}

/// Tries to convert a u8 to a Role
impl TryFrom<u8> for Role {
    type Error = ();
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Role::from_value(value).ok_or(())
    }
}
