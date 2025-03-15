// Neo N3 Contract Manifest Module
//
// This module provides functions for registering contract manifest information
// such as methods, events, and other metadata required by the Neo N3 blockchain.

use crate::Runtime;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};
use serde::{Deserialize, Serialize};

// Flag to track if the contract has been registered
static CONTRACT_REGISTERED: AtomicBool = AtomicBool::new(false);

/// Contract descriptor for inventory
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContractDescriptor {
    name: String,
    version: String,
    author: String,
    email: String,
    description: String,
    dynamic_invoke: bool,
}

impl ContractDescriptor {
    pub fn new(
        name: String,
        version: String,
        author: String,
        email: String,
        description: String,
        dynamic_invoke: bool,
    ) -> Self {
        Self {
            name,
            version,
            author,
            email,
            description,
            dynamic_invoke,
        }
    }
}

inventory::collect!(ContractDescriptor);

/// Method descriptor for inventory
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MethodDescriptor {
    name: String,
    safe: bool,
    param_names: Vec<String>,
    param_types: Vec<String>,
    return_type: String,
}

impl MethodDescriptor {
    pub fn new(
        name: String,
        safe: bool,
        param_names: Vec<String>,
        param_types: Vec<String>,
        return_type: String,
    ) -> Self {
        Self {
            name,
            safe,
            param_names,
            param_types,
            return_type,
        }
    }
}

inventory::collect!(MethodDescriptor);

/// Event descriptor for inventory
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventDescriptor {
    name: String,
    param_names: Vec<String>,
    param_types: Vec<String>,
    indexed_params: Vec<bool>,
}

impl EventDescriptor {
    pub fn new(
        name: String,
        param_names: Vec<String>,
        param_types: Vec<String>,
        indexed_params: Vec<bool>,
    ) -> Self {
        Self {
            name,
            param_names,
            param_types,
            indexed_params,
        }
    }
}

inventory::collect!(EventDescriptor);

/// Standard descriptor for inventory
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StandardDescriptor {
    standard: String,
}

impl StandardDescriptor {
    pub fn new(standard: String) -> Self {
        Self { standard }
    }
}

inventory::collect!(StandardDescriptor);

/// OpCode descriptor for inventory
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OpcodeDescriptor {
    name: String,
    opcode: u8,
}

impl OpcodeDescriptor {
    pub fn new(name: String, opcode: u8) -> Self {
        Self { name, opcode }
    }
}

inventory::collect!(OpcodeDescriptor);

/// Register a contract in the Neo N3 manifest
///
/// This function should be called by the #[contract] macro to register
/// the contract's metadata in the manifest.
pub fn register_contract(
    name: &str,
    version: &str,
    author: &str,
    description: &str,
    email: &str,
    dynamic_invoke: bool,
) {
    // We only need to do this once
    if CONTRACT_REGISTERED.swap(true, Ordering::SeqCst) {
        return;
    }

    // Log registration - useful for debugging during development
    Runtime::log(&format!("Registering Neo N3 contract: {}", name).into());
    
    // Store the contract information in inventory for manifest generation
    inventory::submit! {
        ContractDescriptor::new(
            name.to_string(), 
            version.to_string(), 
            author.to_string(), 
            email.to_string(), 
            description.to_string(), 
            dynamic_invoke
        )
    }
}

/// Register a method in the Neo N3 manifest
///
/// This function should be called by the #[method] and #[safe] macros to register
/// the method's metadata in the manifest.
///
/// # Arguments
/// * `name` - The name of the method
/// * `safe` - Whether the method is safe (read-only)
/// * `param_names` - The names of the parameters
/// * `param_types` - The types of the parameters (Neo VM types)
/// * `return_type` - The return type (Neo VM type)
///
/// # Neo N3 Safe Methods
/// In Neo N3, methods can be marked as "safe", which means they are read-only
/// and don't modify the blockchain state. These methods can be called without
/// a transaction fee and are optimized for quick reads.
pub fn register_method(
    name: &str, 
    safe: bool, 
    param_names: &[&str], 
    param_types: &[&str], 
    return_type: &str
) {
    // Validate parameters
    if param_names.len() != param_types.len() {
        Runtime::log(&format!("Error registering method {}: Parameter names and types must have the same length", name).into());
        return;
    }
    
    // Neo N3 has a parameter limit (not strictly enforced but good practice)
    if param_names.len() > 16 {
        Runtime::log(&format!("Warning: Method {} has {} parameters, which may be too many for optimal performance", name, param_names.len()).into());
    }

    // Convert string slices to owned Strings
    let param_names = param_names.iter().map(|&s| s.to_string()).collect();
    let param_types = param_types.iter().map(|&s| s.to_string()).collect();
    
    // Create and register the method descriptor
    let method = MethodDescriptor::new(
        name.to_string(),
        safe,
        param_names,
        param_types,
        return_type.to_string(),
    );
    
    inventory::submit(method);
    
    // Log registration - useful for debugging during development
    Runtime::log(&format!("Registered Neo N3 method: {}{}", 
                         name, 
                         if safe { " (safe)" } else { "" }).into());
}

/// Simplified method to register a safe method (backward compatibility)
pub fn register_safe_method(name: &str) {
    register_method(name, true, &[], &[], "");
}

/// Register an event in the Neo N3 manifest
///
/// This function should be called by the #[event] macro to register
/// the event's metadata in the manifest.
///
/// # Arguments
/// * `name` - The name of the event
/// * `param_names` - The names of the parameters
/// * `param_types` - The types of the parameters (Neo VM types)
/// * `indexed_params` - Whether each parameter is indexed (for event filtering)
///
/// # Neo N3 Event Registration
/// In Neo N3, events must be registered in the contract manifest, and the
/// parameters can be marked as indexed for better filtering capabilities.
/// Events are emitted at runtime using Runtime::notify method.
pub fn register_event(
    name: &str,
    param_names: &[&str],
    param_types: &[&str],
    indexed_params: &[bool],
) {
    // Validate parameters
    if param_names.len() != param_types.len() || param_names.len() != indexed_params.len() {
        Runtime::log(&format!("Error registering event {}: Parameter names, types, and indexed flags must have the same length", name).into());
        return;
    }
    
    // In Neo N3, there's a limit of 16 indexed parameters
    let indexed_count = indexed_params.iter().filter(|&&indexed| indexed).count();
    if indexed_count > 16 {
        Runtime::log(&format!("Warning: Event {} has {} indexed parameters, but Neo N3 only supports up to 16", name, indexed_count).into());
    }
    
    // Convert string slices to owned Strings
    let param_names = param_names.iter().map(|&s| s.to_string()).collect();
    let param_types = param_types.iter().map(|&s| s.to_string()).collect();
    let indexed_params = indexed_params.to_vec();
    
    // Create and register the event descriptor
    let event = EventDescriptor::new(
        name.to_string(),
        param_names,
        param_types,
        indexed_params,
    );
    
    inventory::submit(event);
    
    // Log registration - useful for debugging during development
    Runtime::log(&format!("Registered Neo N3 event: {}", name).into());
}

/// Simplified event registration for backward compatibility
pub fn register_simple_event(name: &str, param_names: &[&str], indexed_params: &[bool]) {
    register_event(name, param_names, &[], indexed_params);
}

/// Register a supported standard in the Neo N3 manifest
///
/// This function should be called by the #[supported_standards] macro to register
/// the standards that the contract implements.
///
/// # Arguments
/// * `standard` - The name of the standard (e.g., "NEP-17", "NEP-11")
///
/// # Neo N3 Standards
/// Neo N3 has several token standards that contracts can implement:
/// - NEP-17: Fungible Token Standard (similar to ERC-20)
/// - NEP-11: Non-Fungible Token Standard (similar to ERC-721)
/// - And others...
///
/// Declaring supported standards helps wallets and applications interact with the contract.
pub fn register_supported_standard(standard: &str) {
    // Log registration - useful for debugging during development
    Runtime::log(&format!("Registered Neo N3 supported standard: {}", standard).into());
    
    // Create and register the standard descriptor
    let std_descriptor = StandardDescriptor::new(standard.to_string());
    
    inventory::submit(std_descriptor);
    
    // If this is a known standard, check to make sure the required methods are implemented
    match standard {
        "NEP-17" => {
            // Check if the contract has registered the required NEP-17 methods
            let methods = get_registered_methods();
            let required_methods = ["symbol", "decimals", "totalSupply", "balanceOf", "transfer"];
            
            for &method in &required_methods {
                if !methods.iter().any(|m| m.name == method) {
                    Runtime::log(&format!("Warning: Contract registered as NEP-17 but method '{}' is not implemented", method).into());
                }
            }
        },
        "NEP-11" => {
            // Check if the contract has registered the required NEP-11 methods
            let methods = get_registered_methods();
            let required_methods = ["ownerOf", "transfer", "balanceOf", "tokens", "properties"];
            
            for &method in &required_methods {
                if !methods.iter().any(|m| m.name == method) {
                    Runtime::log(&format!("Warning: Contract registered as NEP-11 but method '{}' is not implemented", method).into());
                }
            }
        },
        _ => {
            // Unknown standard - just register it
        }
    }
}

/// Retrieve all registered methods for manifest generation
pub fn get_registered_methods() -> Vec<MethodDescriptor> {
    let mut methods = Vec::new();
    
    // Collect all registered methods from inventory
    for method in inventory::iter::<MethodDescriptor>.into_iter() {
        methods.push(method.clone());
    }
    
    methods
}

/// Retrieve all registered events for manifest generation
pub fn get_registered_events() -> Vec<EventDescriptor> {
    let mut events = Vec::new();
    
    // Collect all registered events from inventory
    for event in inventory::iter::<EventDescriptor>.into_iter() {
        events.push(event.clone());
    }
    
    events
}

/// Retrieve all registered standards for manifest generation
pub fn get_registered_standards() -> Vec<StandardDescriptor> {
    let mut standards = Vec::new();
    
    // Collect all registered standards from inventory
    for standard in inventory::iter::<StandardDescriptor>.into_iter() {
        standards.push(standard.clone());
    }
    
    standards
}

/// Retrieve the contract descriptor for manifest generation
pub fn get_contract_descriptor() -> Option<ContractDescriptor> {
    for contract in inventory::iter::<ContractDescriptor>.into_iter() {
        return Some(contract.clone());
    }
    
    None
}

/// Retrieve all registered opcodes for manifest generation
pub fn get_registered_opcodes() -> Vec<OpcodeDescriptor> {
    let mut opcodes = Vec::new();
    
    // Collect all registered opcodes from inventory
    for opcode in inventory::iter::<OpcodeDescriptor>.into_iter() {
        opcodes.push(opcode.clone());
    }
    
    opcodes
}

/// A trait for manifest-related data structures
pub trait ManifestDescriptor {
    fn to_json(&self) -> String;
}

/// Trait for converting Rust types to Neo VM types
pub trait NeoType {
    /// Converts a Rust type string to a Neo VM type string
    fn to_neo_type() -> String;
}

// Implementation of NeoType should now match with the signature
// in the compiler.rs where it's using neo_contract::types::rust_type_to_neo_type
// instead of neo_contract::manifest::NeoType::to_neo_type
