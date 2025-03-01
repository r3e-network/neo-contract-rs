// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use crate::types::builtin::h160::H160;
use crate::types::builtin::int256::Int256;
use crate::types::builtin::string::ByteString;
use crate::types::key::PublicKey;

/// Contract represents a Neo contract
#[derive(Debug, Clone)]
pub struct Contract {
    script_hash: H160,
    manifest: ContractManifest,
}

/// ContractManifest represents a Neo contract manifest
#[derive(Debug, Clone)]
pub struct ContractManifest {
    name: ByteString,
    groups: Vec<ContractGroup>,
    features: ContractFeatures,
    supported_standards: Vec<ByteString>,
    abi: ContractABI,
    permissions: Vec<ContractPermission>,
    trusts: Vec<ContractTrust>,
    extra: ContractExtra,
}

/// ContractGroup represents a Neo contract group
#[derive(Debug, Clone)]
pub struct ContractGroup {
    pub_key: PublicKey,
    signature: Vec<u8>,
}

/// ContractFeatures represents Neo contract features
#[derive(Debug, Clone)]
pub struct ContractFeatures {
    storage: bool,
    payable: bool,
}

/// ContractABI represents a Neo contract ABI
#[derive(Debug, Clone)]
pub struct ContractABI {
    methods: Vec<ContractMethod>,
    events: Vec<ContractEvent>,
}

/// ContractMethod represents a Neo contract method
#[derive(Debug, Clone)]
pub struct ContractMethod {
    name: ByteString,
    parameters: Vec<ContractParameter>,
    return_type: ContractParameterType,
    offset: u32,
    safe: bool,
}

/// ContractEvent represents a Neo contract event
#[derive(Debug, Clone)]
pub struct ContractEvent {
    name: ByteString,
    parameters: Vec<ContractParameter>,
}

/// ContractParameter represents a Neo contract parameter
#[derive(Debug, Clone)]
pub struct ContractParameter {
    name: ByteString,
    param_type: ContractParameterType,
}

/// ContractParameterType represents a Neo contract parameter type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractParameterType {
    Any,
    Boolean,
    Integer,
    ByteArray,
    String,
    Hash160,
    Hash256,
    PublicKey,
    Signature,
    Array,
    Map,
    InteropInterface,
    Void,
}

/// ContractPermission represents a Neo contract permission
#[derive(Debug, Clone)]
pub struct ContractPermission {
    contract: H160,
    methods: Vec<ByteString>,
}

/// ContractTrust represents a Neo contract trust
#[derive(Debug, Clone)]
pub struct ContractTrust {
    contract: H160,
}

/// ContractExtra represents Neo contract extra information
#[derive(Debug, Clone)]
pub struct ContractExtra {
    data: Vec<(ByteString, ByteString)>,
}

/// NeoCandidate represents a Neo candidate
#[derive(Debug, Clone)]
pub struct NeoCandidate {
    pub_key: PublicKey,
    votes: Int256,
}

impl Contract {
    /// Create a new contract
    pub fn new(script_hash: H160, manifest: ContractManifest) -> Self {
        Self {
            script_hash,
            manifest,
        }
    }

    /// Get the script hash
    pub fn script_hash(&self) -> H160 {
        self.script_hash.clone()
    }

    /// Get the manifest
    pub fn manifest(&self) -> &ContractManifest {
        &self.manifest
    }
}

impl ContractManifest {
    /// Create a new contract manifest
    pub fn new(
        name: ByteString,
        groups: Vec<ContractGroup>,
        features: ContractFeatures,
        supported_standards: Vec<ByteString>,
        abi: ContractABI,
        permissions: Vec<ContractPermission>,
        trusts: Vec<ContractTrust>,
        extra: ContractExtra,
    ) -> Self {
        Self {
            name,
            groups,
            features,
            supported_standards,
            abi,
            permissions,
            trusts,
            extra,
        }
    }

    /// Get the name
    pub fn name(&self) -> ByteString {
        self.name.clone()
    }

    /// Get the groups
    pub fn groups(&self) -> &[ContractGroup] {
        &self.groups
    }

    /// Get the features
    pub fn features(&self) -> &ContractFeatures {
        &self.features
    }

    /// Get the supported standards
    pub fn supported_standards(&self) -> &[ByteString] {
        &self.supported_standards
    }

    /// Get the ABI
    pub fn abi(&self) -> &ContractABI {
        &self.abi
    }

    /// Get the permissions
    pub fn permissions(&self) -> &[ContractPermission] {
        &self.permissions
    }

    /// Get the trusts
    pub fn trusts(&self) -> &[ContractTrust] {
        &self.trusts
    }

    /// Get the extra
    pub fn extra(&self) -> &ContractExtra {
        &self.extra
    }
}

impl NeoCandidate {
    /// Create a new Neo candidate
    pub fn new(pub_key: PublicKey, votes: Int256) -> Self {
        Self { pub_key, votes }
    }

    /// Get the public key
    pub fn pub_key(&self) -> &PublicKey {
        &self.pub_key
    }

    /// Get the votes
    pub fn votes(&self) -> Int256 {
        self.votes.clone()
    }
}
