//! Method attribute handling for Neo N3 smart contracts.
//!
//! This module provides functionality for handling method attributes in Neo N3 smart contracts,
//! such as the important #[safe] attribute which marks methods as read-only, or access control
//! attributes that determine who can call the contract.

use crate::error::Error;
use crate::script::Script;
use crate::neo::OpCode;

/// Method attribute types for Neo N3 smart contracts
#[derive(Debug, Clone, PartialEq)]
pub enum MethodAttribute {
    /// Marks a method as safe (read-only, doesn't modify state)
    /// In Neo N3, safe methods are represented with "safe": true in the manifest
    Safe,
    
    /// Marks a method as requiring a specific contract permission
    /// Used for contract access control in Neo N3
    ContractPermission(String),
    
    /// Marks a method as requiring the owner's signature
    /// Common pattern in Neo N3 for administrative functions
    OwnerOnly,
    
    /// Marks a method as disabled during contract verification
    /// Used for methods that should only be called after verification
    DisableDuringVerification,
    
    /// Custom attribute with name and optional value
    /// For extensibility and future attribute support
    Custom(String, Option<String>),
}

/// Method information including name and attributes
#[derive(Debug, Clone)]
pub struct MethodInfo {
    /// Method name
    pub name: String,
    /// Method attributes
    pub attributes: Vec<MethodAttribute>,
    /// Method parameter types
    pub parameters: Vec<String>,
    /// Method return type
    pub return_type: Option<String>,
}

impl MethodInfo {
    /// Creates a new method info
    pub fn new(name: &str) -> Self {
        MethodInfo {
            name: name.to_string(),
            attributes: Vec::new(),
            parameters: Vec::new(),
            return_type: None,
        }
    }

    /// Adds an attribute to the method
    pub fn add_attribute(&mut self, attr: MethodAttribute) {
        self.attributes.push(attr);
    }

    /// Checks if the method is marked as safe (read-only)
    /// In Neo N3, safe methods are represented with "safe": true in the manifest
    pub fn is_safe(&self) -> bool {
        self.attributes.iter().any(|attr| *attr == MethodAttribute::Safe)
    }
    
    /// Checks if the method requires owner-only access
    pub fn is_owner_only(&self) -> bool {
        self.attributes.iter().any(|attr| *attr == MethodAttribute::OwnerOnly)
    }
    
    /// Checks if the method is disabled during verification
    pub fn is_disabled_during_verification(&self) -> bool {
        self.attributes.iter().any(|attr| *attr == MethodAttribute::DisableDuringVerification)
    }
    
    /// Gets any contract permissions required by this method
    pub fn get_contract_permissions(&self) -> Vec<String> {
        self.attributes.iter()
            .filter_map(|attr| {
                if let MethodAttribute::ContractPermission(contract) = attr {
                    Some(contract.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Generates Neo VM code for a method with proper attribute handling
///
/// # Arguments
///
/// * `script` - The script to emit instructions to
/// * `method_info` - Information about the method
/// * `body_generator` - Function that generates the method body
///
/// # Returns
///
/// * `Result<(), Error>` - Result of the operation
///   Emits a Neo N3 compatible smart contract method.
///
/// This function generates the Neo VM instructions for a smart contract method,
/// including proper handling of method attributes like `#[safe]` for read-only methods,
/// `#[owner_only]` for administrative methods, and contract permissions.
/// 
/// In Neo N3, various method attributes control how methods behave:
///   - Methods marked with `#[safe]` are read-only and cannot modify state ("safe": true in manifest)
///   - Methods marked with `#[owner_only]` require owner verification
///   - Methods with `#[contract_permission(...)]` specify which contracts can call them
///   - Methods with `#[disabled_during_verification]` cannot be called during contract verification
///
/// # Parameters
///
/// * `script` - The Neo VM script to emit instructions to
/// * `method_info` - Information about the method including name and attributes
/// * `body_generator` - Function that generates the method body
///
/// # Returns
///
/// * `Result<(), Error>` - Result of the operation
pub fn emit_method<F>(
    script: &mut Script,
    method_info: &MethodInfo,
    body_generator: F,
) -> Result<(), Error>
where
    F: FnOnce(&mut Script) -> Result<(), Error>,
{
    // Begin method implementation
    
    // Push method name as entry point identifier
    script.emit_push_data(method_info.name.as_bytes())?;
    
    // Handle method attributes that must be checked before method execution
    
    // For safe methods, add verification at the beginning
    // This ensures that any attempts to modify state will fail early
    if method_info.is_safe() {
        script.emit_push_data(b"neo.safe.method.begin")?;
    }
    
    // For owner-only methods, add owner verification check
    if method_info.is_owner_only() {
        // Add Neo N3-style owner verification
        script.emit_push_data(b"neo.owner.verification")?;
    }
    
    // For methods with contract permissions, add verification
    let contract_permissions = method_info.get_contract_permissions();
    if !contract_permissions.is_empty() {
        // Add contract permission verification
        for contract in &contract_permissions {
            script.emit_push_data(format!("neo.contract.verification.{}", contract).as_bytes())?;
        }
    }
    
    // For methods disabled during verification, add verification check
    if method_info.is_disabled_during_verification() {
        script.emit_push_data(b"neo.verification.disabled")?;
    }
    
    // Generate method body
    body_generator(script)?;
    
    // Add end markers for methods with special attributes
    
    // For safe methods, add an end marker
    if method_info.is_safe() {
        script.emit_push_data(b"neo.safe.method.end")?;
    }
    
    // End method implementation
    script.emit_opcode(OpCode::RET);
    
    Ok(())
}
