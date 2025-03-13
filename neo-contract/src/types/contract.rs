// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use alloc::string::String;
use alloc::vec::Vec;

/// Contract represents a smart contract
#[derive(Debug, Clone)]
pub struct Contract {
    /// Script hash
    pub script_hash: H160,
    /// Manifest
    pub manifest: ContractManifest,
}

/// Contract manifest
#[derive(Debug, Clone)]
pub struct ContractManifest {
    /// Name
    pub name: String,
    /// Groups
    pub groups: Vec<ContractGroup>,
    /// Features
    pub features: ContractFeatures,
    /// Supported standards
    pub supported_standards: Vec<String>,
    /// ABI
    pub abi: ContractABI,
    /// Permissions
    pub permissions: Vec<ContractPermission>,
    /// Trusts
    pub trusts: Vec<H160>,
    /// Extra
    pub extra: Option<String>,
}

/// Contract group
#[derive(Debug, Clone)]
pub struct ContractGroup {
    /// Public key
    pub public_key: ByteString,
    /// Signature
    pub signature: ByteString,
}

/// Contract features
#[derive(Debug, Clone)]
pub struct ContractFeatures {
    /// Storage
    pub storage: bool,
    /// Payable
    pub payable: bool,
}

/// Contract ABI
#[derive(Debug, Clone)]
pub struct ContractABI {
    /// Methods
    pub methods: Vec<ContractMethod>,
    /// Events
    pub events: Vec<ContractEvent>,
}

/// Contract method
#[derive(Debug, Clone)]
pub struct ContractMethod {
    /// Name
    pub name: String,
    /// Parameters
    pub parameters: Vec<ContractParameter>,
    /// Return type
    pub return_type: String,
    /// Offset
    pub offset: u32,
    /// Safe
    pub safe: bool,
}

/// Contract parameter
#[derive(Debug, Clone)]
pub struct ContractParameter {
    /// Name
    pub name: String,
    /// Type
    pub parameter_type: String,
}

/// Contract event
#[derive(Debug, Clone)]
pub struct ContractEvent {
    /// Name
    pub name: String,
    /// Parameters
    pub parameters: Vec<ContractParameter>,
}

/// Contract permission
#[derive(Debug, Clone)]
pub struct ContractPermission {
    /// Contract
    pub contract: H160,
    /// Methods
    pub methods: Vec<String>,
}

/// Neo candidate
#[derive(Debug, Clone)]
pub struct NeoCandidate {
    /// Public key
    pub public_key: ByteString,
    /// Votes
    pub votes: i64,
}
