//! Neo Contract Manifest format.
//!
//! This module provides a representation of the Neo Contract Manifest,
//! which is a JSON document that describes a Neo smart contract, including
//! its methods, events, permissions, and other metadata.

use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Contract parameter definition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContractParameterDefinition {
    /// The name of the parameter.
    pub name: String,
    /// The type of the parameter.
    #[serde(rename = "type")]
    pub param_type: String,
}

impl ContractParameterDefinition {
    /// Creates a new contract parameter definition.
    pub fn new(name: impl Into<String>, param_type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            param_type: param_type.into(),
        }
    }
}

/// Contract method definition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContractMethodDefinition {
    /// The name of the method.
    pub name: String,
    /// The parameters of the method.
    pub parameters: Vec<ContractParameterDefinition>,
    /// The return type of the method.
    #[serde(rename = "returntype")]
    pub return_type: String,
    /// Indicates if the method is safe (does not modify state).
    pub safe: bool,
    /// The offset of the method in the script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

impl ContractMethodDefinition {
    /// Creates a new contract method definition.
    pub fn new(
        name: impl Into<String>,
        parameters: Vec<ContractParameterDefinition>,
        return_type: impl Into<String>,
        safe: bool,
    ) -> Self {
        Self {
            name: name.into(),
            parameters,
            return_type: return_type.into(),
            safe,
            offset: None,
        }
    }

    /// Sets the offset of the method.
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// Contract event definition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContractEventDefinition {
    /// The name of the event.
    pub name: String,
    /// The parameters of the event.
    pub parameters: Vec<ContractParameterDefinition>,
}

impl ContractEventDefinition {
    /// Creates a new contract event definition.
    pub fn new(name: impl Into<String>, parameters: Vec<ContractParameterDefinition>) -> Self {
        Self {
            name: name.into(),
            parameters,
        }
    }
}

/// Contract ABI (Application Binary Interface).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ContractAbi {
    /// The methods of the contract.
    pub methods: Vec<ContractMethodDefinition>,
    /// The events of the contract.
    pub events: Vec<ContractEventDefinition>,
}

impl ContractAbi {
    /// Creates a new contract ABI.
    pub fn new(
        methods: Vec<ContractMethodDefinition>,
        events: Vec<ContractEventDefinition>,
    ) -> Self {
        Self { methods, events }
    }

    /// Creates an empty contract ABI.
    pub fn empty() -> Self {
        Self {
            methods: Vec::new(),
            events: Vec::new(),
        }
    }
}

/// Contract features.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ContractFeatures {
    /// Whether the contract uses storage.
    #[serde(default)]
    pub storage: bool,
    /// Whether the contract uses the payable feature.
    #[serde(default)]
    pub payable: bool,
}

/// Contract permission.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContractPermission {
    /// The contract hash or wildcard.
    pub contract: String,
    /// The methods allowed, or wildcard.
    pub methods: Vec<String>,
}

impl ContractPermission {
    /// Creates a new contract permission.
    pub fn new(contract: impl Into<String>, methods: Vec<String>) -> Self {
        Self {
            contract: contract.into(),
            methods,
        }
    }

    /// Creates a wildcard permission (allows access to all contracts and methods).
    pub fn wildcard() -> Self {
        Self {
            contract: "*".to_string(),
            methods: vec!["*".to_string()],
        }
    }
}

/// Contract group.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContractGroup {
    /// The public key of the group.
    pub pubkey: String,
    /// The signature of the contract hash.
    pub signature: String,
}

impl ContractGroup {
    /// Creates a new contract group.
    pub fn new(pubkey: impl Into<String>, signature: impl Into<String>) -> Self {
        Self {
            pubkey: pubkey.into(),
            signature: signature.into(),
        }
    }
}

/// Neo contract manifest.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Manifest {
    /// The name of the contract.
    pub name: String,
    /// The groups the contract belongs to.
    pub groups: Vec<ContractGroup>,
    /// The contracts this contract trusts.
    pub trusts: Vec<String>,
    /// The permissions requested by the contract.
    pub permissions: Vec<ContractPermission>,
    /// The ABI of the contract.
    pub abi: ContractAbi,
    /// The features used by the contract.
    #[serde(default)]
    pub features: ContractFeatures,
    /// Additional custom metadata.
    #[serde(default)]
    pub extra: HashMap<String, serde_json::Value>,
    /// The supported standards (e.g., NEP-17, NEP-11).
    #[serde(default)]
    pub supportedstandards: Vec<String>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            name: String::new(),
            groups: Vec::new(),
            trusts: Vec::new(),
            permissions: vec![ContractPermission::wildcard()],
            abi: ContractAbi::default(),
            features: ContractFeatures::default(),
            extra: HashMap::new(),
            supportedstandards: Vec::new(),
        }
    }
}

impl Manifest {
    /// Creates a new manifest with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// Sets the ABI of the manifest.
    pub fn with_abi(mut self, abi: ContractAbi) -> Self {
        self.abi = abi;
        self
    }

    /// Adds a supported standard to the manifest.
    pub fn add_supported_standard(&mut self, standard: impl Into<String>) {
        self.supportedstandards.push(standard.into());
    }

    /// Adds a permission to the manifest.
    pub fn add_permission(&mut self, permission: ContractPermission) {
        self.permissions.push(permission);
    }

    /// Adds a trusted contract to the manifest.
    pub fn add_trusted_contract(&mut self, contract_hash: impl Into<String>) {
        self.trusts.push(contract_hash.into());
    }

    /// Adds a group to the manifest.
    pub fn add_group(&mut self, group: ContractGroup) {
        self.groups.push(group);
    }

    /// Adds a method to the ABI.
    pub fn add_method(&mut self, method: ContractMethodDefinition) {
        self.abi.methods.push(method);
    }

    /// Adds an event to the ABI.
    pub fn add_event(&mut self, event: ContractEventDefinition) {
        self.abi.events.push(event);
    }

    /// Sets a feature value.
    pub fn set_feature(&mut self, name: &str, value: bool) -> Result<(), Error> {
        match name {
            "storage" => self.features.storage = value,
            "payable" => self.features.payable = value,
            _ => return Err(Error::invalid_manifest(format!("Unknown feature: {}", name))),
        }
        Ok(())
    }

    /// Sets an extra field in the manifest.
    pub fn set_extra<T: Serialize>(&mut self, key: &str, value: T) -> Result<(), Error> {
        self.extra.insert(
            key.to_string(),
            serde_json::to_value(value).map_err(|e| Error::Serialization(e.to_string()))?,
        );
        Ok(())
    }

    /// Serializes the manifest to JSON.
    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string_pretty(self).map_err(|e| Error::Serialization(e.to_string()))
    }

    /// Deserializes a manifest from JSON.
    pub fn from_json(json: &str) -> Result<Self, Error> {
        serde_json::from_str(json).map_err(|e| Error::invalid_manifest(e.to_string()))
    }

    /// Saves the manifest to a file.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let json = self.to_json()?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Loads a manifest from a file.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let json = fs::read_to_string(path)?;
        Self::from_json(&json)
    }

    /// Validates the manifest.
    pub fn validate(&self) -> Result<(), Error> {
        // Check required fields
        if self.name.is_empty() {
            return Err(Error::invalid_manifest("Contract name cannot be empty"));
        }

        // Check method names are unique
        let mut method_names = std::collections::HashSet::new();
        for method in &self.abi.methods {
            if method.name.is_empty() {
                return Err(Error::invalid_manifest("Method name cannot be empty"));
            }
            if !method_names.insert(&method.name) {
                return Err(Error::invalid_manifest(format!(
                    "Duplicate method name: {}",
                    method.name
                )));
            }
        }

        // Check event names are unique
        let mut event_names = std::collections::HashSet::new();
        for event in &self.abi.events {
            if event.name.is_empty() {
                return Err(Error::invalid_manifest("Event name cannot be empty"));
            }
            if !event_names.insert(&event.name) {
                return Err(Error::invalid_manifest(format!(
                    "Duplicate event name: {}",
                    event.name
                )));
            }
        }

        // Successful validation
        Ok(())
    }

    /// Creates a manifest from a template file, filling in the specified fields.
    pub fn from_template<P: AsRef<Path>>(
        template_path: P,
        name: &str,
        abi: ContractAbi,
    ) -> Result<Self, Error> {
        let template_json = fs::read_to_string(template_path)?;
        let mut manifest: Self = Self::from_json(&template_json)?;
        
        manifest.name = name.to_string();
        manifest.abi = abi;
        
        Ok(manifest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_serialization() {
        let mut manifest = Manifest::new("MyContract");
        
        // Add parameters for a method
        let params = vec![
            ContractParameterDefinition::new("owner", "Hash160"),
            ContractParameterDefinition::new("amount", "Integer"),
        ];
        
        // Add a method to the ABI
        let method = ContractMethodDefinition::new(
            "transfer",
            params,
            "Boolean",
            false,
        );
        manifest.add_method(method);
        
        // Add an event to the ABI
        let event_params = vec![
            ContractParameterDefinition::new("from", "Hash160"),
            ContractParameterDefinition::new("to", "Hash160"),
            ContractParameterDefinition::new("amount", "Integer"),
        ];
        let event = ContractEventDefinition::new("Transfer", event_params);
        manifest.add_event(event);
        
        // Set features
        manifest.set_feature("storage", true).unwrap();
        manifest.set_feature("payable", true).unwrap();
        
        // Add a supported standard
        manifest.add_supported_standard("NEP-17");
        
        // Add an extra field
        manifest.set_extra("description", "My first Neo contract").unwrap();
        
        // Serialize to JSON
        let json = manifest.to_json().unwrap();
        
        // Deserialize from JSON
        let deserialized = Manifest::from_json(&json).unwrap();
        
        // Check if the deserialized manifest is equal to the original
        assert_eq!(manifest, deserialized);
        assert_eq!(deserialized.name, "MyContract");
        assert_eq!(deserialized.abi.methods.len(), 1);
        assert_eq!(deserialized.abi.events.len(), 1);
        assert_eq!(deserialized.features.storage, true);
        assert_eq!(deserialized.features.payable, true);
        assert_eq!(deserialized.supportedstandards, vec!["NEP-17"]);
    }

    #[test]
    fn test_manifest_validation() {
        // Valid manifest
        let mut manifest = Manifest::new("MyContract");
        let method = ContractMethodDefinition::new(
            "transfer",
            vec![ContractParameterDefinition::new("amount", "Integer")],
            "Boolean",
            false,
        );
        manifest.add_method(method);
        assert!(manifest.validate().is_ok());
        
        // Invalid: empty name
        let mut invalid = manifest.clone();
        invalid.name = "".to_string();
        assert!(invalid.validate().is_err());
        
        // Invalid: duplicate method names
        let mut invalid = manifest.clone();
        let duplicate_method = ContractMethodDefinition::new(
            "transfer", // Same name as before
            vec![],
            "Void",
            true,
        );
        invalid.add_method(duplicate_method);
        assert!(invalid.validate().is_err());
    }
}