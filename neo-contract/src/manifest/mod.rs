// Neo N3 Contract Manifest Module
//
// This module provides functions for registering contract manifest information
// such as methods, events, and other metadata required by the Neo N3 blockchain.

use crate::Runtime;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cell::RefCell;
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
        Self { name, version, author, email, description, dynamic_invoke }
    }
}

// Define static collections with alternatives that work in no_std
type RegistryCell<T> = RefCell<T>;
static mut CONTRACT_DESCRIPTOR: Option<RegistryCell<Option<ContractDescriptor>>> = None;
static mut METHOD_DESCRIPTORS: Option<RegistryCell<Vec<MethodDescriptor>>> = None;
static mut EVENT_DESCRIPTORS: Option<RegistryCell<Vec<EventDescriptor>>> = None;
static mut STANDARD_DESCRIPTORS: Option<RegistryCell<Vec<StandardDescriptor>>> = None;
static mut OPCODE_DESCRIPTORS: Option<RegistryCell<Vec<OpcodeDescriptor>>> = None;

// Helper function to initialize registries
fn init_registries() {
    unsafe {
        if CONTRACT_DESCRIPTOR.is_none() {
            CONTRACT_DESCRIPTOR = Some(RefCell::new(None));
        }
        if METHOD_DESCRIPTORS.is_none() {
            METHOD_DESCRIPTORS = Some(RefCell::new(Vec::new()));
        }
        if EVENT_DESCRIPTORS.is_none() {
            EVENT_DESCRIPTORS = Some(RefCell::new(Vec::new()));
        }
        if STANDARD_DESCRIPTORS.is_none() {
            STANDARD_DESCRIPTORS = Some(RefCell::new(Vec::new()));
        }
        if OPCODE_DESCRIPTORS.is_none() {
            OPCODE_DESCRIPTORS = Some(RefCell::new(Vec::new()));
        }
    }
}

// Helper functions to access registries
fn with_contract_descriptor<F, R>(f: F) -> R
where F: FnOnce(&RefCell<Option<ContractDescriptor>>) -> R {
    init_registries();
    unsafe { f(CONTRACT_DESCRIPTOR.as_ref().unwrap()) }
}

fn with_method_descriptors<F, R>(f: F) -> R
where F: FnOnce(&RefCell<Vec<MethodDescriptor>>) -> R {
    init_registries();
    unsafe { f(METHOD_DESCRIPTORS.as_ref().unwrap()) }
}

fn with_event_descriptors<F, R>(f: F) -> R
where F: FnOnce(&RefCell<Vec<EventDescriptor>>) -> R {
    init_registries();
    unsafe { f(EVENT_DESCRIPTORS.as_ref().unwrap()) }
}

fn with_standard_descriptors<F, R>(f: F) -> R
where F: FnOnce(&RefCell<Vec<StandardDescriptor>>) -> R {
    init_registries();
    unsafe { f(STANDARD_DESCRIPTORS.as_ref().unwrap()) }
}

fn with_opcode_descriptors<F, R>(f: F) -> R
where F: FnOnce(&RefCell<Vec<OpcodeDescriptor>>) -> R {
    init_registries();
    unsafe { f(OPCODE_DESCRIPTORS.as_ref().unwrap()) }
}

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
        Self { name, safe, param_names, param_types, return_type }
    }
}

/// Event descriptor for inventory
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventDescriptor {
    name: String,
    param_names: Vec<String>,
    param_types: Vec<String>,
    indexed_params: Vec<bool>,
}

impl EventDescriptor {
    pub fn new(name: String, param_names: Vec<String>, param_types: Vec<String>, indexed_params: Vec<bool>) -> Self {
        Self { name, param_names, param_types, indexed_params }
    }
}

/// Standard descriptor for inventory
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StandardDescriptor {
    standard: String,
}

impl StandardDescriptor {
    pub fn new(standard: String) -> Self { Self { standard } }
}

/// OpCode descriptor for inventory
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OpcodeDescriptor {
    name: String,
    opcode: u8,
}

impl OpcodeDescriptor {
    pub fn new(name: String, opcode: u8) -> Self { Self { name, opcode } }
}

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
    Runtime::log_str(&format!("Registering Neo N3 contract: {}", name));

    let descriptor = ContractDescriptor::new(
        name.to_owned(),
        version.to_owned(),
        author.to_owned(),
        email.to_owned(),
        description.to_owned(),
        dynamic_invoke,
    );

    with_contract_descriptor(|registry| {
        *registry.borrow_mut() = Some(descriptor);
    });
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
pub fn register_method(name: &str, safe: bool, param_names: &[&str], param_types: &[&str], return_type: &str) {
    // Validate parameters
    if param_names.len() != param_types.len() {
        Runtime::log_str(&format!(
            "Error registering method {}: Parameter names and types must have the same length",
            name
        ));
        return;
    }

    // Neo N3 has a parameter limit (not strictly enforced but good practice)
    if param_names.len() > 16 {
        Runtime::log_str(&format!(
            "Warning: Method {} has {} parameters, which may be too many for optimal performance",
            name,
            param_names.len()
        ));
    }

    // Convert string slices to owned Strings
    let param_names = param_names.iter().map(|&s| s.to_string()).collect();
    let param_types = param_types.iter().map(|&s| s.to_string()).collect();

    // Create and register the method descriptor
    let method = MethodDescriptor::new(name.to_string(), safe, param_names, param_types, return_type.to_string());

    with_method_descriptors(|registry| {
        registry.borrow_mut().push(method);
    });

    // Log registration - useful for debugging during development
    Runtime::log_str(&format!("Registered Neo N3 method: {}{}", name, if safe { " (safe)" } else { "" }));
}

/// Simplified method to register a safe method (backward compatibility)
pub fn register_safe_method(name: &str) { register_method(name, true, &[], &[], ""); }

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
pub fn register_event(name: &str, param_names: &[&str], param_types: &[&str], indexed_params: &[bool]) {
    // Validate parameters
    if param_names.len() != param_types.len() || param_names.len() != indexed_params.len() {
        Runtime::log_str(&format!(
            "Error registering event {}: Parameter names, types, and indexed flags must have the same length",
            name
        ));
        return;
    }

    // In Neo N3, there's a limit of 16 indexed parameters
    let indexed_count = indexed_params.iter().filter(|&&indexed| indexed).count();
    if indexed_count > 16 {
        Runtime::log_str(&format!(
            "Warning: Event {} has {} indexed parameters, but Neo N3 only supports up to 16",
            name, indexed_count
        ));
    }

    // Convert string slices to owned Strings
    let param_names = param_names.iter().map(|&s| s.to_string()).collect();
    let param_types = param_types.iter().map(|&s| s.to_string()).collect();
    let indexed_params = indexed_params.to_vec();

    // Create and register the event descriptor
    let event = EventDescriptor::new(name.to_string(), param_names, param_types, indexed_params);

    with_event_descriptors(|registry| {
        registry.borrow_mut().push(event);
    });

    // Log registration
    Runtime::log_str(&format!("Registered Neo N3 event: {}", name));
}

/// Simplified event registration for backward compatibility
pub fn register_simple_event(name: &str, param_names: &[&str], indexed_params: &[bool]) {
    register_event(name, param_names, &[], indexed_params);
}

/// Check if a method is registered for this contract
fn has_method(method_name: &str) -> bool {
    let methods = get_registered_methods();
    methods.iter().any(|m| m.name == method_name)
}

/// Register a supported standard
///
/// Declaring supported standards helps wallets and applications interact with the contract.
pub fn register_supported_standard(standard: &str) {
    // Log registration
    Runtime::log_str(&format!("Registered Neo N3 supported standard: {}", standard));

    let std_descriptor = StandardDescriptor::new(standard.to_string());

    // Add the standard to the global list
    with_standard_descriptors(|registry| {
        let mut standards = registry.borrow_mut();
        standards.push(std_descriptor);
    });

    // Check for required NEP-17 methods
    if standard == "NEP-17" {
        let required_methods = ["balanceOf", "totalSupply", "transfer"];
        for method in required_methods.iter() {
            if !has_method(method) {
                Runtime::log_str(&format!(
                    "Warning: Contract registered as NEP-17 but method '{}' is not implemented",
                    method
                ));
            }
        }
    }

    // Check for required NEP-11 methods
    if standard == "NEP-11" {
        let required_methods = ["ownerOf", "balanceOf", "totalSupply", "transfer"];
        for method in required_methods.iter() {
            if !has_method(method) {
                Runtime::log_str(&format!(
                    "Warning: Contract registered as NEP-11 but method '{}' is not implemented",
                    method
                ));
            }
        }
    }
}

/// Retrieve all registered methods for manifest generation
pub fn get_registered_methods() -> Vec<MethodDescriptor> {
    with_method_descriptors(|registry| registry.borrow().clone())
}

/// Retrieve all registered events for manifest generation
pub fn get_registered_events() -> Vec<EventDescriptor> { with_event_descriptors(|registry| registry.borrow().clone()) }

/// Retrieve all registered standards for manifest generation
pub fn get_registered_standards() -> Vec<String> {
    with_standard_descriptors(|registry| registry.borrow().iter().map(|std| std.standard.clone()).collect())
}

/// Retrieve the contract descriptor for manifest generation
pub fn get_contract_descriptor() -> Option<ContractDescriptor> {
    with_contract_descriptor(|registry| registry.borrow().clone())
}

/// Retrieve all registered opcodes for manifest generation
pub fn get_registered_opcodes() -> Vec<OpcodeDescriptor> {
    with_opcode_descriptors(|registry| registry.borrow().clone())
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
