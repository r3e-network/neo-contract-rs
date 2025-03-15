//! Neo Contract Manifest format.
//!
//! This module provides a representation of the Neo Contract Manifest,
//! which is a JSON document that describes a Neo smart contract, including
//! its methods, events, permissions, and other metadata.

use crate::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value as JsonValue};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents a Neo N3 Contract Parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractParameterDefinition {
    /// The name of the parameter
    pub name: String,
    
    /// The Neo VM type of the parameter
    #[serde(rename = "type")]
    pub param_type: String,
    
    /// Whether the parameter is indexed (for events)
    #[serde(skip_serializing_if = "is_default")]
    pub indexed: bool,
}

/// Helper function to skip serialization of default values
fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

impl ContractParameterDefinition {
    /// Creates a new parameter definition
    pub fn new(name: String, param_type: String) -> Self {
        Self {
            name,
            param_type,
            indexed: false,
        }
    }
}

/// Represents a Neo N3 Contract Method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractMethodDefinition {
    /// The name of the method
    pub name: String,
    
    /// The parameters of the method
    pub parameters: Vec<ContractParameterDefinition>,
    
    /// The Neo VM return type of the method
    #[serde(rename = "returntype")]
    pub return_type: String,
    
    /// Whether the method is safe (read-only)
    #[serde(skip_serializing_if = "is_default")]
    pub safe: bool,
    
    /// The offset of the method in the script
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

impl ContractMethodDefinition {
    /// Creates a new method definition
    pub fn new(
        name: String,
        parameters: Vec<ContractParameterDefinition>,
        return_type: String,
        safe: bool,
    ) -> Self {
        Self {
            name,
            parameters,
            return_type,
            safe,
            offset: None,
        }
    }

    /// Sets the offset of the method in the script
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// Represents a Neo N3 Contract Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEventDefinition {
    /// The name of the event
    pub name: String,
    
    /// The parameters of the event
    pub parameters: Vec<ContractParameterDefinition>,
}

impl ContractEventDefinition {
    /// Creates a new event definition
    pub fn new(name: String, parameters: Vec<ContractParameterDefinition>) -> Self {
        Self { name, parameters }
    }
}

/// Represents a Neo N3 Contract ABI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAbi {
    /// The methods in the contract
    pub methods: Vec<ContractMethodDefinition>,
    
    /// The events in the contract
    pub events: Vec<ContractEventDefinition>,
}

impl ContractAbi {
    /// Creates a new empty ABI
    pub fn new() -> Self {
        Self {
            methods: Vec::new(),
            events: Vec::new(),
        }
    }

    /// Adds a method to the ABI
    pub fn add_method(&mut self, method: ContractMethodDefinition) {
        self.methods.push(method);
    }

    /// Adds an event to the ABI
    pub fn add_event(&mut self, event: ContractEventDefinition) {
        self.events.push(event);
    }
}

/// Represents a Neo N3 Contract Group Permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionDescriptor {
    /// The contract hash of the permission, or "*" for wildcard
    pub contract: String,
    
    /// The methods allowed, "Default" for default permissions
    pub methods: Vec<String>,
}

impl PermissionDescriptor {
    /// Creates a new permission descriptor
    pub fn new(contract: &str, methods: &[&str]) -> Self {
        Self {
            contract: contract.to_string(),
            methods: methods.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// Represents a Neo N3 Contract Manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// The name of the contract
    pub name: String,
    
    /// The ABI of the contract
    pub abi: ContractAbi,
    
    /// Features of the contract (storage, payable, etc.)
    pub features: HashMap<String, bool>,
    
    /// Groups of the contract
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    
    /// Supported standards (NEP-17, etc.)
    #[serde(rename = "supportedstandards")]
    pub supported_standards: Vec<String>,
    
    /// Permission descriptors
    pub permissions: Vec<PermissionDescriptor>,
    
    /// Trusted contracts
    #[serde(rename = "trusts")]
    pub trusts: Vec<String>,
    
    /// Extra fields not covered by the standard fields
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, JsonValue>,
}

impl Manifest {
    /// Creates a new manifest with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            abi: ContractAbi::new(),
            features: {
                let mut features = HashMap::new();
                features.insert("storage".to_string(), false);
                features.insert("payable".to_string(), false);
                features
            },
            groups: Vec::new(),
            supported_standards: Vec::new(),
            permissions: Vec::new(),
            trusts: Vec::new(),
            extra: HashMap::new(),
        }
    }

    /// Load a manifest from a JSON file template
    pub fn from_template(
        template_path: &Path,
        contract_name: &str,
        abi: ContractAbi,
    ) -> Result<Self, Error> {
        let template_content = fs::read_to_string(template_path)
            .map_err(|e| Error::IO(format!("Failed to read manifest template: {}", e)))?;

        let mut manifest: Manifest = serde_json::from_str(&template_content)
            .map_err(|e| Error::Parse(format!("Failed to parse manifest template: {}", e)))?;

        manifest.name = contract_name.to_string();
        manifest.abi = abi;

        Ok(manifest)
    }

    /// Sets the ABI of the contract
    pub fn with_abi(mut self, abi: ContractAbi) -> Self {
        self.abi = abi;
        self
    }

    /// Sets the author of the contract
    pub fn set_author(&mut self, author: &str) -> Result<(), Error> {
        self.extra.insert(
            "author".to_string(),
            JsonValue::String(author.to_string()),
        );
        Ok(())
    }

    /// Sets the email of the contract author
    pub fn set_email(&mut self, email: &str) -> Result<(), Error> {
        self.extra
            .insert("email".to_string(), JsonValue::String(email.to_string()));
        Ok(())
    }

    /// Sets the description of the contract
    pub fn set_description(&mut self, description: &str) -> Result<(), Error> {
        self.extra.insert(
            "description".to_string(),
            JsonValue::String(description.to_string()),
        );
        Ok(())
    }

    /// Sets the version of the contract
    pub fn set_version(&mut self, version: &str) -> Result<(), Error> {
        self.extra
            .insert("version".to_string(), JsonValue::String(version.to_string()));
        Ok(())
    }

    /// Sets a feature of the contract
    pub fn set_feature(&mut self, feature: &str, enabled: bool) -> Result<(), Error> {
        self.features.insert(feature.to_string(), enabled);
        Ok(())
    }

    /// Adds a supported standard to the contract
    pub fn add_supported_standard(&mut self, standard: &str) -> Result<(), Error> {
        self.supported_standards.push(standard.to_string());
        Ok(())
    }

    /// Adds a permission to the contract
    pub fn add_permission(&mut self, contract: &str, method: &str) -> Result<(), Error> {
        // Check if we already have a permission for this contract
        for permission in &mut self.permissions {
            if permission.contract == contract {
                permission.methods.push(method.to_string());
                return Ok(());
            }
        }

        // If not, create a new permission
        self.permissions.push(PermissionDescriptor::new(
            contract,
            &[method],
        ));
        Ok(())
    }

    /// Adds a trusted contract to the manifest
    pub fn add_trust(&mut self, contract: &str) -> Result<(), Error> {
        self.trusts.push(contract.to_string());
        Ok(())
    }

    /// Sets whether the contract is payable
    pub fn set_payable(&mut self, payable: bool) -> Result<(), Error> {
        self.set_feature("payable", payable)
    }

    /// Validates the manifest
    pub fn validate(&self) -> Result<(), Error> {
        // Validate that required fields are set
        if self.name.is_empty() {
            return Err(Error::InvalidManifest("Contract name is required".to_string()));
        }

        // Check ABI methods
        for method in &self.abi.methods {
            if method.name.is_empty() {
                return Err(Error::InvalidManifest("Method name is required".to_string()));
            }
            
            // Check parameter types are valid Neo N3 types
            for param in &method.parameters {
                if !is_valid_neo_type(&param.param_type) {
                    return Err(Error::InvalidManifest(
                        format!("Invalid parameter type '{}' for method '{}'", param.param_type, method.name)
                    ));
                }
            }
            
            // Check return type is a valid Neo N3 type
            if !is_valid_neo_type(&method.return_type) {
                return Err(Error::InvalidManifest(
                    format!("Invalid return type '{}' for method '{}'", method.return_type, method.name)
                ));
            }
        }
        
        // Check ABI events
        for event in &self.abi.events {
            if event.name.is_empty() {
                return Err(Error::InvalidManifest("Event name is required".to_string()));
            }
            
            // Check parameter types are valid Neo N3 types
            for param in &event.parameters {
                if !is_valid_neo_type(&param.param_type) {
                    return Err(Error::InvalidManifest(
                        format!("Invalid parameter type '{}' for event '{}'", param.param_type, event.name)
                    ));
                }
            }
            
            // Neo N3 limits the number of indexed parameters
            let indexed_count = event.parameters.iter().filter(|p| p.indexed).count();
            if indexed_count > 16 {
                return Err(Error::InvalidManifest(
                    format!("Event '{}' has {} indexed parameters, but Neo N3 only supports up to 16", 
                            event.name, indexed_count)
                ));
            }
        }

        // At least one permission must be defined
        if self.permissions.is_empty() {
            return Err(Error::InvalidManifest("At least one permission must be defined".to_string()));
        }

        Ok(())
    }

    /// Converts the manifest to JSON
    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string_pretty(self)
            .map_err(|e| Error::Serialization(format!("Failed to serialize manifest: {}", e)))
    }
}

/// Checks if a type is a valid Neo N3 VM type
fn is_valid_neo_type(neo_type: &str) -> bool {
    match neo_type {
        "Signature" | "Boolean" | "Integer" | "Hash160" | "Hash256" | 
        "ByteArray" | "PublicKey" | "String" | "Array" | "Map" | 
        "InteropInterface" | "Void" | "Any" => true,
        _ => false,
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
            ContractParameterDefinition::new("owner".to_string(), "Hash160".to_string()),
            ContractParameterDefinition::new("amount".to_string(), "Integer".to_string()),
        ];

        // Add a method to the ABI
        let method = ContractMethodDefinition::new(
            "transfer".to_string(),
            params,
            "Boolean".to_string(),
            false,
        );
        manifest.abi.add_method(method);

        // Add an event to the ABI
        let event_params = vec![
            ContractParameterDefinition::new("from".to_string(), "Hash160".to_string()),
            ContractParameterDefinition::new("to".to_string(), "Hash160".to_string()),
            ContractParameterDefinition::new("amount".to_string(), "Integer".to_string()),
        ];
        let event = ContractEventDefinition::new("Transfer".to_string(), event_params);
        manifest.abi.add_event(event);

        // Set features
        manifest.set_feature("storage", true).unwrap();
        manifest.set_feature("payable", true).unwrap();

        // Add a supported standard
        manifest.add_supported_standard("NEP-17").unwrap();

        // Add an extra field
        manifest.set_description("My first Neo contract").unwrap();

        // Serialize to JSON
        let json = manifest.to_json().unwrap();

        // Deserialize from JSON
        let deserialized = serde_json::from_str(&json).unwrap();

        // Check if the deserialized manifest is equal to the original
        assert_eq!(manifest, deserialized);
        assert_eq!(deserialized.name, "MyContract");
        assert_eq!(deserialized.abi.methods.len(), 1);
        assert_eq!(deserialized.abi.events.len(), 1);
        assert_eq!(deserialized.features["storage"], true);
        assert_eq!(deserialized.features["payable"], true);
        assert_eq!(deserialized.supported_standards, vec!["NEP-17"]);
    }

    #[test]
    fn test_manifest_validation() {
        // Valid manifest
        let mut manifest = Manifest::new("MyContract");
        let method = ContractMethodDefinition::new(
            "transfer".to_string(),
            vec![],
            "Void".to_string(),
            true,
        );
        manifest.abi.add_method(method);
        assert!(manifest.validate().is_ok());

        // Invalid: empty name
        let mut invalid = manifest.clone();
        invalid.name = "".to_string();
        assert!(invalid.validate().is_err());

        // Invalid: duplicate method names
        let mut invalid = manifest.clone();
        let duplicate_method = ContractMethodDefinition::new(
            "transfer".to_string(), // Same name as before
            vec![],
            "Void".to_string(),
            true,
        );
        invalid.abi.add_method(duplicate_method);
        assert!(invalid.validate().is_err());
    }
}
